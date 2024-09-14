//! `enum Event`

use std::{fmt::Display, str::FromStr};

use crate::macros::{all_events, event_converts};
use crate::payloads::{
    BotMessageStampsUpdatedPayload, ChannelCreatedPayload, ChannelTopicChangedPayload,
    DirectMessageCreatedPayload, DirectMessageDeletedPayload, DirectMessageUpdatedPayload,
    JoinedPayload, LeftPayload, MessageCreatedPayload, MessageDeletedPayload,
    MessageUpdatedPayload, PingPayload, StampCreatedPayload, TagAddedPayload, TagRemovedPayload,
    UserCreatedPayload, UserGroupAdminAddedPayload, UserGroupAdminRemovedPayload,
    UserGroupCreatedPayload, UserGroupDeletedPayload, UserGroupMemberAddedPayload,
    UserGroupMemberRemovedPayload, UserGroupMemberUpdatedPayload, UserGroupUpdatedPayload,
};

/// イベント全てを網羅するenum ([non-exhaustive](https://doc.rust-lang.org/reference/attributes/type_system.html))
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
/// # fn main() -> Result<(), serde_json::Error> {
/// use traq_bot_http::Event;
/// use traq_bot_http::payloads::PingPayload;
/// let payload = r#"{
///     "eventTime": "2019-05-07T04:50:48.582586882Z"
/// }"#;
/// let payload = serde_json::from_str::<PingPayload>(payload)?;
/// let event = Event::Ping(payload);
/// # Ok(())
/// # }
/// ```
#[must_use]
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

all_events! {event_converts}

/// イベントの種類全てを網羅するenum ([non-exhaustive](https://doc.rust-lang.org/reference/attributes/type_system.html))
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
/// # fn main() -> Result<(), serde_json::Error> {
/// use traq_bot_http::{Event, EventKind};
/// use traq_bot_http::payloads::PingPayload;
/// let payload = r#"{
///     "eventTime": "2019-05-07T04:50:48.582586882Z"
/// }"#;
/// let payload = serde_json::from_str::<PingPayload>(payload)?;
/// let event: Event = payload.into();
/// assert_eq!(event.kind(), EventKind::Ping);
/// # Ok(())
/// # }
/// ```
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum EventKind {
    Ping,
    Joined,
    Left,
    MessageCreated,
    MessageDeleted,
    MessageUpdated,
    DirectMessageCreated,
    DirectMessageDeleted,
    DirectMessageUpdated,
    BotMessageStampsUpdated,
    ChannelCreated,
    ChannelTopicChanged,
    UserCreated,
    StampCreated,
    TagAdded,
    TagRemoved,
    UserGroupCreated,
    UserGroupUpdated,
    UserGroupDeleted,
    UserGroupMemberAdded,
    UserGroupMemberUpdated,
    UserGroupMemberRemoved,
    UserGroupAdminAdded,
    UserGroupAdminRemoved,
}

impl Event {
    pub fn kind(&self) -> EventKind {
        macro_rules! match_self_to_kind {
            ($($i:ident),*) => {
                match_event_to_kind!(self, $($i),*)
            };
        }

        use crate::macros::match_event_to_kind;

        all_events!(match_self_to_kind)
    }
}

impl Display for EventKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        macro_rules! match_self_to_str {
            ($($i:ident),*) => {
                match_event_kinds_to_str!(self, $($i),*)
            };
        }

        use crate::macros::match_event_kinds_to_str;

        let s = all_events!(match_self_to_str);
        write!(f, "{s}")
    }
}

impl FromStr for EventKind {
    type Err = crate::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        macro_rules! match_s_to_event_kinds {
            ($($i:ident),*) => {
                match_str_to_event_kinds!(s, $($i),*)
            };
        }

        use crate::macros::match_str_to_event_kinds;

        all_events!(match_s_to_event_kinds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::macros::{all_events, test_event_convert, test_event_to_kind};

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

    test_event_to_kind! {"system", Ping}

    test_event_to_kind! {"system", Joined}

    test_event_to_kind! {"system", Left}

    test_event_to_kind! {"message", MessageCreated}

    test_event_to_kind! {"message", MessageDeleted}

    test_event_to_kind! {"message", MessageUpdated}

    test_event_to_kind! {"message", DirectMessageCreated}

    test_event_to_kind! {"message", DirectMessageDeleted}

    test_event_to_kind! {"message", DirectMessageUpdated}

    test_event_to_kind! {"message", BotMessageStampsUpdated}

    test_event_to_kind! {"channel", ChannelCreated}

    test_event_to_kind! {"channel", ChannelTopicChanged}

    test_event_to_kind! {"user", UserCreated}

    test_event_to_kind! {"stamp", StampCreated}

    test_event_to_kind! {"tag", TagAdded}

    test_event_to_kind! {"tag", TagRemoved}

    test_event_to_kind! {"user-group", UserGroupCreated}

    test_event_to_kind! {"user-group", UserGroupUpdated}

    test_event_to_kind! {"user-group", UserGroupDeleted}

    test_event_to_kind! {"user-group", UserGroupMemberAdded}

    test_event_to_kind! {"user-group", UserGroupMemberUpdated}

    test_event_to_kind! {"user-group", UserGroupMemberRemoved}

    test_event_to_kind! {"user-group", UserGroupAdminAdded}

    test_event_to_kind! {"user-group", UserGroupAdminRemoved}

    macro_rules! tests_event_kind_from_str {
        ($($kind:ident),*) => {
            $( $crate::macros::test_event_kind_from_str! {$kind} )*
        };
    }

    all_events! {tests_event_kind_from_str}

    macro_rules! tests_event_kind_to_string {
        ($($kind:ident),*) => {
            $( $crate::macros::test_event_kind_to_string! {$kind} )*
        };
    }

    all_events! {tests_event_kind_to_string}
}
