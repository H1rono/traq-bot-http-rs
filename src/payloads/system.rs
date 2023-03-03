//! システム関連のイベントペイロード

use serde::{Deserialize, Serialize};

use super::types::Channel;

/// PINGペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_ping.go#L5-L8)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/system.md#ping)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct PingPayload {
    #[serde(rename = "eventTime")]
    pub event_time: String,
}

/// JOINEDペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_joined.go#L9-L13)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/system.md#joined)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct JoinedPayload {
    #[serde(rename = "eventTime")]
    pub event_time: String,
    pub channel: Channel,
}

/// LEFTペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_left.go#L9-L13)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/system.md#left)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct LeftPayload {
    #[serde(rename = "eventTime")]
    pub event_time: String,
    pub channel: Channel,
}
