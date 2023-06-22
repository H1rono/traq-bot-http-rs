//! イベントペイロード内部で使われる型

use serde::{Deserialize, Serialize};

#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};

#[cfg(not(feature = "chrono"))]
#[cfg(feature = "time")]
use time::OffsetDateTime;

#[cfg(feature = "chrono")]
pub type TimeStamp = DateTime<Utc>;
#[cfg(not(feature = "chrono"))]
#[cfg(feature = "time")]
pub type TimeStamp = OffsetDateTime;
#[cfg(not(feature = "chrono"))]
#[cfg(not(feature = "time"))]
pub type TimeStamp = String;

#[cfg(feature = "uuid")]
pub type Uuid = uuid::Uuid;
#[cfg(not(feature = "uuid"))]
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

/// [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/utils/message/embedded.go#L9-L14)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct EmbeddedInfo {
    pub raw: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub id: Uuid,
}

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

/// [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_message_deleted.go#L14-L17)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct DeletedMessage {
    pub id: Uuid,
    #[serde(rename = "channelId")]
    pub channel_id: Uuid,
}

/// [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_direct_message_deleted.go#L14-L18)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct DeletedDirectMessage {
    pub id: Uuid,
    #[serde(rename = "userId")]
    pub user_id: Uuid,
    #[serde(rename = "channelId")]
    pub channel_id: Uuid,
}

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
