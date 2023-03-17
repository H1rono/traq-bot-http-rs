#[cfg(test)]
mod parser_tests {
    const VERIFICATION_TOKEN: &str = "traqbotverificationtoken";

    use http::header::{HeaderMap, CONTENT_TYPE};
    use traq_bot_http::{payloads::*, Event, ParseError, RequestParser};

    fn make_parser() -> RequestParser {
        RequestParser::new(VERIFICATION_TOKEN)
    }

    #[test]
    fn fail_tests() {
        let parser = make_parser();
        let mut headers = HeaderMap::new();
        assert_eq!(
            parser.parse(headers.clone(), b""),
            Err(ParseError::ContentTypeNotFound)
        );
        headers.insert(CONTENT_TYPE, "text/plain".parse().unwrap());
        assert_eq!(
            parser.parse(headers.clone(), b""),
            Err(ParseError::ContentTypeMismatch)
        );
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        assert_eq!(
            parser.parse(headers.clone(), b""),
            Err(ParseError::BotTokenNotFound)
        );
        headers.insert("X-TRAQ-BOT-TOKEN", "invalid　token".parse().unwrap());
        assert_eq!(
            parser.parse(headers.clone(), b""),
            Err(ParseError::ReadBotTokenFailed)
        );
        headers.insert("X-TRAQ-BOT-TOKEN", "invalid_token".parse().unwrap());
        assert_eq!(
            parser.parse(headers.clone(), b""),
            Err(ParseError::BotTokenMismatch)
        );
        headers.insert(
            "X-TRAQ-BOT-TOKEN",
            "traqbotverificationtoken".parse().unwrap(),
        );
        assert_eq!(
            parser.parse(headers.clone(), b""),
            Err(ParseError::BotEventNotFound)
        );
        headers.insert("X-TRAQ-BOT-EVENT", "invalid　event".parse().unwrap());
        assert_eq!(
            parser.parse(headers.clone(), b""),
            Err(ParseError::ReadBotEventFailed)
        );
        headers.insert("X-TRAQ-BOT-EVENT", "invalid_event".parse().unwrap());
        assert_eq!(
            parser.parse(headers.clone(), b""),
            Err(ParseError::BotEventMismatch)
        );
        headers.insert("X-TRAQ-BOT-EVENT", "PING".parse().unwrap());
        assert_eq!(
            parser.parse(headers.clone(), b""),
            Err(ParseError::ParseBodyFailed)
        );
    }

    #[test]
    fn err_fmt() {
        use ParseError::*;
        let pairs = [
            (ContentTypeNotFound, "Content-Type is not set"),
            (ReadContentTypeFailed, "Failed to read Content-Type value"),
            (
                ContentTypeMismatch,
                "Content-Type value is wrong; it must be application/json",
            ),
            (BotTokenNotFound, "X-TRAQ-BOT-TOKEN is not set"),
            (ReadBotTokenFailed, "Failed to read X-TRAQ-BOT-TOKEN value"),
            (BotTokenMismatch, "X-TRAQ-BOT-TOKEN value is wrong"),
            (BotEventNotFound, "X-TRAQ-BOT-EVENT is not set"),
            (ReadBotEventFailed, "Failed to read X-TRAQ-BOT-EVENT value"),
            (BotEventMismatch, "X-TRAQ-BOT-EVENT value is wrong"),
            (ParseBodyFailed, "Failed to parse request body"),
        ];
        for (err, msg) in pairs.iter() {
            assert_eq!(err.to_string(), *msg);
        }
    }

    fn make_headers(event: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert("X-TRAQ-BOT-TOKEN", VERIFICATION_TOKEN.parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert("X-TRAQ-BOT-EVENT", event.parse().unwrap());
        headers
    }

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
        let parser = make_parser();
        let headers = make_headers("PING");
        let body = read_file("testdata/system/ping.json");
        let event = parser.parse(headers, body.as_bytes()).unwrap();
        let payload = serde_json::from_str::<PingPayload>(&body).unwrap();
        assert_eq!(event, Event::Ping(payload));
    }

    #[test]
    fn joined_test() {
        let parser = make_parser();
        let headers = make_headers("JOINED");
        let body = read_file("testdata/system/joined.json");
        let event = parser.parse(headers, body.as_bytes()).unwrap();
        let payload = serde_json::from_str::<JoinedPayload>(&body).unwrap();
        assert_eq!(event, Event::Joined(payload));
    }

    #[test]
    fn left_test() {
        let parser = make_parser();
        let headers = make_headers("LEFT");
        let body = read_file("testdata/system/left.json");
        let event = parser.parse(headers, body.as_bytes()).unwrap();
        let payload = serde_json::from_str::<LeftPayload>(&body).unwrap();
        assert_eq!(event, Event::Left(payload));
    }

    #[test]
    fn message_created_test() {
        let parser = make_parser();
        let headers = make_headers("MESSAGE_CREATED");
        let body = read_file("testdata/message/message_created.json");
        let event = parser.parse(headers, body.as_bytes()).unwrap();
        let payload = serde_json::from_str::<MessageCreatedPayload>(&body).unwrap();
        assert_eq!(event, Event::MessageCreated(payload));
    }

    #[test]
    fn message_deleted_test() {
        let parser = make_parser();
        let headers = make_headers("MESSAGE_DELETED");
        let body = read_file("testdata/message/message_deleted.json");
        let event = parser.parse(headers, body.as_bytes()).unwrap();
        let payload = serde_json::from_str::<MessageDeletedPayload>(&body).unwrap();
        assert_eq!(event, Event::MessageDeleted(payload));
    }

    #[test]
    fn message_updated_test() {
        let parser = make_parser();
        let headers = make_headers("MESSAGE_UPDATED");
        let body = read_file("testdata/message/message_updated.json");
        let event = parser.parse(headers, body.as_bytes()).unwrap();
        let payload = serde_json::from_str::<MessageUpdatedPayload>(&body).unwrap();
        assert_eq!(event, Event::MessageUpdated(payload));
    }

    #[test]
    fn direct_message_created_test() {
        let parser = make_parser();
        let headers = make_headers("DIRECT_MESSAGE_CREATED");
        let body = read_file("testdata/message/direct_message_created.json");
        let event = parser.parse(headers, body.as_bytes()).unwrap();
        let payload = serde_json::from_str::<DirectMessageCreatedPayload>(&body).unwrap();
        assert_eq!(event, Event::DirectMessageCreated(payload));
    }

    #[test]
    fn direct_message_deleted_test() {
        let parser = make_parser();
        let headers = make_headers("DIRECT_MESSAGE_DELETED");
        let body = read_file("testdata/message/direct_message_deleted.json");
        let event = parser.parse(headers, body.as_bytes()).unwrap();
        let payload = serde_json::from_str::<DirectMessageDeletedPayload>(&body).unwrap();
        assert_eq!(event, Event::DirectMessageDeleted(payload));
    }

    #[test]
    fn direct_message_updated_test() {
        let parser = make_parser();
        let headers = make_headers("DIRECT_MESSAGE_UPDATED");
        let body = read_file("testdata/message/direct_message_updated.json");
        let event = parser.parse(headers, body.as_bytes()).unwrap();
        let payload = serde_json::from_str::<DirectMessageUpdatedPayload>(&body).unwrap();
        assert_eq!(event, Event::DirectMessageUpdated(payload));
    }

    #[test]
    fn bot_message_stamps_updated_test() {
        let parser = make_parser();
        let headers = make_headers("BOT_MESSAGE_STAMPS_UPDATED");
        let body = read_file("testdata/message/bot_message_stamps_updated.json");
        let event = parser.parse(headers, body.as_bytes()).unwrap();
        let payload = serde_json::from_str::<BotMessageStampsUpdatedPayload>(&body).unwrap();
        assert_eq!(event, Event::BotMessageStampsUpdated(payload));
    }

    #[test]
    fn channel_created_test() {
        let parser = make_parser();
        let headers = make_headers("CHANNEL_CREATED");
        let body = read_file("testdata/channel/channel_created.json");
        let event = parser.parse(headers, body.as_bytes()).unwrap();
        let payload = serde_json::from_str::<ChannelCreatedPayload>(&body).unwrap();
        assert_eq!(event, Event::ChannelCreated(payload));
    }

    #[test]
    fn channel_topic_changed_test() {
        let parser = make_parser();
        let headers = make_headers("CHANNEL_TOPIC_CHANGED");
        let body = read_file("testdata/channel/channel_topic_changed.json");
        let event = parser.parse(headers, body.as_bytes()).unwrap();
        let payload = serde_json::from_str::<ChannelTopicChangedPayload>(&body).unwrap();
        assert_eq!(event, Event::ChannelTopicChanged(payload));
    }

    #[test]
    fn user_created_test() {
        let parser = make_parser();
        let headers = make_headers("USER_CREATED");
        let body = read_file("testdata/user/user_created.json");
        let event = parser.parse(headers, body.as_bytes()).unwrap();
        let payload = serde_json::from_str::<UserCreatedPayload>(&body).unwrap();
        assert_eq!(event, Event::UserCreated(payload));
    }

    #[test]
    fn tag_added_test() {
        let parser = make_parser();
        let headers = make_headers("TAG_ADDED");
        let body = read_file("testdata/tag/tag_added.json");
        let event = parser.parse(headers, body.as_bytes()).unwrap();
        let payload = serde_json::from_str::<TagAddedPayload>(&body).unwrap();
        assert_eq!(event, Event::TagAdded(payload));
    }

    #[test]
    fn tag_removed_test() {
        let parser = make_parser();
        let headers = make_headers("TAG_REMOVED");
        let body = read_file("testdata/tag/tag_removed.json");
        let event = parser.parse(headers, body.as_bytes()).unwrap();
        let payload = serde_json::from_str::<TagRemovedPayload>(&body).unwrap();
        assert_eq!(event, Event::TagRemoved(payload));
    }
}
