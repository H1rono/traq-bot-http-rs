use serde::{Deserialize, Serialize};

/// TAG_ADDEDペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_tag_added.go#L11-L16)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/tag.md#tag_added)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct TagAddedPayload {
    #[serde(rename = "eventTime")]
    pub event_time: String,
    #[serde(rename = "tagId")]
    pub tag_id: String,
    pub tag: String,
}

/// TAG_REMOVEDペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_tag_removed.go#L11-L16)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/tag.md#tag_removed)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct TagRemovedPayload {
    #[serde(rename = "eventTime")]
    pub event_time: String,
    #[serde(rename = "tagId")]
    pub tag_id: String,
    pub tag: String,
}
