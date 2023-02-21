// CHANNEL_CREATED
// CHANNEL_TOPIC_CHANGED

use serde::{Deserialize, Serialize};

use super::types::{Channel, User};

/// CHANNEL_CREATEDのペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_channel_created.go#L9-L13)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/channel.md#channel_created)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ChannelCreatedPayload {
    #[serde(rename = "eventTime")]
    pub event_time: String,
    pub channel: Channel,
}

/// CHANNEL_TOPIC_CHANGEDペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_channel_topic_changed.go#L9-L15)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/channel.md#channel_topic_changed)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ChannelTopicChangedPayload {
    #[serde(rename = "eventTime")]
    pub event_time: String,
    pub channel: Channel,
    pub topic: String,
    pub updater: User,
}
