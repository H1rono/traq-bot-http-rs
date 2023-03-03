//! ユーザー関連のイベントペイロード

use serde::{Deserialize, Serialize};

use super::types::User;

/// USER_CREATEDペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_user_created.go#L9-L13)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/user.md#user_created)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserCreatedPayload {
    #[serde(rename = "eventTime")]
    event_time: String,
    user: User,
}
