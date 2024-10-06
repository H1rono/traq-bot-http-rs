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
    /// ボットの接続確認
    Ping(PingPayload),
    /// チャンネルに参加した
    Joined(JoinedPayload),
    /// チャンネルから退出した
    Left(LeftPayload),
    /// メッセージが作成された
    MessageCreated(MessageCreatedPayload),
    /// メッセージが削除された
    MessageDeleted(MessageDeletedPayload),
    /// メッセージが更新された
    MessageUpdated(MessageUpdatedPayload),
    /// ダイレクトメッセージが作成された
    DirectMessageCreated(DirectMessageCreatedPayload),
    /// ダイレクトメッセージが削除された
    DirectMessageDeleted(DirectMessageDeletedPayload),
    /// ダイレクトメッセージが更新された
    DirectMessageUpdated(DirectMessageUpdatedPayload),
    /// ボットのメッセージにスタンプが付けられた
    BotMessageStampsUpdated(BotMessageStampsUpdatedPayload),
    /// チャンネルが作成された
    ChannelCreated(ChannelCreatedPayload),
    /// チャンネルのトピックが変更された
    ChannelTopicChanged(ChannelTopicChangedPayload),
    /// ユーザーが作成された
    UserCreated(UserCreatedPayload),
    /// スタンプが作成された
    StampCreated(StampCreatedPayload),
    /// BOTにタグが追加された
    TagAdded(TagAddedPayload),
    /// BOTからタグが削除された
    TagRemoved(TagRemovedPayload),
    /// ユーザーグループが作成された
    UserGroupCreated(UserGroupCreatedPayload),
    /// ユーザーグループが更新された
    UserGroupUpdated(UserGroupUpdatedPayload),
    /// ユーザーグループが削除された
    UserGroupDeleted(UserGroupDeletedPayload),
    /// ユーザーグループにメンバーが追加された
    UserGroupMemberAdded(UserGroupMemberAddedPayload),
    /// ユーザーグループのメンバーが更新された
    UserGroupMemberUpdated(UserGroupMemberUpdatedPayload),
    /// ユーザーグループからメンバーが削除された
    UserGroupMemberRemoved(UserGroupMemberRemovedPayload),
    /// ユーザーグループに管理者が追加された
    UserGroupAdminAdded(UserGroupAdminAddedPayload),
    /// ユーザーグループから管理者が削除された
    UserGroupAdminRemoved(UserGroupAdminRemovedPayload),
}

all_events! {event_converts}

/// イベントの種類全てを網羅するenum ([non-exhaustive](https://doc.rust-lang.org/reference/attributes/type_system.html))
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
    /// ボットの接続確認
    Ping,
    /// チャンネルに参加した
    Joined,
    /// チャンネルから退出した
    Left,
    /// メッセージが作成された
    MessageCreated,
    /// メッセージが削除された
    MessageDeleted,
    /// メッセージが更新された
    MessageUpdated,
    /// ダイレクトメッセージが作成された
    DirectMessageCreated,
    /// ダイレクトメッセージが削除された
    DirectMessageDeleted,
    /// ダイレクトメッセージが更新された
    DirectMessageUpdated,
    /// ボットのメッセージにスタンプが付けられた
    BotMessageStampsUpdated,
    /// チャンネルが作成された
    ChannelCreated,
    /// チャンネルのトピックが変更された
    ChannelTopicChanged,
    /// ユーザーが作成された
    UserCreated,
    /// スタンプが作成された
    StampCreated,
    /// BOTにタグが追加された
    TagAdded,
    /// BOTからタグが削除された
    TagRemoved,
    /// ユーザーグループが作成された
    UserGroupCreated,
    /// ユーザーグループが更新された
    UserGroupUpdated,
    /// ユーザーグループが削除された
    UserGroupDeleted,
    /// ユーザーグループにメンバーが追加された
    UserGroupMemberAdded,
    /// ユーザーグループのメンバーが更新された
    UserGroupMemberUpdated,
    /// ユーザーグループからメンバーが削除された
    UserGroupMemberRemoved,
    /// ユーザーグループに管理者が追加された
    UserGroupAdminAdded,
    /// ユーザーグループから管理者が削除された
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
    type Err = crate::Error;

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
