//! スタンプ関連のイベントペイロード

use serde::{Deserialize, Serialize};

use super::types::User;

/// STAMP_CREATEDペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_stamp_created.go#L11-L18)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/stamp.md#stamp_created)
///
/// ## Example
/// ```
/// use traq_bot_http::payloads::StampCreatedPayload;
/// let payload = r##"{
///     "eventTime": "2019-05-08T08:31:06.566228282Z",
///     "id": "2bc06cda-bdb9-4a68-8000-62f907f36a92",
///     "name": "naruhodo",
///     "fileId": "2bc06cda-bdb9-4a68-8000-62f907f36a92",
///     "creator": {
///         "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45",
///         "name": "takashi_trap",
///         "displayName": "",
///         "iconId": "2bc06cda-bdb9-4a68-8000-62f907f36a92",
///         "bot": false
///     }
/// }"##;
/// let payload: StampCreatedPayload = serde_json::from_str(payload).unwrap();
/// println!("{payload:?}");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct StampCreatedPayload {
    #[serde(rename = "eventTime")]
    pub event_time: String,
    #[cfg(feature = "uuid")]
    pub id: uuid::Uuid,
    #[cfg(not(feature = "uuid"))]
    pub id: String,
    pub name: String,
    #[cfg(feature = "uuid")]
    #[serde(rename = "fileId")]
    pub file_id: uuid::Uuid,
    #[cfg(not(feature = "uuid"))]
    #[serde(rename = "fileId")]
    pub file_id: String,
    pub creator: User,
}
