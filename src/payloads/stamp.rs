//! スタンプ関連のイベントペイロード

use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::types::{TimeStamp, User, Uuid};
use crate::payload_impl;

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
/// let payload: StampCreatedPayload = payload.parse().unwrap();
/// println!("{payload}");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct StampCreatedPayload {
    #[serde(rename = "eventTime", with = "crate::payloads::serde::timestamp")]
    pub event_time: TimeStamp,
    pub id: Uuid,
    pub name: String,
    #[serde(rename = "fileId")]
    pub file_id: Uuid,
    pub creator: User,
}

payload_impl! {StampCreatedPayload}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    use std::fs::read_to_string;

    #[test]
    fn stamp_created_test() {
        let data = read_to_string("testdata/stamp/stamp_created.json").unwrap();
        let payload: StampCreatedPayload = data.parse().unwrap();
        println!("{}", serde_json::to_string_pretty(&payload).unwrap());
        println!("{}", payload);
        assert_eq!(
            payload,
            StampCreatedPayload {
                event_time: timestamp("2019-05-08T08:31:06.566228282Z"),
                id: uuid("2bc06cda-bdb9-4a68-8000-62f907f36a92"),
                name: "naruhodo".to_string(),
                file_id: uuid("2bc06cda-bdb9-4a68-8000-62f907f36a92"),
                creator: {
                    let mut creator = takashi_trap();
                    creator.display_name.clear();
                    creator
                },
            }
        )
    }
}
