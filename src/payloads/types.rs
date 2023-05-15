//! イベントペイロード内部で使われる型

use serde::{Deserialize, Serialize};
#[cfg(feature = "time")]
use time::OffsetDateTime;

/// [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/common.go#L69-L75)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct User {
    #[cfg(feature = "uuid")]
    pub id: uuid::Uuid,
    #[cfg(not(feature = "uuid"))]
    pub id: String,
    pub name: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[cfg(feature = "uuid")]
    #[serde(rename = "iconId")]
    pub icon_id: uuid::Uuid,
    #[cfg(not(feature = "uuid"))]
    #[serde(rename = "iconId")]
    pub icon_id: String,
    pub bot: bool,
}

/// [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/common.go#L47-L55)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Channel {
    #[cfg(feature = "uuid")]
    pub id: uuid::Uuid,
    #[cfg(not(feature = "uuid"))]
    pub id: String,
    pub name: String,
    pub path: String,
    #[cfg(feature = "uuid")]
    #[serde(rename = "parentId")]
    pub parent_id: uuid::Uuid,
    #[cfg(not(feature = "uuid"))]
    #[serde(rename = "parentId")]
    pub parent_id: String,
    pub creator: User,
    #[cfg(feature = "time")]
    #[serde(rename = "createdAt", with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[cfg(not(feature = "time"))]
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[cfg(feature = "time")]
    #[serde(rename = "updatedAt", with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    #[cfg(not(feature = "time"))]
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

/// [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/utils/message/embedded.go#L9-L14)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct EmbeddedInfo {
    pub raw: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[cfg(feature = "uuid")]
    pub id: uuid::Uuid,
    #[cfg(not(feature = "uuid"))]
    pub id: String,
}

/// [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/common.go#L23-L32)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Message {
    #[cfg(feature = "uuid")]
    pub id: uuid::Uuid,
    #[cfg(not(feature = "uuid"))]
    pub id: String,
    pub user: User,
    #[cfg(feature = "uuid")]
    #[serde(rename = "channelId")]
    pub channel_id: uuid::Uuid,
    #[cfg(not(feature = "uuid"))]
    #[serde(rename = "channelId")]
    pub channel_id: String,
    pub text: String,
    #[serde(rename = "plainText")]
    pub plain_text: String,
    pub embedded: Vec<EmbeddedInfo>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

/// [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_message_deleted.go#L14-L17)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct DeletedMessage {
    #[cfg(feature = "uuid")]
    pub id: uuid::Uuid,
    #[cfg(not(feature = "uuid"))]
    pub id: String,
    #[cfg(feature = "uuid")]
    #[serde(rename = "channelId")]
    pub channel_id: uuid::Uuid,
    #[cfg(not(feature = "uuid"))]
    #[serde(rename = "channelId")]
    pub channel_id: String,
}

/// [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/ev_direct_message_deleted.go#L14-L18)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct DeletedDirectMessage {
    #[cfg(feature = "uuid")]
    pub id: uuid::Uuid,
    #[cfg(not(feature = "uuid"))]
    pub id: String,
    #[cfg(feature = "uuid")]
    #[serde(rename = "userId")]
    pub user_id: uuid::Uuid,
    #[cfg(not(feature = "uuid"))]
    #[serde(rename = "userId")]
    pub user_id: String,
    #[cfg(feature = "uuid")]
    #[serde(rename = "channelId")]
    pub channel_id: uuid::Uuid,
    #[cfg(not(feature = "uuid"))]
    #[serde(rename = "channelId")]
    pub channel_id: String,
}

/// [traQの定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/model/message_stamp.go#L9-L20)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct MessageStamp {
    #[cfg(feature = "uuid")]
    #[serde(rename = "stampId")]
    pub stamp_id: uuid::Uuid,
    #[cfg(not(feature = "uuid"))]
    #[serde(rename = "stampId")]
    pub stamp_id: String,
    #[cfg(feature = "uuid")]
    #[serde(rename = "userId")]
    pub user_id: uuid::Uuid,
    #[cfg(not(feature = "uuid"))]
    #[serde(rename = "userId")]
    pub user_id: String,
    pub count: i32,
    #[cfg(feature = "time")]
    #[serde(rename = "createdAt", with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[cfg(not(feature = "time"))]
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[cfg(feature = "time")]
    #[serde(rename = "updatedAt", with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    #[cfg(not(feature = "time"))]
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}
