#[cfg(test)]
mod parser_tests {
    const VERIFICATION_TOKEN: &str = "traqbotverificationtoken";

    use http::header::{HeaderMap, CONTENT_TYPE};
    use traq_bot_http::{Event, ParseError, RequestParser};

    fn make_parser() -> RequestParser {
        RequestParser::new(VERIFICATION_TOKEN)
    }

    #[test]
    fn fail_tests() {
        let parser = make_parser();
        let mut headers = HeaderMap::new();
        match parser.parse(headers.clone(), b"") {
            Err(ParseError::ContentTypeNotFound) => (),
            _ => unreachable!(),
        };
        headers.insert(CONTENT_TYPE, "text/plain".parse().unwrap());
        match parser.parse(headers.clone(), b"") {
            Err(ParseError::ContentTypeMismatch) => (),
            _ => unreachable!(),
        };
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        match parser.parse(headers.clone(), b"") {
            Err(ParseError::BotTokenNotFound) => (),
            _ => unreachable!(),
        };
        headers.insert("X-TRAQ-BOT-TOKEN", "invalid　token".parse().unwrap());
        match parser.parse(headers.clone(), b"") {
            Err(ParseError::ReadBotTokenFailed) => (),
            _ => unreachable!(),
        };
        headers.insert("X-TRAQ-BOT-TOKEN", "invalid_token".parse().unwrap());
        match parser.parse(headers.clone(), b"") {
            Err(ParseError::BotTokenMismatch) => (),
            _ => unreachable!(),
        };
        headers.insert(
            "X-TRAQ-BOT-TOKEN",
            "traqbotverificationtoken".parse().unwrap(),
        );
        match parser.parse(headers.clone(), b"") {
            Err(ParseError::BotEventNotFound) => (),
            _ => unreachable!(),
        };
        headers.insert("X-TRAQ-BOT-EVENT", "invalid　event".parse().unwrap());
        match parser.parse(headers.clone(), b"") {
            Err(ParseError::ReadBotEventFailed) => (),
            _ => unreachable!(),
        };
        headers.insert("X-TRAQ-BOT-EVENT", "invalid_event".parse().unwrap());
        match parser.parse(headers.clone(), b"") {
            Err(ParseError::BotEventMismatch) => (),
            _ => unreachable!(),
        };
        headers.insert("X-TRAQ-BOT-EVENT", "PING".parse().unwrap());
        match parser.parse(headers.clone(), b"invalid format") {
            Err(ParseError::ParseBodyFailed) => (),
            _ => unreachable!(),
        };
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
        match parser.parse(headers, body.as_bytes()) {
            Ok(Event::Ping(_)) => (),
            _ => unreachable!(),
        };
    }

    #[test]
    fn joined_test() {
        let parser = make_parser();
        let headers = make_headers("JOINED");
        let body = read_file("testdata/system/joined.json");
        match parser.parse(headers, body.as_bytes()) {
            Ok(Event::Joined(_)) => (),
            _ => unreachable!(),
        };
    }

    #[test]
    fn left_test() {
        let parser = make_parser();
        let headers = make_headers("LEFT");
        let body = read_file("testdata/system/left.json");
        match parser.parse(headers, body.as_bytes()) {
            Ok(Event::Left(_)) => (),
            _ => unreachable!(),
        };
    }

    #[test]
    fn message_created_test() {
        let parser = make_parser();
        let headers = make_headers("MESSAGE_CREATED");
        let body = read_file("testdata/message/message_created.json");
        match parser.parse(headers, body.as_bytes()) {
            Ok(Event::MessageCreated(_)) => (),
            _ => unreachable!(),
        };
    }

    #[test]
    fn message_deleted_test() {
        let parser = make_parser();
        let headers = make_headers("MESSAGE_DELETED");
        let body = read_file("testdata/message/message_deleted.json");
        match parser.parse(headers, body.as_bytes()) {
            Ok(Event::MessageDeleted(_)) => (),
            _ => unreachable!(),
        };
    }

    #[test]
    fn message_updated_test() {
        let parser = make_parser();
        let headers = make_headers("MESSAGE_UPDATED");
        let body = read_file("testdata/message/message_updated.json");
        match parser.parse(headers, body.as_bytes()) {
            Ok(Event::MessageUpdated(_)) => (),
            _ => unreachable!(),
        };
    }

    #[test]
    fn direct_message_created_test() {
        let parser = make_parser();
        let headers = make_headers("DIRECT_MESSAGE_CREATED");
        let body = read_file("testdata/message/direct_message_created.json");
        match parser.parse(headers, body.as_bytes()) {
            Ok(Event::DirectMessageCreated(_)) => (),
            _ => unreachable!(),
        };
    }

    #[test]
    fn direct_message_deleted_test() {
        let parser = make_parser();
        let headers = make_headers("DIRECT_MESSAGE_DELETED");
        let body = read_file("testdata/message/direct_message_deleted.json");
        match parser.parse(headers, body.as_bytes()) {
            Ok(Event::DirectMessageDeleted(_)) => (),
            _ => unreachable!(),
        };
    }

    #[test]
    fn direct_message_updated_test() {
        let parser = make_parser();
        let headers = make_headers("DIRECT_MESSAGE_UPDATED");
        let body = read_file("testdata/message/direct_message_updated.json");
        match parser.parse(headers, body.as_bytes()) {
            Ok(Event::DirectMessageUpdated(_)) => (),
            _ => unreachable!(),
        };
    }

    #[test]
    fn bot_message_stamps_updated_test() {
        let parser = make_parser();
        let headers = make_headers("BOT_MESSAGE_STAMPS_UPDATED");
        let body = read_file("testdata/message/bot_message_stamps_updated.json");
        match parser.parse(headers, body.as_bytes()) {
            Ok(Event::BotMessageStampsUpdated(_)) => (),
            _ => unreachable!(),
        };
    }

    #[test]
    fn channel_created_test() {
        let parser = make_parser();
        let headers = make_headers("CHANNEL_CREATED");
        let body = read_file("testdata/channel/channel_created.json");
        match parser.parse(headers, body.as_bytes()) {
            Ok(Event::ChannelCreated(_)) => (),
            _ => unreachable!(),
        };
    }

    #[test]
    fn channel_topic_changed_test() {
        let parser = make_parser();
        let headers = make_headers("CHANNEL_TOPIC_CHANGED");
        let body = read_file("testdata/channel/channel_topic_changed.json");
        match parser.parse(headers, body.as_bytes()) {
            Ok(Event::ChannelTopicChanged(_)) => (),
            _ => unreachable!(),
        };
    }

    #[test]
    fn user_created_test() {
        let parser = make_parser();
        let headers = make_headers("USER_CREATED");
        let body = read_file("testdata/user/user_created.json");
        match parser.parse(headers, body.as_bytes()) {
            Ok(Event::UserCreated(_)) => (),
            _ => unreachable!(),
        };
    }

    #[test]
    fn tag_added_test() {
        let parser = make_parser();
        let headers = make_headers("TAG_ADDED");
        let body = read_file("testdata/tag/tag_added.json");
        match parser.parse(headers, body.as_bytes()) {
            Ok(Event::TagAdded(_)) => (),
            _ => unreachable!(),
        };
    }

    #[test]
    fn tag_removed_test() {
        let parser = make_parser();
        let headers = make_headers("TAG_REMOVED");
        let body = read_file("testdata/tag/tag_removed.json");
        match parser.parse(headers, body.as_bytes()) {
            Ok(Event::TagRemoved(_)) => (),
            _ => unreachable!(),
        };
    }
}
