//! ユーザー関連のイベントペイロード

use serde::{Deserialize, Serialize};

use super::types::{TimeStamp, User};
use crate::macros::payload_impl;

/// USER_CREATEDペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_user_created.go#L9-L13)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/user.md#user_created)
///
/// ## Example
/// ```
/// use traq_bot_http::payloads::UserCreatedPayload;
/// let payload = r##"{
///     "eventTime": "2019-05-08T08:31:06.566228282Z",
///     "user": {
///         "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45",
///         "name": "takashi_trap",
///         "displayName": "",
///         "iconId": "2bc06cda-bdb9-4a68-8000-62f907f36a92",
///         "bot": false
///     }
/// }"##;
/// let payload: UserCreatedPayload = payload.parse().unwrap();
/// println!("{payload}");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserCreatedPayload {
    #[serde(rename = "eventTime", with = "crate::payloads::serde::timestamp")]
    pub event_time: TimeStamp,
    pub user: User,
}

payload_impl! {UserCreatedPayload}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    use std::fs::read_to_string;

    #[test]
    fn user_created_test() {
        let data = read_to_string("testdata/user/user_created.json").unwrap();
        let payload: UserCreatedPayload = data.parse().unwrap();
        println!("{}", serde_json::to_string_pretty(&payload).unwrap());
        println!("{}", payload);
        assert_eq!(
            payload,
            UserCreatedPayload {
                event_time: timestamp("2019-05-08T08:31:06.566228282Z"),
                user: User {
                    id: uuid("dfdff0c9-5de0-46ee-9721-2525e8bb3d45"),
                    name: "takashi_trap".to_string(),
                    display_name: "".to_string(),
                    icon_id: uuid("2bc06cda-bdb9-4a68-8000-62f907f36a92"),
                    bot: false,
                }
            }
        )
    }
}
