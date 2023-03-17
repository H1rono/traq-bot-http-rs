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

    #[test]
    fn ping_test() {
        let ping = read_file("testdata/system/ping.json");
        let payload = serde_json::from_str::<PingPayload>(&ping).unwrap();
        println!("{}", serde_json::to_string_pretty(&payload).unwrap());
    }

    #[test]
    fn joined_test() {
        let data = read_file("testdata/system/joined.json");
        serde_json::from_str::<JoinedPayload>(&data).unwrap();
    }

    #[test]
    fn left_test() {
        let data = read_file("testdata/system/left.json");
        serde_json::from_str::<LeftPayload>(&data).unwrap();
    }

    #[test]
    fn message_created_test() {
        let data = read_file("testdata/message/message_created.json");
        serde_json::from_str::<MessageCreatedPayload>(&data).unwrap();
    }

    #[test]
    fn message_deleted_test() {
        let data = read_file("testdata/message/message_deleted.json");
        serde_json::from_str::<MessageDeletedPayload>(&data).unwrap();
    }

    #[test]
    fn message_updated_test() {
        let data = read_file("testdata/message/message_updated.json");
        serde_json::from_str::<MessageUpdatedPayload>(&data).unwrap();
    }

    #[test]
    fn direct_message_created_test() {
        let data = read_file("testdata/message/direct_message_created.json");
        serde_json::from_str::<DirectMessageCreatedPayload>(&data).unwrap();
    }

    #[test]
    fn direct_message_deleted_test() {
        let data = read_file("testdata/message/direct_message_deleted.json");
        serde_json::from_str::<DirectMessageDeletedPayload>(&data).unwrap();
    }

    #[test]
    fn direct_message_updated_test() {
        let data = read_file("testdata/message/direct_message_updated.json");
        serde_json::from_str::<DirectMessageUpdatedPayload>(&data).unwrap();
    }

    #[test]
    fn bot_message_stamps_updated_test() {
        let data = read_file("testdata/message/bot_message_stamps_updated.json");
        serde_json::from_str::<BotMessageStampsUpdatedPayload>(&data).unwrap();
    }

    #[test]
    fn channel_created_test() {
        let data = read_file("testdata/channel/channel_created.json");
        serde_json::from_str::<ChannelCreatedPayload>(&data).unwrap();
    }

    #[test]
    fn chennel_topic_changed_test() {
        let data = read_file("testdata/channel/channel_topic_changed.json");
        serde_json::from_str::<ChannelTopicChangedPayload>(&data).unwrap();
    }

    #[test]
    fn user_created_test() {
        let data = read_file("testdata/user/user_created.json");
        serde_json::from_str::<UserCreatedPayload>(&data).unwrap();
    }

    #[test]
    fn tag_added_test() {
        let data = read_file("testdata/tag/tag_added.json");
        serde_json::from_str::<TagAddedPayload>(&data).unwrap();
    }

    #[test]
    fn tag_removed_test() {
        let data = read_file("testdata/tag/tag_removed.json");
        serde_json::from_str::<TagRemovedPayload>(&data).unwrap();
    }
}
