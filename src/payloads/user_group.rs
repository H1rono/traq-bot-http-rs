//! ユーザーグループ関連のイベントペイロード
//! ## types
//! - [`UserGroupCreatedPayload`](UserGroupCreatedPayload)
//! - [`UserGroupUpdatedPayload`](UserGroupUpdatedPayload)
//! - [`UserGroupDeletedPayload`](UserGroupDeletedPayload)
//! - [`UserGroupMemberAddedPayload`](UserGroupMemberAddedPayload)
//! - [`UserGroupMemberUpdatedPayload`](UserGroupMemberUpdatedPayload)
//! - [`UserGroupMemberRemovedPayload`](UserGroupMemberRemovedPayload)
//! - [`UserGroupAdminAddedPayload`](UserGroupAdminAddedPayload)
//! - [`UserGroupAdminRemovedPayload`](UserGroupAdminRemovedPayload)

use serde::{Deserialize, Serialize};

use super::types::{GroupMember, TimeStamp, UserGroup, Uuid};
use crate::macros::payload_impl;

/// `USER_GROUP_CREATED`ペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/a1aaf12d089a9033461d0f1fcabb69a92873a3b1/service/bot/event/payload/ev_user_group_created.go#L9-L13)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/user-group.md#user_group_created)
///
/// ## Example
/// ```
/// use traq_bot_http::payloads::UserGroupCreatedPayload;
/// let payload = r#"{
///     "eventTime": "2023-08-25T04:04:32.912312Z",
///     "group": {
///         "id": "f265bde2-04cc-4856-9008-3db1d953a539",
///         "name": "fugafuga",
///         "description": "FUGA_FUGA",
///         "type": "ふがふが",
///         "icon": "81f6da0d-eaab-4c42-84ac-74f5111e1eaa",
///         "admins": [
///             {
///                 "groupId": "f265bde2-04cc-4856-9008-3db1d953a539",
///                 "userId": "8e6a088f-9274-42c0-bb20-cee7913d144b"
///             }
///         ],
///         "members": [
///             {
///                 "groupId": "f265bde2-04cc-4856-9008-3db1d953a539",
///                 "userId": "8e6a088f-9274-42c0-bb20-cee7913d144b",
///                 "role": ""
///             }
///         ],
///         "createdAt": "2023-08-25T04:04:32.912312Z",
///         "updatedAt": "2023-08-25T04:04:32.912312Z"
///     }
/// }"#;
/// let payload: UserGroupCreatedPayload = payload.parse().unwrap();
/// println!("{payload}");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserGroupCreatedPayload {
    #[serde(with = "crate::payloads::serde::timestamp")]
    pub event_time: TimeStamp,
    pub group: UserGroup,
}

payload_impl! {UserGroupCreatedPayload}

/// `USER_GROUP_UPDATED`ペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/a1aaf12d089a9033461d0f1fcabb69a92873a3b1/service/bot/event/payload/ev_user_group_updated.go#L10-L13)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/user-group.md#user_group_updated)
///
/// ## Example
/// ```
/// use traq_bot_http::payloads::UserGroupUpdatedPayload;
/// let payload = r#"{
///     "eventTime": "2023-08-25T04:04:32.962264Z",
///     "groupId": "f265bde2-04cc-4856-9008-3db1d953a539"
/// }"#;
/// let payload: UserGroupUpdatedPayload = payload.parse().unwrap();
/// println!("{payload}");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserGroupUpdatedPayload {
    #[serde(with = "crate::payloads::serde::timestamp")]
    pub event_time: TimeStamp,
    pub group_id: Uuid,
}

payload_impl! {UserGroupUpdatedPayload}

impl From<UserGroupDeletedPayload> for UserGroupUpdatedPayload {
    fn from(payload: UserGroupDeletedPayload) -> Self {
        Self {
            event_time: payload.event_time,
            group_id: payload.group_id,
        }
    }
}

/// `USER_GROUP_DELETED`ペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/a1aaf12d089a9033461d0f1fcabb69a92873a3b1/service/bot/event/payload/ev_user_group_deleted.go#L11-L14)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/user-group.md#user_group_deleted)
///
/// ## Example
/// ```
/// use traq_bot_http::payloads::UserGroupDeletedPayload;
/// let payload = r#"{
///     "eventTime": "2023-08-25T06:40:35.971142Z",
///     "groupId": "f265bde2-04cc-4856-9008-3db1d953a539"
/// }"#;
/// let payload: UserGroupDeletedPayload = payload.parse().unwrap();
/// println!("{payload}");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserGroupDeletedPayload {
    #[serde(with = "crate::payloads::serde::timestamp")]
    pub event_time: TimeStamp,
    pub group_id: Uuid,
}

payload_impl! {UserGroupDeletedPayload}

impl From<UserGroupUpdatedPayload> for UserGroupDeletedPayload {
    fn from(payload: UserGroupUpdatedPayload) -> Self {
        Self {
            event_time: payload.event_time,
            group_id: payload.group_id,
        }
    }
}

/// `USER_GROUP_MEMBER_ADDED`ペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/a1aaf12d089a9033461d0f1fcabb69a92873a3b1/service/bot/event/payload/ev_user_group_member_added.go#L9-L12)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/user-group.md#user_group_member_added)
///
/// ## Example
/// ```
/// use traq_bot_http::payloads::UserGroupMemberAddedPayload;
/// let payload = r#"{
///     "eventTime": "2023-08-25T04:04:32.962264Z",
///     "groupMember": {
///         "groupId": "f265bde2-04cc-4856-9008-3db1d953a539",
///         "userId": "8e6a088f-9274-42c0-bb20-cee7913d144b"
///     }
/// }"#;
/// let payload: UserGroupMemberAddedPayload = payload.parse().unwrap();
/// println!("{payload}");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserGroupMemberAddedPayload {
    #[serde(with = "crate::payloads::serde::timestamp")]
    pub event_time: TimeStamp,
    pub group_member: GroupMember,
}

payload_impl! {UserGroupMemberAddedPayload}

impl From<UserGroupMemberUpdatedPayload> for UserGroupMemberAddedPayload {
    fn from(payload: UserGroupMemberUpdatedPayload) -> Self {
        Self {
            event_time: payload.event_time,
            group_member: payload.group_member,
        }
    }
}

impl From<UserGroupMemberRemovedPayload> for UserGroupMemberAddedPayload {
    fn from(payload: UserGroupMemberRemovedPayload) -> Self {
        Self {
            event_time: payload.event_time,
            group_member: payload.group_member,
        }
    }
}

impl From<UserGroupAdminAddedPayload> for UserGroupMemberAddedPayload {
    fn from(payload: UserGroupAdminAddedPayload) -> Self {
        Self {
            event_time: payload.event_time,
            group_member: payload.group_member,
        }
    }
}

/// `USER_GROUP_MEMBER_UPDATED`ペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/a1aaf12d089a9033461d0f1fcabb69a92873a3b1/service/bot/event/payload/ev_user_group_member_updated.go#L9-L12)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/user-group.md#user_group_member_updated)
///
/// ## Example
/// ```
/// use traq_bot_http::payloads::UserGroupMemberUpdatedPayload;
/// let payload = r#"{
///     "eventTime": "2023-08-25T04:04:32.962264Z",
///     "groupMember": {
///         "groupId": "f265bde2-04cc-4856-9008-3db1d953a539",
///         "userId": "8e6a088f-9274-42c0-bb20-cee7913d144b"
///     }
/// }"#;
/// let payload: UserGroupMemberUpdatedPayload = payload.parse().unwrap();
/// println!("{payload}");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserGroupMemberUpdatedPayload {
    #[serde(with = "crate::payloads::serde::timestamp")]
    pub event_time: TimeStamp,
    pub group_member: GroupMember,
}

payload_impl! {UserGroupMemberUpdatedPayload}

impl From<UserGroupMemberAddedPayload> for UserGroupMemberUpdatedPayload {
    fn from(payload: UserGroupMemberAddedPayload) -> Self {
        Self {
            event_time: payload.event_time,
            group_member: payload.group_member,
        }
    }
}

impl From<UserGroupMemberRemovedPayload> for UserGroupMemberUpdatedPayload {
    fn from(payload: UserGroupMemberRemovedPayload) -> Self {
        Self {
            event_time: payload.event_time,
            group_member: payload.group_member,
        }
    }
}

/// `USER_GROUP_MEMBER_REMOVED`ペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/a1aaf12d089a9033461d0f1fcabb69a92873a3b1/service/bot/event/payload/ev_user_group_member_removed.go#L9-L12)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/user-group.md#user_group_member_removed)
///
/// ## Example
/// ```
/// use traq_bot_http::payloads::UserGroupMemberRemovedPayload;
/// let payload = r#"{
///     "eventTime": "2023-08-25T04:04:32.962264Z",
///     "groupMember": {
///         "groupId": "f265bde2-04cc-4856-9008-3db1d953a539",
///         "userId": "8e6a088f-9274-42c0-bb20-cee7913d144b"
///     }
/// }"#;
/// let payload: UserGroupMemberRemovedPayload = payload.parse().unwrap();
/// println!("{payload}");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserGroupMemberRemovedPayload {
    #[serde(with = "crate::payloads::serde::timestamp")]
    pub event_time: TimeStamp,
    pub group_member: GroupMember,
}

payload_impl! {UserGroupMemberRemovedPayload}

impl From<UserGroupMemberAddedPayload> for UserGroupMemberRemovedPayload {
    fn from(payload: UserGroupMemberAddedPayload) -> Self {
        Self {
            event_time: payload.event_time,
            group_member: payload.group_member,
        }
    }
}

impl From<UserGroupMemberUpdatedPayload> for UserGroupMemberRemovedPayload {
    fn from(payload: UserGroupMemberUpdatedPayload) -> Self {
        Self {
            event_time: payload.event_time,
            group_member: payload.group_member,
        }
    }
}

impl From<UserGroupAdminRemovedPayload> for UserGroupMemberRemovedPayload {
    fn from(payload: UserGroupAdminRemovedPayload) -> Self {
        Self {
            event_time: payload.event_time,
            group_member: payload.group_member,
        }
    }
}

/// `USER_GROUP_ADMIN_ADDED`ペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/a1aaf12d089a9033461d0f1fcabb69a92873a3b1/service/bot/event/payload/ev_user_group_admin_added.go#L9-L12)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/user-group.md#user_group_admin_added)
///
/// ## Example
/// ```
/// use traq_bot_http::payloads::UserGroupAdminAddedPayload;
/// let payload = r#"{
///     "eventTime": "2023-08-25T04:04:32.962264Z",
///     "groupMember": {
///         "groupId": "f265bde2-04cc-4856-9008-3db1d953a539",
///         "userId": "8e6a088f-9274-42c0-bb20-cee7913d144b"
///     }
/// }"#;
/// let payload: UserGroupAdminAddedPayload = payload.parse().unwrap();
/// println!("{payload}");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserGroupAdminAddedPayload {
    #[serde(with = "crate::payloads::serde::timestamp")]
    pub event_time: TimeStamp,
    pub group_member: GroupMember,
}

payload_impl! {UserGroupAdminAddedPayload}

impl From<UserGroupAdminRemovedPayload> for UserGroupAdminAddedPayload {
    fn from(payload: UserGroupAdminRemovedPayload) -> Self {
        Self {
            event_time: payload.event_time,
            group_member: payload.group_member,
        }
    }
}

impl From<UserGroupMemberAddedPayload> for UserGroupAdminAddedPayload {
    fn from(payload: UserGroupMemberAddedPayload) -> Self {
        Self {
            event_time: payload.event_time,
            group_member: payload.group_member,
        }
    }
}

/// `USER_GROUP_ADMIN_REMOVED`ペイロード
/// - [traQの型定義](https://github.com/traPtitech/traQ/blob/a1aaf12d089a9033461d0f1fcabb69a92873a3b1/service/bot/event/payload/ev_user_group_admin_removed.go#L9-L12)
/// - [traQ-bot-consoleのリファレンス](https://github.com/traPtitech/traQ-bot-console/blob/dev/src/docs/bot/events/user-group.md#user_group_admin_removed)
///
/// ## Example
/// ```
/// use traq_bot_http::payloads::UserGroupAdminRemovedPayload;
/// let payload = r#"{
///     "eventTime": "2023-08-25T04:04:32.962264Z",
///     "groupMember": {
///         "groupId": "f265bde2-04cc-4856-9008-3db1d953a539",
///         "userId": "8e6a088f-9274-42c0-bb20-cee7913d144b"
///     }
/// }"#;
/// let payload: UserGroupAdminRemovedPayload = payload.parse().unwrap();
/// println!("{payload}");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserGroupAdminRemovedPayload {
    #[serde(with = "crate::payloads::serde::timestamp")]
    pub event_time: TimeStamp,
    pub group_member: GroupMember,
}

payload_impl! {UserGroupAdminRemovedPayload}

impl From<UserGroupAdminAddedPayload> for UserGroupAdminRemovedPayload {
    fn from(payload: UserGroupAdminAddedPayload) -> Self {
        Self {
            event_time: payload.event_time,
            group_member: payload.group_member,
        }
    }
}

impl From<UserGroupMemberRemovedPayload> for UserGroupAdminRemovedPayload {
    fn from(payload: UserGroupMemberRemovedPayload) -> Self {
        Self {
            event_time: payload.event_time,
            group_member: payload.group_member,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::payloads::types::{UserGroupAdmin, UserGroupMember};
    use crate::test_utils::{timestamp, uuid};

    use std::{fs::read_to_string, vec};

    #[test]
    fn user_group_created_test() {
        let data = read_to_string("testdata/user-group/user_group_created.json").unwrap();
        let payload: UserGroupCreatedPayload = data.parse().unwrap();
        assert_eq!(
            payload,
            UserGroupCreatedPayload {
                event_time: timestamp("2023-08-25T04:04:32.912312Z"),
                group: UserGroup {
                    id: uuid("f265bde2-04cc-4856-9008-3db1d953a539"),
                    name: "fugafuga".to_string(),
                    description: "FUGA_FUGA".to_string(),
                    r#type: "ふがふが".to_string(),
                    icon: uuid("81f6da0d-eaab-4c42-84ac-74f5111e1eaa"),
                    admins: vec![UserGroupAdmin {
                        group_id: uuid("f265bde2-04cc-4856-9008-3db1d953a539"),
                        user_id: uuid("8e6a088f-9274-42c0-bb20-cee7913d144b"),
                    }],
                    members: vec![UserGroupMember {
                        group_id: uuid("f265bde2-04cc-4856-9008-3db1d953a539"),
                        user_id: uuid("8e6a088f-9274-42c0-bb20-cee7913d144b"),
                        role: "".to_string(),
                    }],
                    created_at: timestamp("2023-08-25T04:04:32.912312Z"),
                    updated_at: timestamp("2023-08-25T04:04:32.912312Z")
                }
            }
        );
    }

    #[test]
    fn user_group_updated_test() {
        let data = read_to_string("testdata/user-group/user_group_updated.json").unwrap();
        let payload: UserGroupUpdatedPayload = data.parse().unwrap();
        assert_eq!(
            payload,
            UserGroupUpdatedPayload {
                event_time: timestamp("2023-08-25T04:04:32.962264Z"),
                group_id: uuid("f265bde2-04cc-4856-9008-3db1d953a539"),
            }
        );
    }

    #[test]
    fn user_group_deleted_test() {
        let data = read_to_string("testdata/user-group/user_group_deleted.json").unwrap();
        let payload: UserGroupDeletedPayload = data.parse().unwrap();
        assert_eq!(
            payload,
            UserGroupDeletedPayload {
                event_time: timestamp("2023-08-25T06:40:35.971142Z"),
                group_id: uuid("f265bde2-04cc-4856-9008-3db1d953a539"),
            }
        );
    }

    #[test]
    fn user_group_member_added_test() {
        let data = read_to_string("testdata/user-group/user_group_member_added.json").unwrap();
        let payload: UserGroupMemberAddedPayload = data.parse().unwrap();
        assert_eq!(
            payload,
            UserGroupMemberAddedPayload {
                event_time: timestamp("2023-08-25T04:04:32.962264Z"),
                group_member: GroupMember {
                    group_id: uuid("f265bde2-04cc-4856-9008-3db1d953a539"),
                    user_id: uuid("8e6a088f-9274-42c0-bb20-cee7913d144b"),
                },
            }
        );
    }

    #[test]
    fn user_group_member_updated_test() {
        let data = read_to_string("testdata/user-group/user_group_member_updated.json").unwrap();
        let payload: UserGroupMemberUpdatedPayload = data.parse().unwrap();
        assert_eq!(
            payload,
            UserGroupMemberUpdatedPayload {
                event_time: timestamp("2023-08-25T04:04:32.962264Z"),
                group_member: GroupMember {
                    group_id: uuid("f265bde2-04cc-4856-9008-3db1d953a539"),
                    user_id: uuid("8e6a088f-9274-42c0-bb20-cee7913d144b"),
                },
            }
        );
    }

    #[test]
    fn user_group_member_removed_test() {
        let data = read_to_string("testdata/user-group/user_group_member_removed.json").unwrap();
        let payload: UserGroupMemberRemovedPayload = data.parse().unwrap();
        assert_eq!(
            payload,
            UserGroupMemberRemovedPayload {
                event_time: timestamp("2023-08-25T04:04:32.962264Z"),
                group_member: GroupMember {
                    group_id: uuid("f265bde2-04cc-4856-9008-3db1d953a539"),
                    user_id: uuid("8e6a088f-9274-42c0-bb20-cee7913d144b"),
                },
            }
        );
    }

    #[test]
    fn user_group_admin_added_test() {
        let data = read_to_string("testdata/user-group/user_group_admin_added.json").unwrap();
        let payload: UserGroupAdminAddedPayload = data.parse().unwrap();
        assert_eq!(
            payload,
            UserGroupAdminAddedPayload {
                event_time: timestamp("2023-08-25T04:04:32.962264Z"),
                group_member: GroupMember {
                    group_id: uuid("f265bde2-04cc-4856-9008-3db1d953a539"),
                    user_id: uuid("8e6a088f-9274-42c0-bb20-cee7913d144b"),
                },
            }
        );
    }

    #[test]
    fn user_group_admin_removed_test() {
        let data = read_to_string("testdata/user-group/user_group_admin_removed.json").unwrap();
        let payload: UserGroupAdminRemovedPayload = data.parse().unwrap();
        assert_eq!(
            payload,
            UserGroupAdminRemovedPayload {
                event_time: timestamp("2023-08-25T04:04:32.962264Z"),
                group_member: GroupMember {
                    group_id: uuid("f265bde2-04cc-4856-9008-3db1d953a539"),
                    user_id: uuid("8e6a088f-9274-42c0-bb20-cee7913d144b"),
                },
            }
        );
    }
}
