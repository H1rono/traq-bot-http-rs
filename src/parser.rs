//! `struct RequestParser`の定義

use std::str::from_utf8;
use std::sync::Arc;

use crate::error::{Error, ErrorKind, Result};
use crate::macros::all_events;
use crate::{Event, EventKind, RequestParser};

#[cfg(feature = "http")]
mod http;

#[cfg(feature = "http")]
pub use self::http::ParseRequest;

/// ボディをDeserializeして`Event`に渡す
pub(crate) fn parse_body(kind: EventKind, body: &str) -> Result<Event> {
    macro_rules! match_kind_parse_body {
        ($( $k:ident ),*) => {
            match kind {
                $(
                    EventKind::$k => {
                        ::serde_json::from_str(body).map(Event::$k)
                    },
                )*
            }
        };
    }

    all_events!(match_kind_parse_body).map_err(Error::parse_body_failed)
}

// https://datatracker.ietf.org/doc/html/rfc9110#section-5.5
fn valid_header_value(value: &str) -> bool {
    value
        .as_bytes()
        .iter()
        .all(|c| (0x20..=0x7E).contains(c) || *c == 0x09)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Inner {
    verification_token: String,
}

impl Inner {
    pub(crate) fn new(verification_token: &str) -> Self {
        Self {
            verification_token: verification_token.to_string(),
        }
    }
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
            inner: Arc::new(Inner::new(verification_token)),
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
    /// use traq_bot_http::RequestParser;
    ///
    /// let parser = RequestParser::new("verification_token");
    /// let headers = HeaderMap::new();
    /// let kind = parser.parse_headers(&headers);
    /// assert!(kind.is_err());
    /// ```
    ///
    /// ## Errors
    /// [`Error`]のうち、[`Error::kind`]が以下のものを返す可能性があります。
    ///
    /// - [`ErrorKind::ReadContentTypeFailed`] :
    ///     ヘッダー`Content-Type`の値をUTF8の文字列として解釈できなかった
    /// - [`ErrorKind::ContentTypeNotFound`] :
    ///     ヘッダー`Content-Type`が見つからなかった
    /// - [`ErrorKind::ContentTypeMismatch`] :
    ///     ヘッダー`Content-Type`の値が`application/json`で始まらない
    /// - [`ErrorKind::ReadBotTokenFailed`] : ヘッダー`X-TRAQ-BOT-TOKEN`の値に関して、以下のいずれかの場合
    ///     - 値をUTF8の文字列として解釈できなかった
    ///     - 値が`visible US-ASCII octets (VCHAR)`, `SP`, `HTAB`以外の文字を含む ([RFC9110 5.5])
    /// - [`ErrorKind::BotTokenNotFound`] :
    ///     ヘッダー`X-TRAQ-BOT-TOKEN`が見つからなかった
    /// - [`ErrorKind::BotTokenMismatch`] :
    ///     ヘッダー`X-TRAQ-BOT-TOKEN`の値が[`new`]で与えられたVerification Tokenと合わない
    /// - [`ErrorKind::ReadBotEventFailed`] : ヘッダー`X-TRAQ-BOT-EVENT`の値に関して、以下のいずれかの場合
    ///     - 値をUTF8の文字列として解釈できなかった
    ///     - 値が`visible US-ASCII octets (VCHAR)`, `SP`, `HTAB`以外の文字を含む ([RFC9110 5.5])
    /// - [`ErrorKind::BotEventNotFound`] :
    ///     ヘッダー`X-TRAQ-BOT-EVENT`が見つからなかった
    /// - [`ErrorKind::BotEventMismatch`] :
    ///     ヘッダー`X-TRAQ-BOT-EVENT`の値が[`EventKind`]の[`std::str::FromStr`]でパースできなかった
    ///
    /// [`Error::kind`]: crate::Error::kind
    /// [RFC9110 5.5]: https://datatracker.ietf.org/doc/html/rfc9110#section-5.5
    /// [`new`]: RequestParser::new
    pub fn parse_headers<'a, H, K, V>(&self, headers: H) -> Result<EventKind>
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
                continue;
            };
            let v = from_utf8(v.as_ref());
            match k.to_lowercase().as_str() {
                "content-type" => {
                    let v = v.map_err(Error::read_content_type_failed)?;
                    content_type = Some(v);
                }
                "x-traq-bot-token" => {
                    let v = v.map_err(Error::read_bot_token_failed)?;
                    token = Some(v);
                }
                "x-traq-bot-event" => {
                    let v = v.map_err(Error::read_bot_event_failed)?;
                    kind = Some(v);
                }
                _ => continue,
            }
        }
        content_type
            .ok_or(ErrorKind::ContentTypeNotFound)
            .map(|ct| ct.starts_with("application/json"))?
            .then_some(())
            .ok_or(ErrorKind::ContentTypeMismatch)?;
        token
            .ok_or(ErrorKind::BotTokenNotFound)
            .and_then(|t| {
                valid_header_value(t)
                    .then_some(t)
                    .ok_or(ErrorKind::ReadBotTokenFailed)
            })
            .map(|t| t == self.inner.verification_token)?
            .then_some(())
            .ok_or(ErrorKind::BotTokenMismatch)?;
        kind.ok_or(ErrorKind::BotEventNotFound)
            .and_then(|k| {
                valid_header_value(k)
                    .then_some(k)
                    .ok_or(ErrorKind::ReadBotEventFailed)
            })?
            .parse()
            .map_err(Error::bot_event_mismatch)
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
    /// [`Error`]のうち、[`Error::kind`]が以下のものを返す可能性があります。
    ///
    /// - [`parse_headers`]で返されるもの
    /// - [`ErrorKind::ReadBodyFailed`] :
    ///     `body`をUTF8の文字列として解釈できなかった
    /// - [`ErrorKind::ParseBodyFailed`] :
    ///     `body`を[`parse_headers`]で返される[`EventKind`]に対応する
    ///     [`Event`]のペイロードJSONとしてデシリアライズできなかった。
    ///
    /// [`Error::kind`]: crate::Error::kind
    /// [`parse_headers`]: RequestParser::parse_headers
    pub fn parse<'a, H, K, V>(&self, headers: H, body: &[u8]) -> Result<Event>
    where
        H: IntoIterator<Item = (&'a K, &'a V)>,
        K: AsRef<[u8]> + ?Sized + 'static,
        V: AsRef<[u8]> + ?Sized + 'static,
    {
        let kind = self.parse_headers(headers)?;
        let body = from_utf8(body).map_err(Error::read_body_failed)?;
        parse_body(kind, body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::test_parse_payload;

    use ::http::header::HeaderMap;
    use ::http::header::CONTENT_TYPE;

    #[test]
    fn request_parser_new() {
        let verification_token = "verification_token";
        let parser = RequestParser::new(verification_token);
        println!("{parser:?}");
    }

    #[test]
    fn parse_failure() {
        use crate::test_utils::make_parser;
        let parser = make_parser();
        let mut headers = HeaderMap::new();
        assert_eq!(
            parser.parse(&headers, b"").map_err(|e| e.kind()),
            Err(ErrorKind::ContentTypeNotFound)
        );
        headers.insert(CONTENT_TYPE, "text/plain".parse().unwrap());
        assert_eq!(
            parser.parse(&headers, b"").map_err(|e| e.kind()),
            Err(ErrorKind::ContentTypeMismatch)
        );
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        assert_eq!(
            parser.parse(&headers, b"").map_err(|e| e.kind()),
            Err(ErrorKind::BotTokenNotFound)
        );
        headers.insert("X-TRAQ-BOT-TOKEN", "invalid　token".parse().unwrap());
        assert_eq!(
            parser.parse(&headers, b"").map_err(|e| e.kind()),
            Err(ErrorKind::ReadBotTokenFailed)
        );
        headers.insert("X-TRAQ-BOT-TOKEN", "invalid_token".parse().unwrap());
        assert_eq!(
            parser.parse(&headers, b"").map_err(|e| e.kind()),
            Err(ErrorKind::BotTokenMismatch)
        );
        headers.insert(
            "X-TRAQ-BOT-TOKEN",
            "traqbotverificationtoken".parse().unwrap(),
        );
        assert_eq!(
            parser.parse(&headers, b"").map_err(|e| e.kind()),
            Err(ErrorKind::BotEventNotFound)
        );
        headers.insert("X-TRAQ-BOT-EVENT", "invalid　event".parse().unwrap());
        assert_eq!(
            parser.parse(&headers, b"").map_err(|e| e.kind()),
            Err(ErrorKind::ReadBotEventFailed)
        );
        headers.insert("X-TRAQ-BOT-EVENT", "invalid_event".parse().unwrap());
        assert_eq!(
            parser.parse(&headers, b"").map_err(|e| e.kind()),
            Err(ErrorKind::BotEventMismatch)
        );
        headers.insert("X-TRAQ-BOT-EVENT", "PING".parse().unwrap());
        assert_eq!(
            parser
                .parse(&headers, &[0, 159, 146, 150])
                .map_err(|e| e.kind()),
            Err(ErrorKind::ReadBodyFailed)
        );
        assert_eq!(
            parser.parse(&headers, b"").map_err(|e| e.kind()),
            Err(ErrorKind::ParseBodyFailed)
        );
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
