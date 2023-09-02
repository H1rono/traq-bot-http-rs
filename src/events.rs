//! `enum Event`

use super::payloads::{
    BotMessageStampsUpdatedPayload, ChannelCreatedPayload, ChannelTopicChangedPayload,
    DirectMessageCreatedPayload, DirectMessageDeletedPayload, DirectMessageUpdatedPayload,
    JoinedPayload, LeftPayload, MessageCreatedPayload, MessageDeletedPayload,
    MessageUpdatedPayload, PingPayload, StampCreatedPayload, TagAddedPayload, TagRemovedPayload,
    UserCreatedPayload,
};

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
}

impl From<PingPayload> for Event {
    fn from(payload: PingPayload) -> Self {
        Self::Ping(payload)
    }
}

impl From<JoinedPayload> for Event {
    fn from(payload: JoinedPayload) -> Self {
        Self::Joined(payload)
    }
}

impl From<LeftPayload> for Event {
    fn from(payload: LeftPayload) -> Self {
        Self::Left(payload)
    }
}

impl From<MessageCreatedPayload> for Event {
    fn from(payload: MessageCreatedPayload) -> Self {
        Self::MessageCreated(payload)
    }
}

impl From<MessageDeletedPayload> for Event {
    fn from(payload: MessageDeletedPayload) -> Self {
        Self::MessageDeleted(payload)
    }
}

impl From<MessageUpdatedPayload> for Event {
    fn from(payload: MessageUpdatedPayload) -> Self {
        Self::MessageUpdated(payload)
    }
}

impl From<DirectMessageCreatedPayload> for Event {
    fn from(payload: DirectMessageCreatedPayload) -> Self {
        Self::DirectMessageCreated(payload)
    }
}

impl From<DirectMessageDeletedPayload> for Event {
    fn from(payload: DirectMessageDeletedPayload) -> Self {
        Self::DirectMessageDeleted(payload)
    }
}

impl From<DirectMessageUpdatedPayload> for Event {
    fn from(payload: DirectMessageUpdatedPayload) -> Self {
        Self::DirectMessageUpdated(payload)
    }
}

impl From<BotMessageStampsUpdatedPayload> for Event {
    fn from(payload: BotMessageStampsUpdatedPayload) -> Self {
        Self::BotMessageStampsUpdated(payload)
    }
}

impl From<ChannelCreatedPayload> for Event {
    fn from(payload: ChannelCreatedPayload) -> Self {
        Self::ChannelCreated(payload)
    }
}

impl From<ChannelTopicChangedPayload> for Event {
    fn from(payload: ChannelTopicChangedPayload) -> Self {
        Self::ChannelTopicChanged(payload)
    }
}

impl From<UserCreatedPayload> for Event {
    fn from(payload: UserCreatedPayload) -> Self {
        Self::UserCreated(payload)
    }
}

impl From<StampCreatedPayload> for Event {
    fn from(payload: StampCreatedPayload) -> Self {
        Self::StampCreated(payload)
    }
}

impl From<TagAddedPayload> for Event {
    fn from(payload: TagAddedPayload) -> Self {
        Self::TagAdded(payload)
    }
}

impl From<TagRemovedPayload> for Event {
    fn from(payload: TagRemovedPayload) -> Self {
        Self::TagRemoved(payload)
    }
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
