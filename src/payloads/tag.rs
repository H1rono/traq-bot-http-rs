//! タグ関連のイベントペイロード

use serde::{Deserialize, Serialize};

use super::types::{TimeStamp, Uuid};

/// TAG_ADDEDペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_tag_added.go#L11-L16)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/tag.md#tag_added)
///
/// ## Example
/// ```
/// use traq_bot_http::payloads::TagAddedPayload;
/// let payload = r##"{
///     "eventTime": "2019-05-08T08:31:06.566228282Z",
///     "tagId": "2bc06cda-bdb9-4a68-8000-62f907f36a92",
///     "tag": "全強"
/// }"##;
/// let payload: TagAddedPayload = serde_json::from_str(payload).unwrap();
/// println!("{payload:?}");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct TagAddedPayload {
    #[serde(rename = "eventTime", with = "crate::payloads::serde::timestamp")]
    pub event_time: TimeStamp,
    #[serde(rename = "tagId")]
    pub tag_id: Uuid,
    pub tag: String,
}

/// TAG_REMOVEDペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_tag_removed.go#L11-L16)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/tag.md#tag_removed)
///
/// ## Example
/// ```
/// use traq_bot_http::payloads::TagRemovedPayload;
/// let payload = r##"{
///     "eventTime": "2019-05-08T08:31:06.566228282Z",
///     "tagId": "2bc06cda-bdb9-4a68-8000-62f907f36a92",
///     "tag": "全強"
/// }"##;
/// let payload: TagRemovedPayload = serde_json::from_str(payload).unwrap();
/// println!("{payload:?}");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct TagRemovedPayload {
    #[serde(rename = "eventTime", with = "crate::payloads::serde::timestamp")]
    pub event_time: TimeStamp,
    #[serde(rename = "tagId")]
    pub tag_id: Uuid,
    pub tag: String,
}
