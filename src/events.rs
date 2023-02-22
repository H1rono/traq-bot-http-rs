use super::payloads::{
    BotMessageStampsUpdatedPayload, ChannelCreatedPayload, ChannelTopicChangedPayload,
    DirectMessageCreatedPayload, DirectMessageDeletedPayload, DirectMessageUpdatedPayload,
    JoinedPayload, LeftPayload, MessageCreatedPayload, MessageDeletedPayload,
    MessageUpdatedPayload, PingPayload, StampCreatedPayload, TagAddedPayload, TagRemovedPayload,
    UserCreatedPayload,
};

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
