use std::{error::Error, fmt, str};

use http::header::{HeaderMap, CONTENT_TYPE};

use crate::Event;

pub struct RequestParser {
    verification_token: String,
}

impl RequestParser {
    pub fn new(verification_token: &str) -> Self {
        Self {
            verification_token: verification_token.to_string(),
        }
    }

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

    pub fn parse(&self, headers: HeaderMap, body: &[u8]) -> Result<Event, ParseError> {
        let event = self.parse_headers(headers)?;
        let body = str::from_utf8(body).map_err(|_| ParseError::ReadBodyFailed)?;
        match event.as_str() {
            "PING" => serde_json::from_str(body)
                .map(Event::Ping)
                .map_err(|_| ParseError::ParseBodyFailed),
            "JOINED" => serde_json::from_str(body)
                .map(Event::Joined)
                .map_err(|_| ParseError::ParseBodyFailed),
            "LEFT" => serde_json::from_str(body)
                .map(Event::Left)
                .map_err(|_| ParseError::ParseBodyFailed),
            "MESSAGE_CREATED" => serde_json::from_str(body)
                .map(Event::MessageCreated)
                .map_err(|_| ParseError::ParseBodyFailed),
            "MESSAGE_DELETED" => serde_json::from_str(body)
                .map(Event::MessageDeleted)
                .map_err(|_| ParseError::ParseBodyFailed),
            "MESSAGE_UPDATED" => serde_json::from_str(body)
                .map(Event::MessageUpdated)
                .map_err(|_| ParseError::ParseBodyFailed),
            "DIRECT_MESSAGE_CREATED" => serde_json::from_str(body)
                .map(Event::DirectMessageCreated)
                .map_err(|_| ParseError::ParseBodyFailed),
            "DIRECT_MESSAGE_DELETED" => serde_json::from_str(body)
                .map(Event::DirectMessageDeleted)
                .map_err(|_| ParseError::ParseBodyFailed),
            "DIRECT_MESSAGE_UPDATED" => serde_json::from_str(body)
                .map(Event::DirectMessageUpdated)
                .map_err(|_| ParseError::ParseBodyFailed),
            "BOT_MESSAGE_STAMPS_UPDATED" => serde_json::from_str(body)
                .map(Event::BotMessageStampsUpdated)
                .map_err(|_| ParseError::ParseBodyFailed),
            "CHANNEL_CREATED" => serde_json::from_str(body)
                .map(Event::ChannelCreated)
                .map_err(|_| ParseError::ParseBodyFailed),
            "CHANNEL_TOPIC_CHANGED" => serde_json::from_str(body)
                .map(Event::ChannelTopicChanged)
                .map_err(|_| ParseError::ParseBodyFailed),
            "USER_CREATED" => serde_json::from_str(body)
                .map(Event::UserCreated)
                .map_err(|_| ParseError::ParseBodyFailed),
            "STAMP_CREATED" => serde_json::from_str(body)
                .map(Event::StampCreated)
                .map_err(|_| ParseError::ParseBodyFailed),
            "TAG_ADDED" => serde_json::from_str(body)
                .map(Event::TagAdded)
                .map_err(|_| ParseError::ParseBodyFailed),
            "TAG_REMOVED" => serde_json::from_str(body)
                .map(Event::TagRemoved)
                .map_err(|_| ParseError::ParseBodyFailed),
            _ => Err(ParseError::BotEventMismatch),
        }
    }
}

#[derive(Debug, Clone)]
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
            BotEventMismatch => "X-TRAQ-BOT-EVENT is wrong",
            ReadBodyFailed => "Failed to read request body",
            ParseBodyFailed => "Failed to parse request body",
        };
        write!(f, "{message}")
    }
}

impl Error for ParseError {}
