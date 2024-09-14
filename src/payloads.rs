#![allow(clippy::module_name_repetitions)]

//! イベントペイロードの型定義

use crate::macros::{all_events, payloads_impl_for_kinds};

mod channel;
mod message;
pub(crate) mod serde;
mod stamp;
mod system;
mod tag;
pub mod types;
mod user;
mod user_group;

pub use channel::{ChannelCreatedPayload, ChannelTopicChangedPayload};
pub use message::{
    BotMessageStampsUpdatedPayload, DirectMessageCreatedPayload, DirectMessageDeletedPayload,
    DirectMessageUpdatedPayload, MessageCreatedPayload, MessageDeletedPayload,
    MessageUpdatedPayload,
};
pub use stamp::StampCreatedPayload;
pub use system::{JoinedPayload, LeftPayload, PingPayload};
pub use tag::{TagAddedPayload, TagRemovedPayload};
pub use user::UserCreatedPayload;
pub use user_group::{
    UserGroupAdminAddedPayload, UserGroupAdminRemovedPayload, UserGroupCreatedPayload,
    UserGroupDeletedPayload, UserGroupMemberAddedPayload, UserGroupMemberRemovedPayload,
    UserGroupMemberUpdatedPayload, UserGroupUpdatedPayload,
};

all_events! {payloads_impl_for_kinds}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::read_to_string;

    #[test]
    fn direct_message_created_convert() {
        let data = read_to_string("testdata/message/direct_message_created.json").unwrap();
        let payload: DirectMessageCreatedPayload = data.parse().unwrap();
        let payload: MessageCreatedPayload = payload.into();
        assert_eq!(payload, data.parse().unwrap());
    }

    #[test]
    fn message_created_convert() {
        let data = read_to_string("testdata/message/message_created.json").unwrap();
        let payload: MessageCreatedPayload = data.parse().unwrap();
        let payload: DirectMessageCreatedPayload = payload.into();
        assert_eq!(payload, data.parse().unwrap());
    }

    #[test]
    fn direct_message_updated_convert() {
        let data = read_to_string("testdata/message/direct_message_updated.json").unwrap();
        let payload: DirectMessageUpdatedPayload = data.parse().unwrap();
        let payload: MessageUpdatedPayload = payload.into();
        assert_eq!(payload, data.parse().unwrap());
    }

    #[test]
    fn message_updated_convert() {
        let data = read_to_string("testdata/message/message_updated.json").unwrap();
        let payload: MessageUpdatedPayload = data.parse().unwrap();
        let payload: DirectMessageUpdatedPayload = payload.into();
        assert_eq!(payload, data.parse().unwrap());
    }

    #[test]
    fn left_convert() {
        let data = read_to_string("testdata/system/left.json").unwrap();
        let payload: LeftPayload = data.parse().unwrap();
        let payload: JoinedPayload = payload.into();
        assert_eq!(payload, data.parse().unwrap());
    }

    #[test]
    fn joined_convert() {
        let data = read_to_string("testdata/system/joined.json").unwrap();
        let payload: JoinedPayload = data.parse().unwrap();
        let payload: LeftPayload = payload.into();
        assert_eq!(payload, data.parse().unwrap());
    }

    #[test]
    fn tag_removed_convert() {
        let data = read_to_string("testdata/tag/tag_removed.json").unwrap();
        let payload: TagRemovedPayload = data.parse().unwrap();
        let payload: TagAddedPayload = payload.into();
        assert_eq!(payload, data.parse().unwrap());
    }

    #[test]
    fn tag_added_convert() {
        let data = read_to_string("testdata/tag/tag_added.json").unwrap();
        let payload: TagAddedPayload = data.parse().unwrap();
        let payload: TagRemovedPayload = payload.into();
        assert_eq!(payload, data.parse().unwrap());
    }

    #[test]
    fn user_group_updated_convert() {
        let data = read_to_string("testdata/user-group/user_group_updated.json").unwrap();
        let payload: UserGroupUpdatedPayload = data.parse().unwrap();
        let payload: UserGroupDeletedPayload = payload.into();
        assert_eq!(payload, data.parse().unwrap());
    }

    #[test]
    fn user_group_deleted_convert() {
        let data = read_to_string("testdata/user-group/user_group_deleted.json").unwrap();
        let payload: UserGroupDeletedPayload = data.parse().unwrap();
        let payload: UserGroupUpdatedPayload = payload.into();
        assert_eq!(payload, data.parse().unwrap());
    }

    #[test]
    fn user_group_member_added_convert() {
        let data = read_to_string("testdata/user-group/user_group_member_added.json").unwrap();
        let payload_base: UserGroupMemberAddedPayload = data.parse().unwrap();
        let payload: UserGroupMemberUpdatedPayload = payload_base.clone().into();
        assert_eq!(payload, data.parse().unwrap());
        let payload: UserGroupMemberRemovedPayload = payload_base.clone().into();
        assert_eq!(payload, data.parse().unwrap());
        let payload: UserGroupAdminAddedPayload = payload_base.into();
        assert_eq!(payload, data.parse().unwrap());
    }

    #[test]
    fn user_group_member_updated_convert() {
        let data = read_to_string("testdata/user-group/user_group_member_updated.json").unwrap();
        let payload_base: UserGroupMemberUpdatedPayload = data.parse().unwrap();
        let payload: UserGroupMemberAddedPayload = payload_base.clone().into();
        assert_eq!(payload, data.parse().unwrap());
        let payload: UserGroupMemberRemovedPayload = payload_base.into();
        assert_eq!(payload, data.parse().unwrap());
    }

    #[test]
    fn user_group_member_removed_convert() {
        let data = read_to_string("testdata/user-group/user_group_member_removed.json").unwrap();
        let payload_base: UserGroupMemberRemovedPayload = data.parse().unwrap();
        let payload: UserGroupMemberAddedPayload = payload_base.clone().into();
        assert_eq!(payload, data.parse().unwrap());
        let payload: UserGroupMemberUpdatedPayload = payload_base.clone().into();
        assert_eq!(payload, data.parse().unwrap());
        let payload: UserGroupAdminRemovedPayload = payload_base.into();
        assert_eq!(payload, data.parse().unwrap());
    }

    #[test]
    fn user_group_admin_added_convert() {
        let data = read_to_string("testdata/user-group/user_group_admin_added.json").unwrap();
        let payload_base: UserGroupAdminAddedPayload = data.parse().unwrap();
        let payload: UserGroupAdminRemovedPayload = payload_base.clone().into();
        assert_eq!(payload, data.parse().unwrap());
        let payload: UserGroupMemberAddedPayload = payload_base.into();
        assert_eq!(payload, data.parse().unwrap());
    }

    #[test]
    fn user_group_admin_removed_convert() {
        let data = read_to_string("testdata/user-group/user_group_admin_removed.json").unwrap();
        let payload_base: UserGroupAdminRemovedPayload = data.parse().unwrap();
        let payload: UserGroupAdminAddedPayload = payload_base.clone().into();
        assert_eq!(payload, data.parse().unwrap());
        let payload: UserGroupMemberRemovedPayload = payload_base.into();
        assert_eq!(payload, data.parse().unwrap());
    }
}
