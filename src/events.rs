//! `enum Event`

use super::payloads::{
    BotMessageStampsUpdatedPayload, ChannelCreatedPayload, ChannelTopicChangedPayload,
    DirectMessageCreatedPayload, DirectMessageDeletedPayload, DirectMessageUpdatedPayload,
    JoinedPayload, LeftPayload, MessageCreatedPayload, MessageDeletedPayload,
    MessageUpdatedPayload, PingPayload, StampCreatedPayload, TagAddedPayload, TagRemovedPayload,
    UserCreatedPayload, UserGroupAdminAddedPayload, UserGroupAdminRemovedPayload,
    UserGroupCreatedPayload, UserGroupDeletedPayload, UserGroupMemberAddedPayload,
    UserGroupMemberRemovedPayload, UserGroupMemberUpdatedPayload, UserGroupUpdatedPayload,
};
use crate::macros::event_converts;

/// イベント全てを網羅するenum
///
/// ## Variants
/// * `Ping` - ボットの接続確認
/// * `Joined` - チャンネルに参加した
/// * `Left` - チャンネルから退出した
/// * `MessageCreated` - メッセージが作成された
/// * `MessageDeleted` - メッセージが削除された
/// * `MessageUpdated` - メッセージが更新された
/// * `DirectMessageCreated` - ダイレクトメッセージが作成された
/// * `DirectMessageDeleted` - ダイレクトメッセージが削除された
/// * `DirectMessageUpdated` - ダイレクトメッセージが更新された
/// * `BotMessageStampsUpdated` - ボットのメッセージにスタンプが付けられた
/// * `ChannelCreated` - チャンネルが作成された
/// * `ChannelTopicChanged` - チャンネルのトピックが変更された
/// * `UserCreated` - ユーザーが作成された
/// * `StampCreated` - スタンプが作成された
/// * `TagAdded` - BOTにタグが追加された
/// * `TagRemoved` - BOTからタグが削除された
/// * `UserGroupCreated` - ユーザーグループが作成された
/// * `UserGroupUpdated` - ユーザーグループが更新された
/// * `UserGroupDeleted` - ユーザーグループが削除された
/// * `UserGroupMemberAdded` - ユーザーグループにメンバーが追加された
/// * `UserGroupMemberUpdated` - ユーザーグループのメンバーが更新された
/// * `UserGroupMemberRemoved` - ユーザーグループからメンバーが削除された
/// * `UserGroupAdminAdded` - ユーザーグループに管理者が追加された
/// * `UserGroupAdminRemoved` - ユーザーグループから管理者が削除された
///
/// ## Example
/// ```
/// use traq_bot_http::Event;
/// use traq_bot_http::payloads::PingPayload;
/// let payload = r#"{
///     "eventTime": "2019-05-07T04:50:48.582586882Z"
/// }"#;
/// let payload = serde_json::from_str::<PingPayload>(payload).unwrap();
/// let event = Event::Ping(payload);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Event {
    Ping(PingPayload),
    Joined(JoinedPayload),
    Left(LeftPayload),
    MessageCreated(MessageCreatedPayload),
    MessageDeleted(MessageDeletedPayload),
    MessageUpdated(MessageUpdatedPayload),
    DirectMessageCreated(DirectMessageCreatedPayload),
    DirectMessageDeleted(DirectMessageDeletedPayload),
    DirectMessageUpdated(DirectMessageUpdatedPayload),
    BotMessageStampsUpdated(BotMessageStampsUpdatedPayload),
    ChannelCreated(ChannelCreatedPayload),
    ChannelTopicChanged(ChannelTopicChangedPayload),
    UserCreated(UserCreatedPayload),
    StampCreated(StampCreatedPayload),
    TagAdded(TagAddedPayload),
    TagRemoved(TagRemovedPayload),
    UserGroupCreated(UserGroupCreatedPayload),
    UserGroupUpdated(UserGroupUpdatedPayload),
    UserGroupDeleted(UserGroupDeletedPayload),
    UserGroupMemberAdded(UserGroupMemberAddedPayload),
    UserGroupMemberUpdated(UserGroupMemberUpdatedPayload),
    UserGroupMemberRemoved(UserGroupMemberRemovedPayload),
    UserGroupAdminAdded(UserGroupAdminAddedPayload),
    UserGroupAdminRemoved(UserGroupAdminRemovedPayload),
}

// system
event_converts! {
    Ping, Joined, Left
}

// message
event_converts! {
    MessageCreated, MessageDeleted, MessageUpdated,
    DirectMessageCreated, DirectMessageDeleted, DirectMessageUpdated,
    BotMessageStampsUpdated
}

// channel
event_converts! {
    ChannelCreated, ChannelTopicChanged
}

// user
event_converts! {
    UserCreated
}

// stamp
event_converts! {
    StampCreated
}

// tag
event_converts! {
    TagAdded, TagRemoved
}

// user group
event_converts! {
    UserGroupCreated, UserGroupUpdated, UserGroupDeleted,
    UserGroupMemberAdded, UserGroupMemberUpdated, UserGroupMemberRemoved,
    UserGroupAdminAdded, UserGroupAdminRemoved
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::read_to_string;

    #[test]
    fn ping_convert() {
        let data = read_to_string("testdata/system/ping.json").unwrap();
        let payload: PingPayload = data.parse().unwrap();
        let event: Event = payload.into();
        assert_eq!(event, Event::Ping(data.parse().unwrap()));
    }

    #[test]
    fn joined_convert() {
        let data = read_to_string("testdata/system/joined.json").unwrap();
        let payload: JoinedPayload = data.parse().unwrap();
        let event: Event = payload.into();
        assert_eq!(event, Event::Joined(data.parse().unwrap()));
    }

    #[test]
    fn left_convert() {
        let data = read_to_string("testdata/system/left.json").unwrap();
        let payload: LeftPayload = data.parse().unwrap();
        let event: Event = payload.into();
        assert_eq!(event, Event::Left(data.parse().unwrap()));
    }

    #[test]
    fn message_created_convert() {
        let data = read_to_string("testdata/message/message_created.json").unwrap();
        let payload: MessageCreatedPayload = data.parse().unwrap();
        let event: Event = payload.into();
        assert_eq!(event, Event::MessageCreated(data.parse().unwrap()));
    }

    #[test]
    fn message_deleted_convert() {
        let data = read_to_string("testdata/message/message_deleted.json").unwrap();
        let payload: MessageDeletedPayload = data.parse().unwrap();
        let event: Event = payload.into();
        assert_eq!(event, Event::MessageDeleted(data.parse().unwrap()));
    }

    #[test]
    fn message_updated_convert() {
        let data = read_to_string("testdata/message/message_updated.json").unwrap();
        let payload: MessageUpdatedPayload = data.parse().unwrap();
        let event: Event = payload.into();
        assert_eq!(event, Event::MessageUpdated(data.parse().unwrap()));
    }

    #[test]
    fn direct_message_created_convert() {
        let data = read_to_string("testdata/message/direct_message_created.json").unwrap();
        let payload: DirectMessageCreatedPayload = data.parse().unwrap();
        let event: Event = payload.into();
        assert_eq!(event, Event::DirectMessageCreated(data.parse().unwrap()));
    }

    #[test]
    fn direct_message_deleted_convert() {
        let data = read_to_string("testdata/message/direct_message_deleted.json").unwrap();
        let payload: DirectMessageDeletedPayload = data.parse().unwrap();
        let event: Event = payload.into();
        assert_eq!(event, Event::DirectMessageDeleted(data.parse().unwrap()));
    }

    #[test]
    fn direct_message_updated_convert() {
        let data = read_to_string("testdata/message/direct_message_updated.json").unwrap();
        let payload: DirectMessageUpdatedPayload = data.parse().unwrap();
        let event: Event = payload.into();
        assert_eq!(event, Event::DirectMessageUpdated(data.parse().unwrap()));
    }

    #[test]
    fn bot_message_stamps_updated_convert() {
        let data = read_to_string("testdata/message/bot_message_stamps_updated.json").unwrap();
        let payload: BotMessageStampsUpdatedPayload = data.parse().unwrap();
        let event: Event = payload.into();
        assert_eq!(event, Event::BotMessageStampsUpdated(data.parse().unwrap()));
    }

    #[test]
    fn channel_created_convert() {
        let data = read_to_string("testdata/channel/channel_created.json").unwrap();
        let payload: ChannelCreatedPayload = data.parse().unwrap();
        let event: Event = payload.into();
        assert_eq!(event, Event::ChannelCreated(data.parse().unwrap()));
    }

    #[test]
    fn channel_topic_changed_convert() {
        let data = read_to_string("testdata/channel/channel_topic_changed.json").unwrap();
        let payload: ChannelTopicChangedPayload = data.parse().unwrap();
        let event: Event = payload.into();
        assert_eq!(event, Event::ChannelTopicChanged(data.parse().unwrap()));
    }

    #[test]
    fn user_created_convert() {
        let data = read_to_string("testdata/user/user_created.json").unwrap();
        let payload: UserCreatedPayload = data.parse().unwrap();
        let event: Event = payload.into();
        assert_eq!(event, Event::UserCreated(data.parse().unwrap()));
    }

    #[test]
    fn stamp_created_convert() {
        let data = read_to_string("testdata/stamp/stamp_created.json").unwrap();
        let payload: StampCreatedPayload = data.parse().unwrap();
        let event: Event = payload.into();
        assert_eq!(event, Event::StampCreated(data.parse().unwrap()));
    }

    #[test]
    fn tag_added_convert() {
        let data = read_to_string("testdata/tag/tag_added.json").unwrap();
        let payload: TagAddedPayload = data.parse().unwrap();
        let event: Event = payload.into();
        assert_eq!(event, Event::TagAdded(data.parse().unwrap()));
    }

    #[test]
    fn tag_removed_convert() {
        let data = read_to_string("testdata/tag/tag_removed.json").unwrap();
        let payload: TagRemovedPayload = data.parse().unwrap();
        let event: Event = payload.into();
        assert_eq!(event, Event::TagRemoved(data.parse().unwrap()));
    }
}
