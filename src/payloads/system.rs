//! システム関連のイベントペイロード

use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::types::{Channel, TimeStamp};

/// PINGペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_ping.go#L5-L8)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/system.md#ping)
///
/// ## Example
/// ```
/// use traq_bot_http::payloads::PingPayload;
/// let payload = r##"{
///     "eventTime": "2019-05-07T04:50:48.582586882Z"
/// }"##;
/// let payload: PingPayload = serde_json::from_str(payload).unwrap();
/// println!("{payload:?}");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct PingPayload {
    #[serde(rename = "eventTime", with = "crate::payloads::serde::timestamp")]
    pub event_time: TimeStamp,
}

impl FromStr for PingPayload {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl Display for PingPayload {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).expect("failed to serialize PingPayload")
        )
    }
}

/// JOINEDペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_joined.go#L9-L13)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/system.md#joined)
///
/// ## Example
/// ```
/// use traq_bot_http::payloads::JoinedPayload;
/// let payload = r##"{
///     "eventTime": "2019-05-08T13:49:13.769110201Z",
///     "channel": {
///         "id": "f86c925c-3002-4ba5-939a-c92344e534f9",
///         "name": "po",
///         "path": "#a/po",
///         "parentId": "ea452867-553b-4808-a14f-a47ee0009ee6",
///         "creator": {
///             "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45",
///             "name": "takashi_trap",
///             "displayName": "寺田 健二",
///             "iconId": "2bc06cda-bdb9-4a68-8000-62f907f36a92",
///             "bot": false
///         },
///         "createdAt": "2018-04-25T12:22:02Z",
///         "updatedAt": "2018-04-25T12:22:02Z"
///     }
/// }"##;
/// let payload: JoinedPayload = serde_json::from_str(payload).unwrap();
/// println!("{payload:?}");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct JoinedPayload {
    #[serde(rename = "eventTime", with = "crate::payloads::serde::timestamp")]
    pub event_time: TimeStamp,
    pub channel: Channel,
}

impl FromStr for JoinedPayload {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl Display for JoinedPayload {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).expect("failed to serialize JoinedPayload")
        )
    }
}

impl From<LeftPayload> for JoinedPayload {
    fn from(payload: LeftPayload) -> Self {
        let LeftPayload {
            event_time,
            channel,
        } = payload;
        Self {
            event_time,
            channel,
        }
    }
}

/// LEFTペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_left.go#L9-L13)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/system.md#left)
///
/// ## Example
/// ```
/// use traq_bot_http::payloads::LeftPayload;
/// let payload = r##"{
///     "eventTime": "2019-05-08T13:49:16.497848449Z",
///     "channel": {
///         "id": "f86c925c-3002-4ba5-939a-c92344e534f9",
///         "name": "po",
///         "path": "#a/po",
///         "parentId": "ea452867-553b-4808-a14f-a47ee0009ee6",
///         "creator": {
///             "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45",
///             "name": "takashi_trap",
///             "displayName": "寺田 健二",
///             "iconId": "2bc06cda-bdb9-4a68-8000-62f907f36a92",
///             "bot": false
///         },
///         "createdAt": "2018-04-25T12:22:02Z",
///         "updatedAt": "2018-04-25T12:22:02Z"
///     }
/// }"##;
/// let payload: LeftPayload = serde_json::from_str(payload).unwrap();
/// println!("{payload:?}");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct LeftPayload {
    #[serde(rename = "eventTime", with = "crate::payloads::serde::timestamp")]
    pub event_time: TimeStamp,
    pub channel: Channel,
}

impl FromStr for LeftPayload {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl Display for LeftPayload {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).expect("failed to serialize LeftPayload")
        )
    }
}

impl From<JoinedPayload> for LeftPayload {
    fn from(payload: JoinedPayload) -> Self {
        let JoinedPayload {
            event_time,
            channel,
        } = payload;
        Self {
            event_time,
            channel,
        }
    }
}
