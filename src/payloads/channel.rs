//! チャンネル関連のイベントペイロード
//! ## types
//! - [`ChannelCreatedPayload`](ChannelCreatedPayload)
//! - [`ChannelTopicChangedPayload`](ChannelTopicChangedPayload)

use serde::{Deserialize, Serialize};

use super::types::{Channel, TimeStamp, User};
use crate::macros::payload_impl;

/// CHANNEL_CREATEDペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_channel_created.go#L9-L13)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/channel.md#channel_created)
///
/// ## Example
/// ```
/// use traq_bot_http::payloads::ChannelCreatedPayload;
/// let payload = r##"{
///     "eventTime": "2019-05-08T13:45:51.506206852Z",
///     "channel": {
///         "id": "711afb4c-23e7-46dc-b845-5160f7088ce9",
///         "name": "yamada",
///         "path": "#gps/yamada",
///         "parentId": "ea452867-553b-4808-a14f-a47ee0009ee6",
///         "creator": {
///             "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45",
///             "name": "takashi_trap",
///             "displayName": "寺田 健二",
///             "iconId": "2bc06cda-bdb9-4a68-8000-62f907f36a92",
///             "bot": false
///         },
///         "createdAt": "2019-05-08T13:45:51.487718Z",
///         "updatedAt": "2019-05-08T13:45:51.487718Z"
///     }
/// }"##;
/// let payload: ChannelCreatedPayload = payload.parse().unwrap();
/// println!("{payload}");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelCreatedPayload {
    #[serde(with = "crate::payloads::serde::timestamp")]
    pub event_time: TimeStamp,
    pub channel: Channel,
}

payload_impl! {ChannelCreatedPayload}

/// CHANNEL_TOPIC_CHANGEDペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_channel_topic_changed.go#L9-L15)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/channel.md#channel_topic_changed)
///
/// ## Example
/// ```
/// use traq_bot_http::payloads::ChannelTopicChangedPayload;
/// let payload = r##"{
///     "eventTime": "2019-05-09T11:32:49.505357701Z",
///     "channel": {
///         "id": "9aba50da-f605-4cd0-a428-5e4558cb911e",
///         "name": "bot",
///         "path": "#a/bot",
///         "parentId": "ea452867-553b-4808-a14f-a47ee0009ee6",
///         "creator": {
///             "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45",
///             "name": "takashi_trap",
///             "displayName": "寺田 健二",
///             "iconId": "2bc06cda-bdb9-4a68-8000-62f907f36a92",
///             "bot": false
///         },
///         "createdAt": "2019-05-08T13:45:51.487718Z",
///         "updatedAt": "2019-05-08T13:45:51.487718Z"
///     },
///     "topic": "トピック",
///     "updater": {
///         "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45",
///         "name": "takashi_trap",
///         "displayName": "寺田 健二",
///         "iconId": "2bc06cda-bdb9-4a68-8000-62f907f36a92",
///         "bot": false
///     }
/// }"##;
/// let payload: ChannelTopicChangedPayload = payload.parse().unwrap();
/// println!("{payload}");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelTopicChangedPayload {
    #[serde(with = "crate::payloads::serde::timestamp")]
    pub event_time: TimeStamp,
    pub channel: Channel,
    pub topic: String,
    pub updater: User,
}

payload_impl! {ChannelTopicChangedPayload}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    use std::fs::read_to_string;

    #[test]
    fn channel_created_test() {
        let data = read_to_string("testdata/channel/channel_created.json").unwrap();
        let payload: ChannelCreatedPayload = data.parse().unwrap();
        println!("{}", serde_json::to_string_pretty(&payload).unwrap());
        println!("{}", payload);
        assert_eq!(
            payload,
            ChannelCreatedPayload {
                event_time: timestamp("2019-05-08T13:45:51.506206852Z"),
                channel: Channel {
                    id: uuid("711afb4c-23e7-46dc-b845-5160f7088ce9"),
                    name: "yamada".to_string(),
                    path: "#gps/yamada".to_string(),
                    parent_id: uuid("ea452867-553b-4808-a14f-a47ee0009ee6"),
                    creator: takashi_trap(),
                    created_at: timestamp("2019-05-08T13:45:51.487718Z"),
                    updated_at: timestamp("2019-05-08T13:45:51.487718Z"),
                },
            }
        );
    }

    #[test]
    fn channel_topic_changed_test() {
        let data = read_to_string("testdata/channel/channel_topic_changed.json").unwrap();
        let payload: ChannelTopicChangedPayload = data.parse().unwrap();
        println!("{}", serde_json::to_string_pretty(&payload).unwrap());
        println!("{}", payload);
        assert_eq!(
            payload,
            ChannelTopicChangedPayload {
                event_time: timestamp("2019-05-09T11:32:49.505357701Z"),
                channel: Channel {
                    id: uuid("9aba50da-f605-4cd0-a428-5e4558cb911e"),
                    name: "bot".to_string(),
                    path: "#a/bot".to_string(),
                    parent_id: uuid("ea452867-553b-4808-a14f-a47ee0009ee6"),
                    creator: takashi_trap(),
                    created_at: timestamp("2019-04-02T06:31:16.229419Z"),
                    updated_at: timestamp("2019-05-09T11:32:49.475127Z"),
                },
                topic: "トピック".to_string(),
                updater: takashi_trap(),
            }
        );
    }
}
