use crate::{payloads::types::*, ParseError, RequestParser};
use ParseError::*;

use http::header::{HeaderMap, CONTENT_TYPE};

pub const VERIFICATION_TOKEN: &str = "traqbotverificationtoken";

pub const PARSE_ERROR_VARIANTS: [ParseError; 11] = [
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
];

pub fn make_parser() -> RequestParser {
    RequestParser::new(VERIFICATION_TOKEN)
}

pub fn make_headers(event: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("X-TRAQ-BOT-TOKEN", VERIFICATION_TOKEN.parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert("X-TRAQ-BOT-EVENT", event.parse().unwrap());
    headers
}

pub fn timestamp(v: &'static str) -> TimeStamp {
    use serde::de::value::{Error, StrDeserializer};
    let de = StrDeserializer::<'_, Error>::new(v);
    crate::payloads::serde::timestamp::deserialize(de).unwrap()
}

pub fn uuid(v: &'static str) -> Uuid {
    use serde::de::value::{Error, StrDeserializer};
    let de = StrDeserializer::<'_, Error>::new(v);
    crate::payloads::serde::uuid::deserialize(de).unwrap()
}

pub fn takashi_trap() -> User {
    User {
        id: uuid("dfdff0c9-5de0-46ee-9721-2525e8bb3d45"),
        name: "takashi_trap".to_string(),
        display_name: "寺田 健二".to_string(),
        icon_id: uuid("2bc06cda-bdb9-4a68-8000-62f907f36a92"),
        bot: false,
    }
}

pub fn channel_a_po() -> Channel {
    Channel {
        id: uuid("f86c925c-3002-4ba5-939a-c92344e534f9"),
        name: "po".to_string(),
        path: "#a/po".to_string(),
        parent_id: uuid("ea452867-553b-4808-a14f-a47ee0009ee6"),
        creator: takashi_trap(),
        created_at: timestamp("2018-04-25T12:22:02Z"),
        updated_at: timestamp("2018-04-25T12:22:02Z"),
    }
}

pub fn embedded_takashi_trap() -> EmbeddedInfo {
    EmbeddedInfo {
        raw: "@takashi_trap".to_string(),
        type_: "user".to_string(),
        id: uuid("dfdff0c9-5de0-46ee-9721-2525e8bb3d45"),
    }
}
