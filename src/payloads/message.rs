//! メッセージ関連のイベントペイロード

use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::types::{DeletedDirectMessage, DeletedMessage, Message, MessageStamp, TimeStamp, Uuid};

/// MESSAGE_CREATEDペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_message_created.go#L10-L14)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/message.md#message_created)
///
/// ## Example
/// ```
/// use traq_bot_http::payloads::MessageCreatedPayload;
/// let payload = r##"{
///     "eventTime": "2019-05-08T13:33:51.690308239Z",
///     "message": {
///         "id": "bc9106b3-f9b2-4eca-9ba1-72b39b40954e",
///         "user": {
///             "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45",
///             "name": "takashi_trap",
///             "displayName": "寺田 健二",
///             "iconId": "2bc06cda-bdb9-4a68-8000-62f907f36a92",
///             "bot": false
///         },
///         "channelId": "9aba50da-f605-4cd0-a428-5e4558cb911e",
///         "text": "!{\"type\": \"user\", \"raw\": \"@takashi_trap\", \"id\": \"dfdff0c9-5de0-46ee-9721-2525e8bb3d45\"} こんにちは",
///         "plainText": "@takashi_trap こんにちは",
///         "embedded": [
///             {
///                 "raw": "@takashi_trap",
///                 "type": "user",
///                 "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45"
///             }
///         ],
///         "createdAt": "2019-05-08T13:33:51.632149265Z",
///         "updatedAt": "2019-05-08T13:33:51.632149265Z"
///     }
/// }"##;
/// let payload = serde_json::from_str::<MessageCreatedPayload>(payload).unwrap();
/// println!("{payload:?}");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct MessageCreatedPayload {
    #[serde(rename = "eventTime", with = "crate::payloads::serde::timestamp")]
    pub event_time: TimeStamp,
    pub message: Message,
}

impl FromStr for MessageCreatedPayload {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl Display for MessageCreatedPayload {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).expect("failed to serialize MessageCreatedPayload")
        )
    }
}

impl From<DirectMessageCreatedPayload> for MessageCreatedPayload {
    fn from(payload: DirectMessageCreatedPayload) -> Self {
        let DirectMessageCreatedPayload {
            event_time,
            message,
        } = payload;
        Self {
            event_time,
            message,
        }
    }
}

/// MESSAGE_DELETEDペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_message_deleted.go#L11-L18)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/message.md#message_deleted)
///
/// ## Example
/// ```
/// use traq_bot_http::payloads::MessageDeletedPayload;
/// let payload = r##"{
///     "eventTime": "2019-05-08T13:33:51.690308239Z",
///     "message": {
///         "id": "bc9106b3-f9b2-4eca-9ba1-72b39b40954e",
///         "channelId": "9aba50da-f605-4cd0-a428-5e4558cb911e"
///     }
/// }"##;
/// let payload = serde_json::from_str::<MessageDeletedPayload>(payload).unwrap();
/// println!("{payload:?}");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct MessageDeletedPayload {
    #[serde(rename = "eventTime", with = "crate::payloads::serde::timestamp")]
    pub event_time: TimeStamp,
    pub message: DeletedMessage,
}

impl FromStr for MessageDeletedPayload {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl Display for MessageDeletedPayload {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).expect("failed to serialize MessageDeletedPayload")
        )
    }
}

/// MESSAGE_UPDATEDペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_direct_message_updated.go#L10-L14)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/message.md#message_updated)
///
/// ## Example
/// ```
/// use traq_bot_http::payloads::MessageUpdatedPayload;
/// let payload = r##"{
///     "eventTime": "2019-05-08T13:33:51.690308239Z",
///     "message": {
///         "id": "bc9106b3-f9b2-4eca-9ba1-72b39b40954e",
///         "user": {
///             "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45",
///             "name": "takashi_trap",
///             "displayName": "寺田 健二",
///             "iconId": "2bc06cda-bdb9-4a68-8000-62f907f36a92",
///             "bot": false
///         },
///         "channelId": "9aba50da-f605-4cd0-a428-5e4558cb911e",
///         "text": "!{\"type\": \"user\", \"raw\": \"@takashi_trap\", \"id\": \"dfdff0c9-5de0-46ee-9721-2525e8bb3d45\"} こんにちは",
///         "plainText": "@takashi_trap こんにちは",
///         "embedded": [
///             {
///                 "raw": "@takashi_trap",
///                 "type": "user",
///                 "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45"
///             }
///         ],
///         "createdAt": "2019-05-08T13:33:51.632149265Z",
///         "updatedAt": "2019-05-08T13:33:51.632149265Z"
///     }
/// }"##;
/// let payload = serde_json::from_str::<MessageUpdatedPayload>(payload).unwrap();
/// println!("{payload:?}");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct MessageUpdatedPayload {
    #[serde(rename = "eventTime", with = "crate::payloads::serde::timestamp")]
    pub event_time: TimeStamp,
    pub message: Message,
}

impl FromStr for MessageUpdatedPayload {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl Display for MessageUpdatedPayload {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).expect("failed to serialize MessageUpdatedPayload")
        )
    }
}

impl From<DirectMessageUpdatedPayload> for MessageUpdatedPayload {
    fn from(payload: DirectMessageUpdatedPayload) -> Self {
        let DirectMessageUpdatedPayload {
            event_time,
            message,
        } = payload;
        Self {
            event_time,
            message,
        }
    }
}

/// DIRECT_MESSAGE_CREATEDペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_direct_message_created.go#L10-L14)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/message.md#direct_message_created)
///
/// ## Example
/// ```
/// use traq_bot_http::payloads::DirectMessageCreatedPayload;
/// let payload = r##"{
///     "eventTime": "2019-05-08T13:36:09.421492525Z",
///     "message": {
///         "id": "2d7ff3f5-c313-4f4a-a9bb-0b5f84d2b6f8",
///         "user": {
///             "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45",
///             "name": "takashi_trap",
///             "displayName": "寺田 健二",
///             "iconId": "2bc06cda-bdb9-4a68-8000-62f907f36a92",
///             "bot": false
///         },
///         "channelId": "c5a5a697-3bad-4540-b2da-93dc88181d34",
///         "text": "!{\"type\": \"user\", \"raw\": \"@takashi_trap\", \"id\": \"dfdff0c9-5de0-46ee-9721-2525e8bb3d45\"} こんにちは",
///         "plainText": "@takashi_trap こんにちは",
///         "embedded": [
///             {
///                 "raw": "@takashi_trap",
///                 "type": "user",
///                 "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45"
///             }
///         ],
///         "createdAt": "2019-05-08T13:36:09.365393261Z",
///         "updatedAt": "2019-05-08T13:36:09.365393261Z"
///     }
/// }"##;
/// let payload = serde_json::from_str::<DirectMessageCreatedPayload>(payload).unwrap();
/// println!("{payload:?}");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct DirectMessageCreatedPayload {
    #[serde(rename = "eventTime", with = "crate::payloads::serde::timestamp")]
    pub event_time: TimeStamp,
    pub message: Message,
}

impl FromStr for DirectMessageCreatedPayload {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl Display for DirectMessageCreatedPayload {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).expect("failed to serialize DirectMessageCreatedPayload")
        )
    }
}

impl From<MessageCreatedPayload> for DirectMessageCreatedPayload {
    fn from(payload: MessageCreatedPayload) -> Self {
        let MessageCreatedPayload {
            event_time,
            message,
        } = payload;
        Self {
            event_time,
            message,
        }
    }
}

/// DIRECT_MESSAGE_DELETEDペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_direct_message_deleted.go#L11-L19)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/message.md#direct_message_deleted)
///
/// ## Example
/// ```
/// use traq_bot_http::payloads::DirectMessageDeletedPayload;
/// let payload = r##"{
///     "eventTime": "2019-05-08T13:36:09.421492525Z",
///     "message": {
///         "id": "2d7ff3f5-c313-4f4a-a9bb-0b5f84d2b6f8",
///         "userId": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45",
///         "channelId": "c5a5a697-3bad-4540-b2da-93dc88181d34"
///     }
/// }"##;
/// let payload = serde_json::from_str::<DirectMessageDeletedPayload>(payload).unwrap();
/// println!("{payload:?}");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct DirectMessageDeletedPayload {
    #[serde(rename = "eventTime", with = "crate::payloads::serde::timestamp")]
    pub event_time: TimeStamp,
    pub message: DeletedDirectMessage,
}

impl FromStr for DirectMessageDeletedPayload {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl Display for DirectMessageDeletedPayload {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).expect("failed to serialize DirectMessageDeletedPayload")
        )
    }
}

/// DIRECT_MESSAGE_UPDATEDペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_direct_message_updated.go#L10-L14)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/message.md#direct_message_updated)
///
/// ## Example
/// ```
/// use traq_bot_http::payloads::DirectMessageUpdatedPayload;
/// let payload = r##"{
///     "eventTime": "2019-05-08T13:36:09.421492525Z",
///     "message": {
///         "id": "2d7ff3f5-c313-4f4a-a9bb-0b5f84d2b6f8",
///         "user": {
///             "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45",
///             "name": "takashi_trap",
///             "displayName": "寺田 健二",
///             "iconId": "2bc06cda-bdb9-4a68-8000-62f907f36a92",
///             "bot": false
///         },
///         "channelId": "c5a5a697-3bad-4540-b2da-93dc88181d34",
///         "text": "!{\"type\": \"user\", \"raw\": \"@takashi_trap\", \"id\": \"dfdff0c9-5de0-46ee-9721-2525e8bb3d45\"} こんにちは",
///         "plainText": "@takashi_trap こんにちは",
///         "embedded": [
///             {
///                 "raw": "@takashi_trap",
///                 "type": "user",
///                 "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45"
///             }
///         ],
///         "createdAt": "2019-05-08T13:36:09.365393261Z",
///         "updatedAt": "2019-05-08T13:36:09.365393261Z"
///     }
/// }"##;
/// let payload = serde_json::from_str::<DirectMessageUpdatedPayload>(payload).unwrap();
/// println!("{payload:?}");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct DirectMessageUpdatedPayload {
    #[serde(rename = "eventTime", with = "crate::payloads::serde::timestamp")]
    pub event_time: TimeStamp,
    pub message: Message,
}

impl FromStr for DirectMessageUpdatedPayload {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl Display for DirectMessageUpdatedPayload {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).expect("failed to serialize DirectMessageUpdatedPayload")
        )
    }
}

impl From<MessageUpdatedPayload> for DirectMessageUpdatedPayload {
    fn from(payload: MessageUpdatedPayload) -> Self {
        let MessageUpdatedPayload {
            event_time,
            message,
        } = payload;
        Self {
            event_time,
            message,
        }
    }
}

/// BOT_MESSAGE_STAMPS_UPDATEDペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_bot_message_stamps_updated.go#L11-L16)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/message.md#bot_message_stamps_updated)
///
/// ## Example
/// ```
/// use traq_bot_http::payloads::BotMessageStampsUpdatedPayload;
/// let payload = r##"{
///     "eventTime": "2020-10-17T03:35:34.5326265Z",
///     "messageId": "200b6600-b2cd-4c1e-b366-9c40308cc087",
///     "stamps": [
///         {
///             "stampId": "1cd58034-8998-4b1c-afe4-fcd591354a97",
///             "userId": "b80551a5-2768-4d29-ad78-8e0e92330c8d",
///             "count": 22,
///             "createdAt": "2020-10-17T03:35:17.89545Z",
///             "updatedAt": "2020-10-17T03:35:34Z"
///         },
///         {
///             "stampId": "6fc62b49-dea0-45b8-8c0c-38035082b111",
///             "userId": "b80551a5-2768-4d29-ad78-8e0e92330c8d",
///             "count": 23,
///             "createdAt": "2020-10-17T03:35:17.737127Z",
///             "updatedAt": "2020-10-17T03:35:34Z"
///         },
///         {
///             "stampId": "b77fad4e-b63f-42a2-916c-5cfe5af3d8b9",
///             "userId": "b80551a5-2768-4d29-ad78-8e0e92330c8d",
///             "count": 24,
///             "createdAt": "2020-10-17T03:34:56.575099Z",
///             "updatedAt": "2020-10-17T03:35:34Z"
///         }
///     ]
/// }"##;
/// let payload = serde_json::from_str::<BotMessageStampsUpdatedPayload>(payload).unwrap();
/// println!("{payload:?}");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct BotMessageStampsUpdatedPayload {
    #[serde(rename = "eventTime", with = "crate::payloads::serde::timestamp")]
    pub event_time: TimeStamp,
    #[serde(rename = "messageId")]
    pub message_id: Uuid,
    pub stamps: Vec<MessageStamp>,
}

impl FromStr for BotMessageStampsUpdatedPayload {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl Display for BotMessageStampsUpdatedPayload {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self)
                .expect("failed to serialize BotMessageStampsUpdatedPayload")
        )
    }
}
