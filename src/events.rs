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
