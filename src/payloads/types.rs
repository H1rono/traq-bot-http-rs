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

/// [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/common.go#L69-L75)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "iconId")]
    pub icon_id: Uuid,
    pub bot: bool,
}

payload_impl! {User}

/// [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/common.go#L47-L55)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Channel {
    pub id: Uuid,
    pub name: String,
    pub path: String,
    #[serde(rename = "parentId")]
    pub parent_id: Uuid,
    pub creator: User,
    #[serde(rename = "createdAt", with = "crate::payloads::serde::timestamp")]
    pub created_at: TimeStamp,
    #[serde(rename = "updatedAt", with = "crate::payloads::serde::timestamp")]
    pub updated_at: TimeStamp,
}

payload_impl! {Channel}

/// [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/utils/message/embedded.go#L9-L14)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct EmbeddedInfo {
    pub raw: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub id: Uuid,
}

payload_impl! {EmbeddedInfo}

/// [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/common.go#L23-L32)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Message {
    pub id: Uuid,
    pub user: User,
    #[serde(rename = "channelId")]
    pub channel_id: Uuid,
    pub text: String,
    #[serde(rename = "plainText")]
    pub plain_text: String,
    pub embedded: Vec<EmbeddedInfo>,
    #[serde(rename = "createdAt", with = "crate::payloads::serde::timestamp")]
    pub created_at: TimeStamp,
    #[serde(rename = "updatedAt", with = "crate::payloads::serde::timestamp")]
    pub updated_at: TimeStamp,
}

payload_impl! {Message}

/// [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_message_deleted.go#L14-L17)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct DeletedMessage {
    pub id: Uuid,
    #[serde(rename = "channelId")]
    pub channel_id: Uuid,
}

payload_impl! {DeletedMessage}

/// [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_direct_message_deleted.go#L14-L18)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct DeletedDirectMessage {
    pub id: Uuid,
    #[serde(rename = "userId")]
    pub user_id: Uuid,
    #[serde(rename = "channelId")]
    pub channel_id: Uuid,
}

payload_impl! {DeletedDirectMessage}

/// [traQの定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/model/message_stamp.go#L9-L20)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct MessageStamp {
    #[serde(rename = "stampId")]
    pub stamp_id: Uuid,
    #[serde(rename = "userId")]
    pub user_id: Uuid,
    pub count: i32,
    #[serde(rename = "createdAt", with = "crate::payloads::serde::timestamp")]
    pub created_at: TimeStamp,
    #[serde(rename = "updatedAt", with = "crate::payloads::serde::timestamp")]
    pub updated_at: TimeStamp,
}

payload_impl! {MessageStamp}

/// [traQの定義](https://github.com/traPtitech/traQ/blob/a1aaf12d089a9033461d0f1fcabb69a92873a3b1/service/bot/event/payload/common.go#L92-L95)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct GroupMember {
    #[serde(rename = "groupId")]
    pub group_id: Uuid,
    #[serde(rename = "userId")]
    pub user_id: Uuid,
}

payload_impl! {GroupMember}

/// [traQの定義](https://github.com/traPtitech/traQ/blob/a1aaf12d089a9033461d0f1fcabb69a92873a3b1/service/bot/event/payload/common.go#L104)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserGroupAdmin {
    #[serde(rename = "groupId")]
    pub group_id: Uuid,
    #[serde(rename = "userId")]
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

/// [traQの定義](https://github.com/traPtitech/traQ/blob/a1aaf12d089a9033461d0f1fcabb69a92873a3b1/service/bot/event/payload/common.go#L113-L116)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserGroupMember {
    #[serde(rename = "groupId")]
    pub group_id: Uuid,
    #[serde(rename = "userId")]
    pub user_id: Uuid,
    pub role: String,
}

payload_impl! {UserGroupMember}

/// [traQの定義](https://github.com/traPtitech/traQ/blob/a1aaf12d089a9033461d0f1fcabb69a92873a3b1/service/bot/event/payload/common.go#L128-L138)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserGroup {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub icon: Uuid,
    pub admins: Vec<UserGroupAdmin>,
    pub members: Vec<UserGroupMember>,
    #[serde(rename = "createdAt", with = "crate::payloads::serde::timestamp")]
    pub created_at: TimeStamp,
    #[serde(rename = "updatedAt", with = "crate::payloads::serde::timestamp")]
    pub updated_at: TimeStamp,
}

payload_impl! {UserGroup}
