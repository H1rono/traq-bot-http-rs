//! メッセージ関連のイベントペイロード

use serde::{Deserialize, Serialize};

use super::types::{DeletedMessage, Message, MessageStamp};

/// MESSAGE_CREATEDペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_message_created.go#L10-L14)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/message.md#message_created)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct MessageCreatedPayload {
    #[serde(rename = "eventTime")]
    pub event_time: String,
    pub message: Message,
}

/// MESSAGE_DELETEDペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_message_deleted.go#L11-L18)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/message.md#message_deleted)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct MessageDeletedPayload {
    #[serde(rename = "eventTime")]
    pub event_time: String,
    pub message: DeletedMessage,
}

/// MESSAGE_UPDATEDペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_direct_message_updated.go#L10-L14)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/message.md#message_updated)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct MessageUpdatedPayload {
    #[serde(rename = "eventTime")]
    pub event_time: String,
    pub message: Message,
}

/// DIRECT_MESSAGE_CREATEDペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_direct_message_created.go#L10-L14)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/message.md#direct_message_created)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct DirectMessageCreatedPayload {
    #[serde(rename = "eventTime")]
    pub event_time: String,
    pub message: Message,
}

/// DIRECT_MESSAGE_DELETEDペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_direct_message_deleted.go#L11-L19)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/message.md#direct_message_deleted)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct DirectMessageDeletedPayload {
    #[serde(rename = "eventTime")]
    pub event_time: String,
    pub message: DeletedMessage,
}

/// DIRECT_MESSAGE_UPDATEDペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_direct_message_updated.go#L10-L14)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/message.md#direct_message_updated)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct DirectMessageUpdatedPayload {
    #[serde(rename = "eventTime")]
    pub event_time: String,
    pub message: Message,
}

/// BOT_MESSAGE_STAMPS_UPDATEDペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_bot_message_stamps_updated.go#L11-L16)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/message.md#bot_message_stamps_updated)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct BotMessageStampsUpdatedPayload {
    #[serde(rename = "eventTime")]
    pub event_time: String,
    #[serde(rename = "messageId")]
    pub message_id: String,
    pub stamps: Vec<MessageStamp>,
}
