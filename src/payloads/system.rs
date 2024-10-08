//! システム関連のイベントペイロード
//! ## types
//! - [`PingPayload`](PingPayload)
//! - [`JoinedPayload`](JoinedPayload)
//! - [`LeftPayload`](LeftPayload)

use serde::{Deserialize, Serialize};

use super::types::{Channel, TimeStamp};

/// `PING`ペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_ping.go#L5-L8)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/system.md#ping)
///
/// ## Example
/// ```
/// # fn main() -> Result<(), serde_json::Error> {
/// use traq_bot_http::payloads::PingPayload;
/// let payload = r##"{
///     "eventTime": "2019-05-07T04:50:48.582586882Z"
/// }"##;
/// let payload: PingPayload = payload.parse()?;
/// println!("{payload}");
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PingPayload {
    #[serde(with = "crate::payloads::serde::timestamp")]
    pub event_time: TimeStamp,
}

/// `JOINED`ペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_joined.go#L9-L13)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/system.md#joined)
///
/// ## Example
/// ```
/// # fn main() -> Result<(), serde_json::Error> {
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
/// let payload: JoinedPayload = payload.parse()?;
/// println!("{payload}");
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JoinedPayload {
    #[serde(with = "crate::payloads::serde::timestamp")]
    pub event_time: TimeStamp,
    pub channel: Channel,
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

/// `LEFT`ペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_left.go#L9-L13)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/system.md#left)
///
/// ## Example
/// ```
/// # fn main() -> Result<(), serde_json::Error> {
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
/// let payload: LeftPayload = payload.parse()?;
/// println!("{payload}");
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LeftPayload {
    #[serde(with = "crate::payloads::serde::timestamp")]
    pub event_time: TimeStamp,
    pub channel: Channel,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{channel_a_po, timestamp};

    use std::fs::read_to_string;

    #[test]
    fn ping_test() {
        let data = read_to_string("testdata/system/ping.json").unwrap();
        let payload: PingPayload = data.parse().unwrap();
        let pretty_payload = serde_json::to_string_pretty(&payload).unwrap();
        println!("{pretty_payload}");
        println!("{payload}");
        assert_eq!(
            payload,
            PingPayload {
                event_time: timestamp("2019-05-07T04:50:48.582586882Z")
            }
        );
    }

    #[test]
    fn joined_test() {
        let data = read_to_string("testdata/system/joined.json").unwrap();
        let payload: JoinedPayload = data.parse().unwrap();
        let pretty_payload = serde_json::to_string_pretty(&payload).unwrap();
        println!("{pretty_payload}");
        println!("{payload}");
        assert_eq!(
            payload,
            JoinedPayload {
                event_time: timestamp("2019-05-08T13:49:13.769110201Z"),
                channel: channel_a_po(),
            }
        );
    }

    #[test]
    fn left_test() {
        let data = read_to_string("testdata/system/left.json").unwrap();
        let payload: LeftPayload = data.parse().unwrap();
        let pretty_payload = serde_json::to_string_pretty(&payload).unwrap();
        println!("{pretty_payload}");
        println!("{payload}");
        assert_eq!(
            payload,
            LeftPayload {
                event_time: timestamp("2019-05-08T13:49:16.497848449Z"),
                channel: channel_a_po(),
            },
        );
    }
}
