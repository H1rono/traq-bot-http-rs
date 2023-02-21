use serde::{Deserialize, Serialize};

/// [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/common.go#L69-L75)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "iconId")]
    pub icon_id: String,
    pub bot: bool,
}

/// [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/common.go#L47-L55)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "parentId")]
    pub parent_id: String,
    pub creator: User,
}

/// [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/utils/message/embedded.go#L9-L14)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct EmbeddedInfo {
    pub raw: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub id: String,
}

/// [traQの型定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/service/bot/event/payload/common.go#L23-L32)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Message {
    pub id: String,
    pub user: User,
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
    pub id: String,
    #[serde(rename = "channelId")]
    pub channel_id: String,
}

/// [traQの定義](https://github.com/traPtitech/traQ/blob/d2bc98f1e0e68f4acc371eb78e6a49a167446761/model/message_stamp.go#L9-L20)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct MessageStamp {
    #[serde(rename = "stampId")]
    stamp_id: String,
    #[serde(rename = "userId")]
    user_id: String,
    count: i32,
    #[serde(rename = "createdAt")]
    created_at: String,
    #[serde(rename = "updatedAt")]
    updated_at: String,
}
