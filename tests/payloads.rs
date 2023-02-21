#[cfg(test)]
mod payload_tests {
    use traq_bot_http::payloads::*;

    fn read_file(filename: &str) -> String {
        use std::{fs::File, io::Read, path::Path};
        // https://doc.rust-lang.org/rust-by-example/std_misc/file/open.html
        let path = Path::new(filename);
        let mut file = File::open(path).expect("Failed to open file");
        let mut buf = String::new();
        file.read_to_string(&mut buf).expect("Failed to read file");
        buf
    }

    // PING
    #[test]
    fn ping_test() {
        let ping = read_file("testdata/system/ping.json");
        if let Err(err) = serde_json::from_str::<PingPayload>(&ping) {
            panic!("Failed to parse {ping}: {err}")
        }
    }

    // JOINED
    #[test]
    fn joined_test() {
        let data = read_file("testdata/system/joined.json");
        if let Err(err) = serde_json::from_str::<JoinedPayload>(&data) {
            panic!("Failed to parse {data}: {err}")
        }
    }

    // LEFT
    #[test]
    fn left_test() {
        let data = read_file("testdata/system/left.json");
        if let Err(err) = serde_json::from_str::<LeftPayload>(&data) {
            panic!("Failed to parse {data}: {err}")
        }
    }

    // MESSAGE_CREATED
    #[test]
    fn message_created_test() {
        let data = read_file("testdata/message/message_created.json");
        if let Err(err) = serde_json::from_str::<MessageCreatedPayload>(&data) {
            panic!("Failed to parse {data}: {err}")
        }
    }

    // MESSAGE_DELETED
    #[test]
    fn message_deleted_test() {
        let data = read_file("testdata/message/message_deleted.json");
        if let Err(err) = serde_json::from_str::<MessageDeletedPayload>(&data) {
            panic!("Failed to parse {data}: {err}")
        }
    }

    // MESSAGE_UPDATED
    #[test]
    fn message_updated_test() {
        let data = read_file("testdata/message/message_updated.json");
        if let Err(err) = serde_json::from_str::<MessageUpdatedPayload>(&data) {
            panic!("Failed to parse {data}: {err}")
        }
    }

    // DIRECT_MESSAGE_CREATED
    #[test]
    fn direct_message_created_test() {
        let data = read_file("testdata/message/direct_message_created.json");
        if let Err(err) = serde_json::from_str::<DirectMessageCreatedPayload>(&data) {
            panic!("Failed to parse {data}: {err}")
        }
    }

    // DIRECT_MESSAGE_DELETED
    #[test]
    fn direct_message_deleted_test() {
        let data = read_file("testdata/message/direct_message_deleted.json");
        if let Err(err) = serde_json::from_str::<DirectMessageDeletedPayload>(&data) {
            panic!("Failed to parse {data}: {err}")
        }
    }

    // DIRECT_MESSAGE_UPDATED
    #[test]
    fn direct_message_updated_test() {
        let data = read_file("testdata/message/direct_message_updated.json");
        if let Err(err) = serde_json::from_str::<DirectMessageUpdatedPayload>(&data) {
            panic!("Failed to parse {data}: {err}")
        }
    }

    // BOT_MESSAGE_STAMPS_UPDATED
    #[test]
    fn bot_message_stamps_updated_test() {
        let data = read_file("testdata/message/bot_message_stamps_updated.json");
        if let Err(err) = serde_json::from_str::<BotMessageStampsUpdatedPayload>(&data) {
            panic!("Failed to parse {data}: {err}")
        }
    }

    // CHANNEL_CREATED
    #[test]
    fn channel_created_test() {
        let data = read_file("testdata/channel/channel_created.json");
        if let Err(err) = serde_json::from_str::<ChannelCreatedPayload>(&data) {
            panic!("Failed to parse {data}: {err}")
        }
    }

    // CHANNEL_TOPIC_CHANGED
    #[test]
    fn chennel_topic_changed_test() {
        let data = read_file("testdata/channel/channel_topic_changed.json");
        if let Err(err) = serde_json::from_str::<ChannelTopicChangedPayload>(&data) {
            panic!("Failed to parse {data}: {err}")
        }
    }

    // USER_CREATED
    #[test]
    fn user_created_test() {
        let data = read_file("testdata/user/user_created.json");
        if let Err(err) = serde_json::from_str::<UserCreatedPayload>(&data) {
            panic!("Failed to parse {data}: {err}")
        }
    }

    // TAG_ADDED
    #[test]
    fn tag_added_test() {
        let data = read_file("testdata/tag/tag_added.json");
        if let Err(err) = serde_json::from_str::<TagAddedPayload>(&data) {
            panic!("Failed to parse {data}: {err}")
        }
    }

    // TAG_REMOVED
    #[test]
    fn tag_removed_test() {
        let data = read_file("testdata/tag/tag_removed.json");
        if let Err(err) = serde_json::from_str::<TagRemovedPayload>(&data) {
            panic!("Failed to parse {data}: {err}")
        }
    }
}
