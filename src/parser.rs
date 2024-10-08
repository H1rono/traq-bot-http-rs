//! `struct RequestParser`と`enum ParseError`

use std::{error::Error, fmt, str::from_utf8};

use serde::Deserialize;

use crate::macros::all_events;
use crate::{Event, EventKind, RequestParser};

/// ボディをDeserializeして`Event`に渡す
fn parse_body<'a, T, F>(f: F, body: &'a str) -> Result<Event, ParseError>
where
    T: Deserialize<'a>,
    F: Fn(T) -> Event,
{
    serde_json::from_str(body)
        .map(f)
        .map_err(|_| ParseError::ParseBodyFailed)
}

// https://datatracker.ietf.org/doc/html/rfc9110#section-5.5
fn valid_header_value(value: &str) -> bool {
    value
        .as_bytes()
        .iter()
        .all(|c| (0x20..=0x7E).contains(c) || *c == 0x09)
}

impl RequestParser {
    /// 新しい`RequestParser`を作成します。
    ///
    /// ## Arguments
    /// * `verification_token` - ボットのVerification Token
    ///
    /// ## Example
    /// ```
    /// use traq_bot_http::RequestParser;
    /// let parser = RequestParser::new("verification_token");
    /// ```
    pub fn new(verification_token: &str) -> Self {
        Self {
            verification_token: verification_token.to_string(),
        }
    }

    /// POSTリクエストのヘッダーからイベント名を取得します。
    ///
    /// ## Arguments
    /// * `headers` - リクエストのヘッダー
    ///
    /// ## Example
    /// ```
    /// use http::HeaderMap;
    /// use traq_bot_http::{RequestParser, ParseError};
    /// let parser = RequestParser::new("verification_token");
    /// let headers = HeaderMap::new();
    /// let kind = parser.parse_headers(&headers);
    /// assert!(matches!(kind, Err(ParseError::ContentTypeNotFound)));
    /// ```
    ///
    /// ## Errors
    /// [`ParseError`]のうち、以下のものを返す可能性があります。
    ///
    /// - [`ParseError::ReadContentTypeFailed`] :
    ///     ヘッダー`Content-Type`の値をUTF8の文字列として解釈できなかった
    /// - [`ParseError::ContentTypeNotFound`] :
    ///     ヘッダー`Content-Type`が見つからなかった
    /// - [`ParseError::ContentTypeMismatch`] :
    ///     ヘッダー`Content-Type`の値が`application/json`で始まらない
    /// - [`ParseError::ReadBotTokenFailed`] : ヘッダー`X-TRAQ-BOT-TOKEN`の値に関して、以下のいずれかの場合
    ///     - 値をUTF8の文字列として解釈できなかった
    ///     - 値が`visible US-ASCII octets (VCHAR)`, `SP`, `HTAB`以外の文字を含む ([RFC9110 5.5])
    /// - [`ParseError::BotTokenNotFound`] :
    ///     ヘッダー`X-TRAQ-BOT-TOKEN`が見つからなかった
    /// - [`ParseError::BotTokenMismatch`] :
    ///     ヘッダー`X-TRAQ-BOT-TOKEN`の値が[`new`]で与えられたVerification Tokenと合わない
    /// - [`ParseError::ReadBotEventFailed`] : ヘッダー`X-TRAQ-BOT-EVENT`の値に関して、以下のいずれかの場合
    ///     - 値をUTF8の文字列として解釈できなかった
    ///     - 値が`visible US-ASCII octets (VCHAR)`, `SP`, `HTAB`以外の文字を含む ([RFC9110 5.5])
    /// - [`ParseError::BotEventNotFound`] :
    ///     ヘッダー`X-TRAQ-BOT-EVENT`が見つからなかった
    /// - [`ParseError::BotEventMismatch`] :
    ///     ヘッダー`X-TRAQ-BOT-EVENT`の値が[`EventKind`]の[`std::str::FromStr`]でパースできなかった
    ///
    /// [RFC9110 5.5]: https://datatracker.ietf.org/doc/html/rfc9110#section-5.5
    /// [`new`]: RequestParser::new
    pub fn parse_headers<'a, H, K, V>(&self, headers: H) -> Result<EventKind, ParseError>
    where
        H: IntoIterator<Item = (&'a K, &'a V)>,
        K: AsRef<[u8]> + ?Sized + 'static,
        V: AsRef<[u8]> + ?Sized + 'static,
    {
        // Content-Type: application/json
        let mut content_type = None;
        // X-TRAQ-BOT-TOKEN: ${self.verification_token}
        let mut token = None;
        // X-TRAQ-BOT-EVENTがヘッダーに含まれており、かつその値はイベント名のいずれかである
        let mut kind = None;
        for (k, v) in headers {
            let Ok(k) = from_utf8(k.as_ref()) else {
                continue
            };
            let v = from_utf8(v.as_ref());
            match k.to_lowercase().as_str() {
                "content-type" => {
                    let v = v.map_err(|_| ParseError::ReadContentTypeFailed)?;
                    content_type = Some(v);
                }
                "x-traq-bot-token" => {
                    let v = v.map_err(|_| ParseError::ReadBotTokenFailed)?;
                    token = Some(v);
                }
                "x-traq-bot-event" => {
                    let v = v.map_err(|_| ParseError::ReadBotEventFailed)?;
                    kind = Some(v);
                }
                _ => continue,
            }
        }
        content_type
            .ok_or(ParseError::ContentTypeNotFound)
            .map(|ct| ct.starts_with("application/json"))?
            .then_some(())
            .ok_or(ParseError::ContentTypeMismatch)?;
        token
            .ok_or(ParseError::BotTokenNotFound)
            .and_then(|t| {
                valid_header_value(t)
                    .then_some(t)
                    .ok_or(ParseError::ReadBotTokenFailed)
            })
            .map(|t| t == self.verification_token)?
            .then_some(())
            .ok_or(ParseError::BotTokenMismatch)?;
        kind.ok_or(ParseError::BotEventNotFound)
            .and_then(|k| {
                valid_header_value(k)
                    .then_some(k)
                    .ok_or(ParseError::ReadBotEventFailed)
            })?
            .parse()
            .map_err(|_| ParseError::BotEventMismatch)
    }

    /// HTTP POSTリクエストをパースします。
    ///
    /// ## Arguments
    /// * `headers` - リクエストのヘッダー
    /// * `body` - リクエストのボディ
    ///
    /// ## Example
    /// ```
    /// use traq_bot_http::{RequestParser, Event};
    /// let headers = [
    ///     ("Content-Type", "application/json"),
    ///     ("X-TRAQ-BOT-TOKEN", "verification_token"),
    ///     ("X-TRAQ-BOT-EVENT", "PING"),
    /// ];
    /// let body = br#"{"eventTime": "2019-05-07T04:50:48.582586882Z"}"#;
    /// let verification_token = "verification_token";
    /// let parser = RequestParser::new(verification_token);
    /// let event = parser.parse(headers, body);
    /// assert!(matches!(event, Ok(Event::Ping(_))));
    /// ```
    ///
    /// ## Errors
    /// [`ParseError`]のうち、以下のものを返す可能性があります。
    ///
    /// - [`parse_headers`]で返されるもの
    /// - [`ParseError::ReadBodyFailed`] :
    ///     `body`をUTF8の文字列として解釈できなかった
    /// - [`ParseError::ParseBodyFailed`] :
    ///     `body`を[`parse_headers`]で返される[`EventKind`]に対応する
    ///     [`Event`]のペイロードJSONとしてデシリアライズできなかった。
    ///
    /// [`parse_headers`]: RequestParser::parse_headers
    pub fn parse<'a, H, K, V>(&self, headers: H, body: &[u8]) -> Result<Event, ParseError>
    where
        H: IntoIterator<Item = (&'a K, &'a V)>,
        K: AsRef<[u8]> + ?Sized + 'static,
        V: AsRef<[u8]> + ?Sized + 'static,
    {
        let kind = self.parse_headers(headers)?;
        let body = from_utf8(body).map_err(|_| ParseError::ReadBodyFailed)?;

        macro_rules! match_kind_parse_body {
            ($( $k:ident ),*) => {
                match kind {
                    $(
                        EventKind::$k => parse_body(Event::$k, body),
                    )*
                }
            };
        }

        all_events!(match_kind_parse_body)
    }
}

/// `RequestParser::parse`時のエラー型
///
/// ## Variants
/// * `ContentTypeNotFound` - Content-Typeがヘッダーに含まれていない
/// * `ReadContentTypeFailed` - Content-Typeの値を読み取れなかった
/// * `ContentTypeMismatch` - Content-Typeの値がapplication/jsonで始まっていない
/// * `BotTokenNotFound` - X-TRAQ-BOT-TOKENがヘッダーに含まれていない
/// * `ReadBotTokenFailed` - X-TRAQ-BOT-TOKENの値を読み取れなかった3
/// * `BotTokenMismatch` - X-TRAQ-BOT-TOKENの値がverification_tokenと等しくない
/// * `BotEventNotFound` - X-TRAQ-BOT-EVENTがヘッダーに含まれていない
/// * `ReadBotEventFailed` - X-TRAQ-BOT-EVENTの値を読み取れなかった
/// * `BotEventMismatch` - X-TRAQ-BOT-EVENTの値がイベント名のいずれでもない
/// * `ReadBodyFailed` - リクエストボディの値を読み取れなかった
/// * `ParseBodyFailed` - リクエストボディの値をパースできなかった
///
/// ## Example
/// ```
/// use traq_bot_http::RequestParser;
/// use traq_bot_http::ParseError;
/// use http::HeaderMap;
///
/// let verification_token = "verification_token";
/// let parser = RequestParser::new(verification_token);
/// let headers = HeaderMap::new();
/// let body = b"";
/// assert_eq!(parser.parse(&headers, body), Err(ParseError::ContentTypeNotFound));
/// ```
#[must_use]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    ContentTypeNotFound,
    ReadContentTypeFailed,
    ContentTypeMismatch,
    BotTokenNotFound,
    ReadBotTokenFailed,
    BotTokenMismatch,
    BotEventNotFound,
    ReadBotEventFailed,
    BotEventMismatch,
    ReadBodyFailed,
    ParseBodyFailed,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            ParseError::ContentTypeNotFound => "Content-Type is not set",
            ParseError::ReadContentTypeFailed => "Failed to read Content-Type value",
            ParseError::ContentTypeMismatch => {
                "Content-Type value is wrong; it must be application/json"
            }
            ParseError::BotTokenNotFound => "X-TRAQ-BOT-TOKEN is not set",
            ParseError::ReadBotTokenFailed => "Failed to read X-TRAQ-BOT-TOKEN value",
            ParseError::BotTokenMismatch => "X-TRAQ-BOT-TOKEN value is wrong",
            ParseError::BotEventNotFound => "X-TRAQ-BOT-EVENT is not set",
            ParseError::ReadBotEventFailed => "Failed to read X-TRAQ-BOT-EVENT value",
            ParseError::BotEventMismatch => "X-TRAQ-BOT-EVENT value is wrong",
            ParseError::ReadBodyFailed => "Failed to read request body",
            ParseError::ParseBodyFailed => "Failed to parse request body",
        };
        write!(f, "{message}")
    }
}

impl Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::test_parse_payload;

    use http::header::HeaderMap;
    use http::header::CONTENT_TYPE;

    #[test]
    fn request_parser_new() {
        let verification_token = "verification_token";
        let parser = RequestParser::new(verification_token);
        println!("{parser:?}");
    }

    #[test]
    fn parse_error_derives() {
        use crate::test_utils::PARSE_ERROR_VARIANTS;
        for variant in PARSE_ERROR_VARIANTS {
            let error = variant.clone();
            assert_eq!(&variant, &error);
            assert_eq!(format!("{variant:?}"), format!("{error:?}"));
        }
    }

    #[test]
    fn parse_failure() {
        use crate::test_utils::make_parser;
        let parser = make_parser();
        let mut headers = HeaderMap::new();
        assert_eq!(
            parser.parse(&headers, b""),
            Err(ParseError::ContentTypeNotFound)
        );
        headers.insert(CONTENT_TYPE, "text/plain".parse().unwrap());
        assert_eq!(
            parser.parse(&headers, b""),
            Err(ParseError::ContentTypeMismatch)
        );
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        assert_eq!(
            parser.parse(&headers, b""),
            Err(ParseError::BotTokenNotFound)
        );
        headers.insert("X-TRAQ-BOT-TOKEN", "invalid　token".parse().unwrap());
        assert_eq!(
            parser.parse(&headers, b""),
            Err(ParseError::ReadBotTokenFailed)
        );
        headers.insert("X-TRAQ-BOT-TOKEN", "invalid_token".parse().unwrap());
        assert_eq!(
            parser.parse(&headers, b""),
            Err(ParseError::BotTokenMismatch)
        );
        headers.insert(
            "X-TRAQ-BOT-TOKEN",
            "traqbotverificationtoken".parse().unwrap(),
        );
        assert_eq!(
            parser.parse(&headers, b""),
            Err(ParseError::BotEventNotFound)
        );
        headers.insert("X-TRAQ-BOT-EVENT", "invalid　event".parse().unwrap());
        assert_eq!(
            parser.parse(&headers, b""),
            Err(ParseError::ReadBotEventFailed)
        );
        headers.insert("X-TRAQ-BOT-EVENT", "invalid_event".parse().unwrap());
        assert_eq!(
            parser.parse(&headers, b""),
            Err(ParseError::BotEventMismatch)
        );
        headers.insert("X-TRAQ-BOT-EVENT", "PING".parse().unwrap());
        assert_eq!(
            parser.parse(&headers, &[0, 159, 146, 150]),
            Err(ParseError::ReadBodyFailed)
        );
        assert_eq!(
            parser.parse(&headers, b""),
            Err(ParseError::ParseBodyFailed)
        );
    }

    #[test]
    fn err_display() {
        let pairs = [
            (ParseError::ContentTypeNotFound, "Content-Type is not set"),
            (
                ParseError::ReadContentTypeFailed,
                "Failed to read Content-Type value",
            ),
            (
                ParseError::ContentTypeMismatch,
                "Content-Type value is wrong; it must be application/json",
            ),
            (ParseError::BotTokenNotFound, "X-TRAQ-BOT-TOKEN is not set"),
            (
                ParseError::ReadBotTokenFailed,
                "Failed to read X-TRAQ-BOT-TOKEN value",
            ),
            (
                ParseError::BotTokenMismatch,
                "X-TRAQ-BOT-TOKEN value is wrong",
            ),
            (ParseError::BotEventNotFound, "X-TRAQ-BOT-EVENT is not set"),
            (
                ParseError::ReadBotEventFailed,
                "Failed to read X-TRAQ-BOT-EVENT value",
            ),
            (
                ParseError::BotEventMismatch,
                "X-TRAQ-BOT-EVENT value is wrong",
            ),
            (ParseError::ReadBodyFailed, "Failed to read request body"),
            (ParseError::ParseBodyFailed, "Failed to parse request body"),
        ];
        for (err, msg) in pairs {
            assert_eq!(err.to_string(), *msg);
        }
    }

    test_parse_payload! {"system", Ping}

    test_parse_payload! {"system", Joined}

    test_parse_payload! {"system", Left}

    test_parse_payload! {"message", MessageCreated}

    test_parse_payload! {"message", MessageDeleted}

    test_parse_payload! {"message", MessageUpdated}

    test_parse_payload! {"message", DirectMessageCreated}

    test_parse_payload! {"message", DirectMessageDeleted}

    test_parse_payload! {"message", DirectMessageUpdated}

    test_parse_payload! {"message", BotMessageStampsUpdated}

    test_parse_payload! {"channel", ChannelCreated}

    test_parse_payload! {"channel", ChannelTopicChanged}

    test_parse_payload! {"user", UserCreated}

    test_parse_payload! {"stamp", StampCreated}

    test_parse_payload! {"tag", TagAdded}

    test_parse_payload! {"tag", TagRemoved}

    test_parse_payload! {"user-group", UserGroupCreated}

    test_parse_payload! {"user-group", UserGroupUpdated}

    test_parse_payload! {"user-group", UserGroupDeleted}

    test_parse_payload! {"user-group", UserGroupMemberAdded}

    test_parse_payload! {"user-group", UserGroupMemberUpdated}

    test_parse_payload! {"user-group", UserGroupMemberRemoved}

    test_parse_payload! {"user-group", UserGroupAdminAdded}

    test_parse_payload! {"user-group", UserGroupAdminRemoved}
}
