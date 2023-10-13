//! `struct RequestParser`と`enum ParseError`

use std::{error::Error, fmt, str};

use http::header::{HeaderMap, CONTENT_TYPE};
use serde::Deserialize;

use crate::{Event, EventKind};

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

/// HTTP POSTリクエストのパーサー
#[derive(Debug, Clone)]
pub struct RequestParser {
    verification_token: String,
}

impl RequestParser {
    /// 新しい`RequestParser`を作成します。
    ///
    /// ## Arguments
    /// * `verification_token` - ボットのVerificationToken
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
    /// ```ignore
    /// use http::HeaderMap;
    /// use traq_bot_http::RequestParser;
    /// let parser = RequestParser::new("verification_token");
    /// let headers = HeaderMap::new();
    /// let kind = parser.parse_headers(headers);
    /// ```
    fn parse_headers(&self, headers: &HeaderMap) -> Result<EventKind, ParseError> {
        // Content-Type: application/json
        let content_type = headers
            .get(CONTENT_TYPE)
            .ok_or(ParseError::ContentTypeNotFound)?
            .to_str()
            .map_err(|_| ParseError::ReadContentTypeFailed)?;
        if !content_type.starts_with("application/json") {
            return Err(ParseError::ContentTypeMismatch);
        }
        // X-TRAQ-BOT-TOKENがヘッダーに含まれており、かつその値はverification_tokenと等しい
        let token = headers
            .get("X-TRAQ-BOT-TOKEN")
            .ok_or(ParseError::BotTokenNotFound)?
            .to_str()
            .map_err(|_| ParseError::ReadBotTokenFailed)?;
        if token != self.verification_token {
            return Err(ParseError::BotTokenMismatch);
        }
        // X-TRAQ-BOT-EVENTがヘッダーに含まれており、かつその値はイベント名のいずれかである
        let kind = headers
            .get("X-TRAQ-BOT-EVENT")
            .ok_or(ParseError::BotEventNotFound)?
            .to_str()
            .map_err(|_| ParseError::ReadBotEventFailed)?
            .parse()?;
        Ok(kind)
    }

    /// HTTP POSTリクエストをパースします。
    ///
    /// ## Arguments
    /// * `headers` - リクエストのヘッダー
    /// * `body` - リクエストのボディ
    ///
    /// ## Example
    /// ```
    /// use http::HeaderMap;
    /// use traq_bot_http::{RequestParser, Event};
    /// let headers: HeaderMap = [
    ///     ("Content-Type", "application/json"),
    ///     ("X-TRAQ-BOT-TOKEN", "verification_token"),
    ///     ("X-TRAQ-BOT-EVENT", "PING"),
    /// ]
    /// .into_iter()
    /// .map(|(k, v)| (k.parse().unwrap(), v.parse().unwrap()))
    /// .collect();
    /// let body = br#"{"eventTime": "2019-05-07T04:50:48.582586882Z"}"#;
    /// let verification_token = "verification_token";
    /// let parser = RequestParser::new(verification_token);
    /// let event = parser.parse(headers, body);
    /// match event {
    ///     Ok(Event::Ping(_)) => (),
    ///     _ => unreachable!(),
    /// }
    /// ```
    pub fn parse(&self, headers: HeaderMap, body: &[u8]) -> Result<Event, ParseError> {
        use EventKind::*;
        let kind = self.parse_headers(&headers)?;
        let body = str::from_utf8(body).map_err(|_| ParseError::ReadBodyFailed)?;
        match kind {
            Ping => parse_body(Event::Ping, body),
            Joined => parse_body(Event::Joined, body),
            Left => parse_body(Event::Left, body),
            MessageCreated => parse_body(Event::MessageCreated, body),
            MessageDeleted => parse_body(Event::MessageDeleted, body),
            MessageUpdated => parse_body(Event::MessageUpdated, body),
            DirectMessageCreated => parse_body(Event::DirectMessageCreated, body),
            DirectMessageDeleted => parse_body(Event::DirectMessageDeleted, body),
            DirectMessageUpdated => parse_body(Event::DirectMessageUpdated, body),
            BotMessageStampsUpdated => parse_body(Event::BotMessageStampsUpdated, body),
            ChannelCreated => parse_body(Event::ChannelCreated, body),
            ChannelTopicChanged => parse_body(Event::ChannelTopicChanged, body),
            UserCreated => parse_body(Event::UserCreated, body),
            StampCreated => parse_body(Event::StampCreated, body),
            TagAdded => parse_body(Event::TagAdded, body),
            TagRemoved => parse_body(Event::TagRemoved, body),
            UserGroupCreated => parse_body(Event::UserGroupCreated, body),
            UserGroupUpdated => parse_body(Event::UserGroupUpdated, body),
            UserGroupDeleted => parse_body(Event::UserGroupDeleted, body),
            UserGroupMemberAdded => parse_body(Event::UserGroupMemberAdded, body),
            UserGroupMemberUpdated => parse_body(Event::UserGroupMemberUpdated, body),
            UserGroupMemberRemoved => parse_body(Event::UserGroupMemberRemoved, body),
            UserGroupAdminAdded => parse_body(Event::UserGroupAdminAdded, body),
            UserGroupAdminRemoved => parse_body(Event::UserGroupAdminRemoved, body),
        }
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
/// assert_eq!(parser.parse(headers, body), Err(ParseError::ContentTypeNotFound));
/// ```
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
        use ParseError::*;
        let message = match self {
            ContentTypeNotFound => "Content-Type is not set",
            ReadContentTypeFailed => "Failed to read Content-Type value",
            ContentTypeMismatch => "Content-Type value is wrong; it must be application/json",
            BotTokenNotFound => "X-TRAQ-BOT-TOKEN is not set",
            ReadBotTokenFailed => "Failed to read X-TRAQ-BOT-TOKEN value",
            BotTokenMismatch => "X-TRAQ-BOT-TOKEN value is wrong",
            BotEventNotFound => "X-TRAQ-BOT-EVENT is not set",
            ReadBotEventFailed => "Failed to read X-TRAQ-BOT-EVENT value",
            BotEventMismatch => "X-TRAQ-BOT-EVENT value is wrong",
            ReadBodyFailed => "Failed to read request body",
            ParseBodyFailed => "Failed to parse request body",
        };
        write!(f, "{message}")
    }
}

impl Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::test_parse_payload;
    use crate::payloads::*;

    #[test]
    fn request_parser_new() {
        let verification_token = "verification_token";
        let parser = RequestParser::new(verification_token);
        println!("{:?}", parser);
    }

    #[test]
    fn parse_error_derives() {
        use crate::test_utils::PARSE_ERROR_VARIANTS;
        for variant in PARSE_ERROR_VARIANTS.iter() {
            let error = variant.clone();
            assert_eq!(variant, &error);
            assert_eq!(format!("{:?}", variant), format!("{:?}", error));
        }
    }

    #[test]
    fn parse_failure() {
        use crate::test_utils::make_parser;
        let parser = make_parser();
        let mut headers = HeaderMap::new();
        assert_eq!(
            parser.parse(headers.clone(), b""),
            Err(ParseError::ContentTypeNotFound)
        );
        headers.insert(CONTENT_TYPE, "text/plain".parse().unwrap());
        assert_eq!(
            parser.parse(headers.clone(), b""),
            Err(ParseError::ContentTypeMismatch)
        );
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        assert_eq!(
            parser.parse(headers.clone(), b""),
            Err(ParseError::BotTokenNotFound)
        );
        headers.insert("X-TRAQ-BOT-TOKEN", "invalid　token".parse().unwrap());
        assert_eq!(
            parser.parse(headers.clone(), b""),
            Err(ParseError::ReadBotTokenFailed)
        );
        headers.insert("X-TRAQ-BOT-TOKEN", "invalid_token".parse().unwrap());
        assert_eq!(
            parser.parse(headers.clone(), b""),
            Err(ParseError::BotTokenMismatch)
        );
        headers.insert(
            "X-TRAQ-BOT-TOKEN",
            "traqbotverificationtoken".parse().unwrap(),
        );
        assert_eq!(
            parser.parse(headers.clone(), b""),
            Err(ParseError::BotEventNotFound)
        );
        headers.insert("X-TRAQ-BOT-EVENT", "invalid　event".parse().unwrap());
        assert_eq!(
            parser.parse(headers.clone(), b""),
            Err(ParseError::ReadBotEventFailed)
        );
        headers.insert("X-TRAQ-BOT-EVENT", "invalid_event".parse().unwrap());
        assert_eq!(
            parser.parse(headers.clone(), b""),
            Err(ParseError::BotEventMismatch)
        );
        headers.insert("X-TRAQ-BOT-EVENT", "PING".parse().unwrap());
        assert_eq!(
            parser.parse(headers.clone(), &[0, 159, 146, 150]),
            Err(ParseError::ReadBodyFailed)
        );
        assert_eq!(
            parser.parse(headers.clone(), b""),
            Err(ParseError::ParseBodyFailed)
        );
    }

    #[test]
    fn err_display() {
        use ParseError::*;
        let pairs = [
            (ContentTypeNotFound, "Content-Type is not set"),
            (ReadContentTypeFailed, "Failed to read Content-Type value"),
            (
                ContentTypeMismatch,
                "Content-Type value is wrong; it must be application/json",
            ),
            (BotTokenNotFound, "X-TRAQ-BOT-TOKEN is not set"),
            (ReadBotTokenFailed, "Failed to read X-TRAQ-BOT-TOKEN value"),
            (BotTokenMismatch, "X-TRAQ-BOT-TOKEN value is wrong"),
            (BotEventNotFound, "X-TRAQ-BOT-EVENT is not set"),
            (ReadBotEventFailed, "Failed to read X-TRAQ-BOT-EVENT value"),
            (BotEventMismatch, "X-TRAQ-BOT-EVENT value is wrong"),
            (ReadBodyFailed, "Failed to read request body"),
            (ParseBodyFailed, "Failed to parse request body"),
        ];
        for (err, msg) in pairs.iter() {
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

    test_parse_payload! {"tag", TagAdded}

    test_parse_payload! {"tag", TagRemoved}
}
