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

    use crate::macros::test_event_convert;

    test_event_convert! {"system", Ping}

    test_event_convert! {"system", Joined}

    test_event_convert! {"system", Left}

    test_event_convert! {"message", MessageCreated}

    test_event_convert! {"message", MessageDeleted}

    test_event_convert! {"message", MessageUpdated}

    test_event_convert! {"message", DirectMessageCreated}

    test_event_convert! {"message", DirectMessageDeleted}

    test_event_convert! {"message", DirectMessageUpdated}

    test_event_convert! {"message", BotMessageStampsUpdated}

    test_event_convert! {"channel", ChannelCreated}

    test_event_convert! {"channel", ChannelTopicChanged}

    test_event_convert! {"user", UserCreated}

    test_event_convert! {"stamp", StampCreated}

    test_event_convert! {"tag", TagAdded}

    test_event_convert! {"tag", TagRemoved}

    test_event_convert! {"user-group", UserGroupCreated}

    test_event_convert! {"user-group", UserGroupUpdated}

    test_event_convert! {"user-group", UserGroupDeleted}

    test_event_convert! {"user-group", UserGroupMemberAdded}

    test_event_convert! {"user-group", UserGroupMemberUpdated}

    test_event_convert! {"user-group", UserGroupMemberRemoved}

    test_event_convert! {"user-group", UserGroupAdminAdded}

    test_event_convert! {"user-group", UserGroupAdminRemoved}
}
