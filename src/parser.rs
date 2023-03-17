//! `struct RequestParser`と`enum ParseError`

use std::{error::Error, fmt, str};

use http::header::{HeaderMap, CONTENT_TYPE};
use serde::Deserialize;

use crate::Event;

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
    /// let event = parser.parse_headers(headers);
    /// ```
    fn parse_headers(&self, headers: HeaderMap) -> Result<String, ParseError> {
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
        let event = headers
            .get("X-TRAQ-BOT-EVENT")
            .ok_or(ParseError::BotEventNotFound)?
            .to_str()
            .map_err(|_| ParseError::ReadBotEventFailed)?;
        Ok(event.to_string())
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
        let event = self.parse_headers(headers)?;
        let body = str::from_utf8(body).map_err(|_| ParseError::ReadBodyFailed)?;
        match event.as_str() {
            "PING" => parse_body(Event::Ping, body),
            "JOINED" => parse_body(Event::Joined, body),
            "LEFT" => parse_body(Event::Left, body),
            "MESSAGE_CREATED" => parse_body(Event::MessageCreated, body),
            "MESSAGE_DELETED" => parse_body(Event::MessageDeleted, body),
            "MESSAGE_UPDATED" => parse_body(Event::MessageUpdated, body),
            "DIRECT_MESSAGE_CREATED" => parse_body(Event::DirectMessageCreated, body),
            "DIRECT_MESSAGE_DELETED" => parse_body(Event::DirectMessageDeleted, body),
            "DIRECT_MESSAGE_UPDATED" => parse_body(Event::DirectMessageUpdated, body),
            "BOT_MESSAGE_STAMPS_UPDATED" => parse_body(Event::BotMessageStampsUpdated, body),
            "CHANNEL_CREATED" => parse_body(Event::ChannelCreated, body),
            "CHANNEL_TOPIC_CHANGED" => parse_body(Event::ChannelTopicChanged, body),
            "USER_CREATED" => parse_body(Event::UserCreated, body),
            "STAMP_CREATED" => parse_body(Event::StampCreated, body),
            "TAG_ADDED" => parse_body(Event::TagAdded, body),
            "TAG_REMOVED" => parse_body(Event::TagRemoved, body),
            _ => Err(ParseError::BotEventMismatch),
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
