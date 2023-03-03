//! スタンプ関連のイベントペイロード

use serde::{Deserialize, Serialize};

use super::types::User;

/// STAMP_CREATEDペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_stamp_created.go#L11-L18)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/stamp.md#stamp_created)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct StampCreatedPayload {
    #[serde(rename = "eventTime")]
    pub event_time: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "fileId")]
    pub file_id: String,
    pub creator: User,
}
