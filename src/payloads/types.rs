//! イベントペイロード内部で使われる型

use serde::{Deserialize, Serialize};

#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};

#[cfg(not(feature = "chrono"))]
#[cfg(feature = "time")]
use time::OffsetDateTime;

use crate::macros::payload_impl;

#[cfg(feature = "chrono")]
/// タイムスタンプ([RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.6))を表現する型
///
/// デフォルトでは[`String`](https://doc.rust-lang.org/stable/alloc/string/struct.String.html)型、
/// `time` featureで[`time::OffsetDateTime`](https://docs.rs/time/latest/time/struct.OffsetDateTime.html)型になる
pub type TimeStamp = DateTime<Utc>;

#[cfg(not(feature = "chrono"))]
#[cfg(feature = "time")]
/// タイムスタンプ([RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.6))を表現する型
///
/// デフォルトでは[`String`](https://doc.rust-lang.org/stable/alloc/string/struct.String.html)型、
/// `chrono` featureで[`chrono::DateTime<chrono::Utc>`](https://docs.rs/chrono/latest/chrono/struct.DateTime.html)型になる
pub type TimeStamp = OffsetDateTime;

#[cfg(not(feature = "chrono"))]
#[cfg(not(feature = "time"))]
/// タイムスタンプ([RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.6))を表現する型
///
/// `time` featureで[`time::OffsetDateTime`](https://docs.rs/time/latest/time/struct.OffsetDateTime.html)型、
/// `chrono` featureで[`chrono::DateTime<chrono::Utc>`](https://docs.rs/chrono/latest/chrono/struct.DateTime.html)型になる
///
/// *`chrono` featureが優先
pub type TimeStamp = String;

#[cfg(feature = "uuid")]
/// UUIDを表現する型
///
/// デフォルトでは[`String`](https://doc.rust-lang.org/stable/alloc/string/struct.String.html)型
pub type Uuid = uuid::Uuid;

#[cfg(not(feature = "uuid"))]
/// UUIDを表現する型
///
/// `uuid` featureで[`uuid::Uuid`](https://docs.rs/uuid/latest/uuid/struct.Uuid.html)型になる
pub type Uuid = String;

/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/common.go#L69-L75)
///
/// ## Example
/// ```
/// use traq_bot_http::payloads::types::User;
/// let payload = r##"{
///     "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45",
///     "name": "takashi_trap",
///     "displayName": "寺田 健二",
///     "iconId": "2bc06cda-bdb9-4a68-8000-62f907f36a92",
///     "bot": false
/// }"##;
/// let payload: User = payload.parse().unwrap();
/// println!("{payload}");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub display_name: String,
    pub icon_id: Uuid,
    pub bot: bool,
}

payload_impl! {User}

/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/common.go#L47-L55)
///
/// ## Example
/// ```
/// use traq_bot_http::payloads::types::Channel;
/// let payload = r##"{
///     "id": "f86c925c-3002-4ba5-939a-c92344e534f9",
///     "name": "po",
///     "path": "#a/po",
///     "parentId": "ea452867-553b-4808-a14f-a47ee0009ee6",
///     "creator":{
///         "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45",
///         "name": "takashi_trap",
///         "displayName": "寺田 健二",
///         "iconId": "2bc06cda-bdb9-4a68-8000-62f907f36a92",
///         "bot": false
///     },
///     "createdAt": "2018-04-25T12:22:02Z",
///     "updatedAt": "2018-04-25T12:22:02Z"
/// }"##;
/// let payload: Channel = payload.parse().unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    pub id: Uuid,
    pub name: String,
    pub path: String,
    pub parent_id: Uuid,
    pub creator: User,
    #[serde(with = "crate::payloads::serde::timestamp")]
    pub created_at: TimeStamp,
    #[serde(with = "crate::payloads::serde::timestamp")]
    pub updated_at: TimeStamp,
}

payload_impl! {Channel}

/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/utils/message/embedded.go#L9-L14)
///
/// ## Example
/// ```
/// use traq_bot_http::payloads::types::EmbeddedInfo;
/// let payload = r##"{
///     "raw": "@takashi_trap",
///     "type": "user",
///     "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45"
/// }"##;
/// let payload: EmbeddedInfo = payload.parse().unwrap();
/// println!("{payload}");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmbeddedInfo {
    pub raw: String,
    pub r#type: String,
    pub id: Uuid,
}

payload_impl! {EmbeddedInfo}

/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/common.go#L23-L32)
///
/// ## Example
/// ```
/// use traq_bot_http::payloads::types::Message;
/// let payload = r##"{
///     "id": "bc9106b3-f9b2-4eca-9ba1-72b39b40954e",
///     "user": {
///         "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45",
///         "name": "takashi_trap",
///         "displayName": "寺田 健二",
///         "iconId": "2bc06cda-bdb9-4a68-8000-62f907f36a92",
///         "bot": false
///     },
///     "channelId": "9aba50da-f605-4cd0-a428-5e4558cb911e",
///     "text": "!{\"type\": \"user\", \"raw\": \"@takashi_trap\", \"id\": \"dfdff0c9-5de0-46ee-9721-2525e8bb3d45\"} こんにちは",
///     "plainText": "@takashi_trap こんにちは",
///     "embedded": [
///         {
///             "raw": "@takashi_trap",
///             "type": "user",
///             "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45"
///         }
///     ],
///     "createdAt": "2019-05-08T13:33:51.632149265Z",
///     "updatedAt": "2019-05-08T13:33:51.632149265Z"
/// }"##;
/// let payload: Message = payload.parse().unwrap();
/// println!("{payload}");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub id: Uuid,
    pub user: User,
    pub channel_id: Uuid,
    pub text: String,
    pub plain_text: String,
    pub embedded: Vec<EmbeddedInfo>,
    #[serde(with = "crate::payloads::serde::timestamp")]
    pub created_at: TimeStamp,
    #[serde(with = "crate::payloads::serde::timestamp")]
    pub updated_at: TimeStamp,
}

payload_impl! {Message}

/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_message_deleted.go#L14-L17)
///
/// ### Example
/// ```
/// use traq_bot_http::payloads::types::DeletedMessage;
/// let payload = r##"{
///     "id": "bc9106b3-f9b2-4eca-9ba1-72b39b40954e",
///     "channelId": "9aba50da-f605-4cd0-a428-5e4558cb911e"
/// }"##;
/// let payload: DeletedMessage = payload.parse().unwrap();
/// println!("{payload}");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeletedMessage {
    pub id: Uuid,
    pub channel_id: Uuid,
}

payload_impl! {DeletedMessage}

/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_direct_message_deleted.go#L14-L18)
///
/// ## Example
/// ```
/// use traq_bot_http::payloads::types::DeletedDirectMessage;
/// let payload = r##"{
///     "id": "2d7ff3f5-c313-4f4a-a9bb-0b5f84d2b6f8",
///     "userId": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45",
///     "channelId": "c5a5a697-3bad-4540-b2da-93dc88181d34"
/// }"##;
/// let payload: DeletedDirectMessage = payload.parse().unwrap();
/// println!("{payload}");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeletedDirectMessage {
    pub id: Uuid,
    pub user_id: Uuid,
    pub channel_id: Uuid,
}

payload_impl! {DeletedDirectMessage}

/// - [traQの定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/model/message_stamp.go#L9-L20)
///
/// ## Example
/// ```
/// use traq_bot_http::payloads::types::MessageStamp;
/// let payload = r##"{
///     "stampId": "b77fad4e-b63f-42a2-916c-5cfe5af3d8b9",
///     "userId": "b80551a5-2768-4d29-ad78-8e0e92330c8d",
///     "count": 24,
///     "createdAt": "2020-10-17T03:34:56.575099Z",
///     "updatedAt": "2020-10-17T03:35:34Z"
/// }"##;
/// let payload: MessageStamp = payload.parse().unwrap();
/// println!("{payload}");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageStamp {
    pub stamp_id: Uuid,
    pub user_id: Uuid,
    pub count: i32,
    #[serde(with = "crate::payloads::serde::timestamp")]
    pub created_at: TimeStamp,
    #[serde(with = "crate::payloads::serde::timestamp")]
    pub updated_at: TimeStamp,
}

payload_impl! {MessageStamp}

/// [traQの定義](https://github.com/traPtitech/traQ/blob/a1aaf12d089a9033461d0f1fcabb69a92873a3b1/service/bot/event/payload/common.go#L92-L95)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMember {
    pub group_id: Uuid,
    pub user_id: Uuid,
}

payload_impl! {GroupMember}

/// - [traQの定義](https://github.com/traPtitech/traQ/blob/a1aaf12d089a9033461d0f1fcabb69a92873a3b1/service/bot/event/payload/common.go#L104)
///
/// ## Example
/// ```
/// use traq_bot_http::payloads::types::UserGroupAdmin;
/// let payload = r#"{
///     "groupId": "f265bde2-04cc-4856-9008-3db1d953a539",
///     "userId": "8e6a088f-9274-42c0-bb20-cee7913d144b"
/// }"#;
/// let payload: UserGroupAdmin = payload.parse().unwrap();
/// println!("{payload}");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserGroupAdmin {
    pub group_id: Uuid,
    pub user_id: Uuid,
}

payload_impl! {UserGroupAdmin}

impl From<UserGroupAdmin> for GroupMember {
    fn from(admin: UserGroupAdmin) -> Self {
        Self {
            group_id: admin.group_id,
            user_id: admin.user_id,
        }
    }
}

impl From<GroupMember> for UserGroupAdmin {
    fn from(member: GroupMember) -> Self {
        Self {
            group_id: member.group_id,
            user_id: member.user_id,
        }
    }
}

/// - [traQの定義](https://github.com/traPtitech/traQ/blob/a1aaf12d089a9033461d0f1fcabb69a92873a3b1/service/bot/event/payload/common.go#L113-L116)
///
/// ## Example
/// ```
/// use traq_bot_http::payloads::types::UserGroupMember;
/// let payload = r##"{
///     "groupId": "f265bde2-04cc-4856-9008-3db1d953a539",
///     "userId": "8e6a088f-9274-42c0-bb20-cee7913d144b",
///     "role": ""
/// }"##;
/// let payload: UserGroupMember = payload.parse().unwrap();
/// println!("{payload}");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserGroupMember {
    pub group_id: Uuid,
    pub user_id: Uuid,
    pub role: String,
}

payload_impl! {UserGroupMember}

/// - [traQの定義](https://github.com/traPtitech/traQ/blob/a1aaf12d089a9033461d0f1fcabb69a92873a3b1/service/bot/event/payload/common.go#L128-L138)
///
/// ## Example
/// ```
/// use traq_bot_http::payloads::types::UserGroup;
/// let payload = r##"{
///     "id": "f265bde2-04cc-4856-9008-3db1d953a539",
///     "name": "fugafuga",
///     "description": "FUGA_FUGA",
///     "type": "ふがふが",
///     "icon": "81f6da0d-eaab-4c42-84ac-74f5111e1eaa",
///     "admins": [
///         {
///             "groupId": "f265bde2-04cc-4856-9008-3db1d953a539",
///             "userId": "8e6a088f-9274-42c0-bb20-cee7913d144b"
///         }
///     ],
///     "members": [
///         {
///             "groupId": "f265bde2-04cc-4856-9008-3db1d953a539",
///             "userId": "8e6a088f-9274-42c0-bb20-cee7913d144b",
///             "role": ""
///         }
///     ],
///     "createdAt": "2023-08-25T04:04:32.912312Z",
///     "updatedAt": "2023-08-25T04:04:32.912312Z"
/// }"##;
/// let payload: UserGroup = payload.parse().unwrap();
/// println!("{payload}");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserGroup {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub r#type: String,
    pub icon: Uuid,
    pub admins: Vec<UserGroupAdmin>,
    pub members: Vec<UserGroupMember>,
    #[serde(with = "crate::payloads::serde::timestamp")]
    pub created_at: TimeStamp,
    #[serde(with = "crate::payloads::serde::timestamp")]
    pub updated_at: TimeStamp,
}

payload_impl! {UserGroup}
