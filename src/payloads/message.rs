//! メッセージ関連のイベントペイロード
//! ## types
//! - [`MessageCreatedPayload`](MessageCreatedPayload)
//! - [`MessageDeletedPayload`](MessageDeletedPayload)
//! - [`MessageUpdatedPayload`](MessageUpdatedPayload)
//! - [`DirectMessageCreatedPayload`](DirectMessageCreatedPayload)
//! - [`DirectMessageDeletedPayload`](DirectMessageDeletedPayload)
//! - [`DirectMessageUpdatedPayload`](DirectMessageUpdatedPayload)
//! - [`BotMessageStampsUpdatedPayload`](BotMessageStampsUpdatedPayload)

use serde::{Deserialize, Serialize};

use super::types::{DeletedDirectMessage, DeletedMessage, Message, MessageStamp, TimeStamp, Uuid};

/// `MESSAGE_CREATED`ペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_message_created.go#L10-L14)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/message.md#message_created)
///
/// ## Example
/// ```
/// # fn main() -> Result<(), serde_json::Error> {
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
/// let payload: MessageCreatedPayload = payload.parse()?;
/// println!("{payload}");
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageCreatedPayload {
    #[serde(with = "crate::payloads::serde::timestamp")]
    pub event_time: TimeStamp,
    pub message: Message,
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

/// `MESSAGE_DELETED`ペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_message_deleted.go#L11-L18)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/message.md#message_deleted)
///
/// ## Example
/// ```
/// # fn main() -> Result<(), serde_json::Error> {
/// use traq_bot_http::payloads::MessageDeletedPayload;
/// let payload = r##"{
///     "eventTime": "2019-05-08T13:33:51.690308239Z",
///     "message": {
///         "id": "bc9106b3-f9b2-4eca-9ba1-72b39b40954e",
///         "channelId": "9aba50da-f605-4cd0-a428-5e4558cb911e"
///     }
/// }"##;
/// let payload = serde_json::from_str::<MessageDeletedPayload>(payload)?;
/// println!("{payload}");
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageDeletedPayload {
    #[serde(with = "crate::payloads::serde::timestamp")]
    pub event_time: TimeStamp,
    pub message: DeletedMessage,
}

/// `MESSAGE_UPDATED`ペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_direct_message_updated.go#L10-L14)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/message.md#message_updated)
///
/// ## Example
/// ```
/// # fn main() -> Result<(), serde_json::Error> {
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
/// let payload = serde_json::from_str::<MessageUpdatedPayload>(payload)?;
/// println!("{payload}");
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageUpdatedPayload {
    #[serde(with = "crate::payloads::serde::timestamp")]
    pub event_time: TimeStamp,
    pub message: Message,
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

/// `DIRECT_MESSAGE_CREATED`ペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_direct_message_created.go#L10-L14)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/message.md#direct_message_created)
///
/// ## Example
/// ```
/// # fn main() -> Result<(), serde_json::Error> {
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
/// let payload = serde_json::from_str::<DirectMessageCreatedPayload>(payload)?;
/// println!("{payload}");
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectMessageCreatedPayload {
    #[serde(with = "crate::payloads::serde::timestamp")]
    pub event_time: TimeStamp,
    pub message: Message,
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

/// `DIRECT_MESSAGE_DELETED`ペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_direct_message_deleted.go#L11-L19)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/message.md#direct_message_deleted)
///
/// ## Example
/// ```
/// # fn main() -> Result<(), serde_json::Error> {
/// use traq_bot_http::payloads::DirectMessageDeletedPayload;
/// let payload = r##"{
///     "eventTime": "2019-05-08T13:36:09.421492525Z",
///     "message": {
///         "id": "2d7ff3f5-c313-4f4a-a9bb-0b5f84d2b6f8",
///         "userId": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45",
///         "channelId": "c5a5a697-3bad-4540-b2da-93dc88181d34"
///     }
/// }"##;
/// let payload = serde_json::from_str::<DirectMessageDeletedPayload>(payload)?;
/// println!("{payload}");
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectMessageDeletedPayload {
    #[serde(with = "crate::payloads::serde::timestamp")]
    pub event_time: TimeStamp,
    pub message: DeletedDirectMessage,
}

/// `DIRECT_MESSAGE_UPDATED`ペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_direct_message_updated.go#L10-L14)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/message.md#direct_message_updated)
///
/// ## Example
/// ```
/// # fn main() -> Result<(), serde_json::Error> {
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
/// let payload = serde_json::from_str::<DirectMessageUpdatedPayload>(payload)?;
/// println!("{payload}");
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectMessageUpdatedPayload {
    #[serde(with = "crate::payloads::serde::timestamp")]
    pub event_time: TimeStamp,
    pub message: Message,
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

/// `BOT_MESSAGE_STAMPS_UPDATED`ペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_bot_message_stamps_updated.go#L11-L16)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/message.md#bot_message_stamps_updated)
///
/// ## Example
/// ```
/// # fn main() -> Result<(), serde_json::Error> {
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
/// let payload = serde_json::from_str::<BotMessageStampsUpdatedPayload>(payload)?;
/// println!("{payload}");
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BotMessageStampsUpdatedPayload {
    #[serde(with = "crate::payloads::serde::timestamp")]
    pub event_time: TimeStamp,
    pub message_id: Uuid,
    pub stamps: Vec<MessageStamp>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{embedded_takashi_trap, takashi_trap, timestamp, uuid};

    use std::fs::read_to_string;

    #[test]
    fn message_created_test() {
        let data = read_to_string("testdata/message/message_created.json").unwrap();
        let payload: MessageCreatedPayload = data.parse().unwrap();
        let pretty_payload = serde_json::to_string_pretty(&payload).unwrap();
        println!("{pretty_payload}");
        println!("{payload}");
        assert_eq!(
            payload,
            MessageCreatedPayload {
                event_time: timestamp("2019-05-08T13:33:51.690308239Z"),
                message: Message {
                    id: uuid("bc9106b3-f9b2-4eca-9ba1-72b39b40954e"),
                    user: takashi_trap(),
                    channel_id: uuid("9aba50da-f605-4cd0-a428-5e4558cb911e"),
                    text: r#"!{"type": "user", "raw": "@takashi_trap", "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45"} こんにちは"#.to_string(),
                    plain_text: "@takashi_trap こんにちは".to_string(),
                    embedded: vec![
                        embedded_takashi_trap(),
                    ],
                    created_at: timestamp("2019-05-08T13:33:51.632149265Z"),
                    updated_at: timestamp("2019-05-08T13:33:51.632149265Z"),
                },
            }
        );
    }

    #[test]
    fn message_deleted_test() {
        let data = read_to_string("testdata/message/message_deleted.json").unwrap();
        let payload: MessageDeletedPayload = data.parse().unwrap();
        let pretty_payload = serde_json::to_string_pretty(&payload).unwrap();
        println!("{pretty_payload}");
        println!("{payload}");
        assert_eq!(
            payload,
            MessageDeletedPayload {
                event_time: timestamp("2019-05-08T13:33:51.690308239Z"),
                message: DeletedMessage {
                    id: uuid("bc9106b3-f9b2-4eca-9ba1-72b39b40954e"),
                    channel_id: uuid("9aba50da-f605-4cd0-a428-5e4558cb911e"),
                },
            }
        );
    }

    #[test]
    fn message_updated_test() {
        let data = read_to_string("testdata/message/message_updated.json").unwrap();
        let payload: MessageUpdatedPayload = data.parse().unwrap();
        let pretty_payload = serde_json::to_string_pretty(&payload).unwrap();
        println!("{pretty_payload}");
        println!("{payload}");
        assert_eq!(
            payload,
            MessageUpdatedPayload {
                event_time: timestamp("2019-05-08T13:33:51.690308239Z"),
                message: Message {
                    id: uuid("bc9106b3-f9b2-4eca-9ba1-72b39b40954e"),
                    user: takashi_trap(),
                    channel_id: uuid("9aba50da-f605-4cd0-a428-5e4558cb911e"),
                    text: r#"!{"type": "user", "raw": "@takashi_trap", "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45"} こんにちは"#.to_string(),
                    plain_text: "@takashi_trap こんにちは".to_string(),
                    embedded: vec![
                        embedded_takashi_trap(),
                    ],
                    created_at: timestamp("2019-05-08T13:33:51.632149265Z"),
                    updated_at: timestamp("2019-05-08T13:33:51.632149265Z"),
                }
            }
        );
    }

    #[test]
    fn direct_message_created_test() {
        let data = read_to_string("testdata/message/direct_message_created.json").unwrap();
        let payload: DirectMessageCreatedPayload = data.parse().unwrap();
        let pretty_payload = serde_json::to_string_pretty(&payload).unwrap();
        println!("{pretty_payload}");
        println!("{payload}");
        assert_eq!(
            payload,
            DirectMessageCreatedPayload {
                event_time: timestamp("2019-05-08T13:36:09.421492525Z"),
                message: Message {
                    id: uuid("2d7ff3f5-c313-4f4a-a9bb-0b5f84d2b6f8"),
                    user: takashi_trap(),
                    channel_id: uuid("c5a5a697-3bad-4540-b2da-93dc88181d34"),
                    text: r#"!{"type": "user", "raw": "@takashi_trap", "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45"} こんにちは"#.to_string(),
                    plain_text: "@takashi_trap こんにちは".to_string(),
                    embedded: vec![
                        embedded_takashi_trap(),
                    ],
                    created_at: timestamp("2019-05-08T13:36:09.365393261Z"),
                    updated_at: timestamp("2019-05-08T13:36:09.365393261Z"),
                },
            }
        );
    }

    #[test]
    fn direct_message_deleted_test() {
        let data = read_to_string("testdata/message/direct_message_deleted.json").unwrap();
        let payload: DirectMessageDeletedPayload = data.parse().unwrap();
        let pretty_payload = serde_json::to_string_pretty(&payload).unwrap();
        println!("{pretty_payload}");
        println!("{payload}");
        assert_eq!(
            payload,
            DirectMessageDeletedPayload {
                event_time: timestamp("2019-05-08T13:36:09.421492525Z"),
                message: DeletedDirectMessage {
                    id: uuid("2d7ff3f5-c313-4f4a-a9bb-0b5f84d2b6f8"),
                    user_id: uuid("dfdff0c9-5de0-46ee-9721-2525e8bb3d45"),
                    channel_id: uuid("c5a5a697-3bad-4540-b2da-93dc88181d34"),
                }
            }
        );
    }

    #[test]
    fn direct_message_updated_test() {
        let data = read_to_string("testdata/message/direct_message_updated.json").unwrap();
        let payload: DirectMessageUpdatedPayload = data.parse().unwrap();
        let pretty_payload = serde_json::to_string_pretty(&payload).unwrap();
        println!("{pretty_payload}");
        println!("{payload}");
        assert_eq!(
            payload,
            DirectMessageUpdatedPayload {
                event_time: timestamp("2019-05-08T13:36:09.421492525Z"),
                message: Message {
                    id: uuid("2d7ff3f5-c313-4f4a-a9bb-0b5f84d2b6f8"),
                    user: takashi_trap(),
                    channel_id: uuid("c5a5a697-3bad-4540-b2da-93dc88181d34"),
                    text: r#"!{"type": "user", "raw": "@takashi_trap", "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45"} こんにちは"#.to_string(),
                    plain_text: "@takashi_trap こんにちは".to_string(),
                    embedded: vec![
                        embedded_takashi_trap(),
                    ],
                    created_at: timestamp("2019-05-08T13:36:09.365393261Z"),
                    updated_at: timestamp("2019-05-08T13:36:09.365393261Z"),
                },
            }
        );
    }

    #[test]
    fn bot_message_stamps_updated_test() {
        let data = read_to_string("testdata/message/bot_message_stamps_updated.json").unwrap();
        let payload: BotMessageStampsUpdatedPayload = data.parse().unwrap();
        let pretty_payload = serde_json::to_string_pretty(&payload).unwrap();
        println!("{pretty_payload}");
        println!("{payload}");
        assert_eq!(
            payload,
            BotMessageStampsUpdatedPayload {
                event_time: timestamp("2020-10-17T03:35:34.5326265Z"),
                message_id: uuid("200b6600-b2cd-4c1e-b366-9c40308cc087"),
                stamps: vec![
                    MessageStamp {
                        stamp_id: uuid("1cd58034-8998-4b1c-afe4-fcd591354a97"),
                        user_id: uuid("b80551a5-2768-4d29-ad78-8e0e92330c8d"),
                        count: 22,
                        created_at: timestamp("2020-10-17T03:35:17.89545Z"),
                        updated_at: timestamp("2020-10-17T03:35:34Z"),
                    },
                    MessageStamp {
                        stamp_id: uuid("6fc62b49-dea0-45b8-8c0c-38035082b111"),
                        user_id: uuid("b80551a5-2768-4d29-ad78-8e0e92330c8d"),
                        count: 23,
                        created_at: timestamp("2020-10-17T03:35:17.737127Z"),
                        updated_at: timestamp("2020-10-17T03:35:34Z"),
                    },
                    MessageStamp {
                        stamp_id: uuid("b77fad4e-b63f-42a2-916c-5cfe5af3d8b9"),
                        user_id: uuid("b80551a5-2768-4d29-ad78-8e0e92330c8d"),
                        count: 24,
                        created_at: timestamp("2020-10-17T03:34:56.575099Z"),
                        updated_at: timestamp("2020-10-17T03:35:34Z"),
                    },
                ],
            }
        );
    }
}
