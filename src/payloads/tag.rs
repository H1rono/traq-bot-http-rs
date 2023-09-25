//! タグ関連のイベントペイロード

use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::types::{TimeStamp, Uuid};
use crate::macros::payload_impl;

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
/// let payload: TagAddedPayload = payload.parse().unwrap();
/// println!("{payload}");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct TagAddedPayload {
    #[serde(rename = "eventTime", with = "crate::payloads::serde::timestamp")]
    pub event_time: TimeStamp,
    #[serde(rename = "tagId")]
    pub tag_id: Uuid,
    pub tag: String,
}

payload_impl! {TagAddedPayload}

impl From<TagRemovedPayload> for TagAddedPayload {
    fn from(payload: TagRemovedPayload) -> Self {
        let TagRemovedPayload {
            event_time,
            tag_id,
            tag,
        } = payload;
        Self {
            event_time,
            tag_id,
            tag,
        }
    }
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
/// let payload: TagRemovedPayload = payload.parse().unwrap();
/// println!("{payload}");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct TagRemovedPayload {
    #[serde(rename = "eventTime", with = "crate::payloads::serde::timestamp")]
    pub event_time: TimeStamp,
    #[serde(rename = "tagId")]
    pub tag_id: Uuid,
    pub tag: String,
}

payload_impl! {TagRemovedPayload}

impl From<TagAddedPayload> for TagRemovedPayload {
    fn from(payload: TagAddedPayload) -> Self {
        let TagAddedPayload {
            event_time,
            tag_id,
            tag,
        } = payload;
        Self {
            event_time,
            tag_id,
            tag,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    use std::fs::read_to_string;

    #[test]
    fn tag_added_test() {
        let data = read_to_string("testdata/tag/tag_added.json").unwrap();
        let payload: TagAddedPayload = data.parse().unwrap();
        println!("{}", serde_json::to_string_pretty(&payload).unwrap());
        println!("{}", payload);
        assert_eq!(
            payload,
            TagAddedPayload {
                event_time: timestamp("2019-05-08T08:31:06.566228282Z"),
                tag_id: uuid("2bc06cda-bdb9-4a68-8000-62f907f36a92"),
                tag: "全強".to_string(),
            }
        );
    }

    #[test]
    fn tag_removed_test() {
        let data = read_to_string("testdata/tag/tag_removed.json").unwrap();
        let payload: TagRemovedPayload = data.parse().unwrap();
        println!("{}", serde_json::to_string_pretty(&payload).unwrap());
        println!("{}", payload);
        assert_eq!(
            payload,
            TagRemovedPayload {
                event_time: timestamp("2019-05-08T08:31:06.566228282Z"),
                tag_id: uuid("2bc06cda-bdb9-4a68-8000-62f907f36a92"),
                tag: "全強".to_string(),
            }
        );
    }
}
