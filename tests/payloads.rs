#[cfg(test)]
mod payload_tests {
    use traq_bot_http::payloads::{types::*, *};

    use ::serde;

    fn read_file(filename: &str) -> String {
        use std::{fs::File, io::Read, path::Path};
        // https://doc.rust-lang.org/rust-by-example/std_misc/file/open.html
        let path = Path::new(filename);
        let mut file = File::open(path).expect("Failed to open file");
        let mut buf = String::new();
        file.read_to_string(&mut buf).expect("Failed to read file");
        buf
    }

    fn timestamp(v: &'static str) -> TimeStamp {
        use serde::de::value::{Error, StrDeserializer};
        let de = StrDeserializer::<'_, Error>::new(v);
        traq_bot_http::payloads::serde::time::deserialize(de).unwrap()
    }

    fn uuid(v: &'static str) -> Uuid {
        use serde::de::value::{Error, StrDeserializer};
        let de = StrDeserializer::<'_, Error>::new(v);
        traq_bot_http::payloads::serde::uuid::deserialize(de).unwrap()
    }

    fn takashi_trap() -> User {
        User {
            id: uuid("dfdff0c9-5de0-46ee-9721-2525e8bb3d45"),
            name: "takashi_trap".to_string(),
            display_name: "寺田 健二".to_string(),
            icon_id: uuid("2bc06cda-bdb9-4a68-8000-62f907f36a92"),
            bot: false,
        }
    }

    fn channel_a_po() -> Channel {
        Channel {
            id: uuid("f86c925c-3002-4ba5-939a-c92344e534f9"),
            name: "po".to_string(),
            path: "#a/po".to_string(),
            parent_id: uuid("ea452867-553b-4808-a14f-a47ee0009ee6"),
            creator: takashi_trap(),
            created_at: timestamp("2018-04-25T12:22:02Z"),
            updated_at: timestamp("2018-04-25T12:22:02Z"),
        }
    }

    fn embedded_takashi_trap() -> EmbeddedInfo {
        EmbeddedInfo {
            raw: "@takashi_trap".to_string(),
            type_: "user".to_string(),
            id: uuid("dfdff0c9-5de0-46ee-9721-2525e8bb3d45"),
        }
    }

    #[test]
    fn ping_test() {
        let ping = read_file("testdata/system/ping.json");
        let payload = serde_json::from_str::<PingPayload>(&ping).unwrap();
        println!("{}", serde_json::to_string_pretty(&payload).unwrap());
        assert_eq!(
            payload,
            PingPayload {
                event_time: timestamp("2019-05-07T04:50:48.582586882Z"),
            }
        );
    }

    #[test]
    fn joined_test() {
        let data = read_file("testdata/system/joined.json");
        let payload = serde_json::from_str::<JoinedPayload>(&data).unwrap();
        println!("{}", serde_json::to_string_pretty(&payload).unwrap());
        assert_eq!(
            payload,
            JoinedPayload {
                event_time: timestamp("2019-05-08T13:49:13.769110201Z"),
                channel: channel_a_po(),
            }
        );
    }

    #[test]
    fn left_test() {
        let data = read_file("testdata/system/left.json");
        let payload = serde_json::from_str::<LeftPayload>(&data).unwrap();
        println!("{}", serde_json::to_string_pretty(&payload).unwrap());
        assert_eq!(
            payload,
            LeftPayload {
                event_time: timestamp("2019-05-08T13:49:16.497848449Z"),
                channel: channel_a_po(),
            },
        );
    }

    #[test]
    fn message_created_test() {
        let data = read_file("testdata/message/message_created.json");
        let payload = serde_json::from_str::<MessageCreatedPayload>(&data).unwrap();
        println!("{}", serde_json::to_string_pretty(&payload).unwrap());
        assert_eq!(
            payload,
            MessageCreatedPayload {
                event_time: timestamp("2019-05-08T13:33:51.690308239Z"),
                message: Message {
                    id: uuid("bc9106b3-f9b2-4eca-9ba1-72b39b40954e"),
                    user: takashi_trap(),
                    channel_id: uuid("9aba50da-f605-4cd0-a428-5e4558cb911e"),
                    text: r#"!{"type": "user", "raw": "@takashi_trap", "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45"} こんにちは"#.to_string(),
                    plain_text: "@takashi_trap こんにちは".to_string(),
                    embedded: vec![
                        embedded_takashi_trap(),
                    ],
                    created_at: timestamp("2019-05-08T13:33:51.632149265Z"),
                    updated_at: timestamp("2019-05-08T13:33:51.632149265Z"),
                },
            }
        );
    }

    #[test]
    fn message_deleted_test() {
        let data = read_file("testdata/message/message_deleted.json");
        let payload = serde_json::from_str::<MessageDeletedPayload>(&data).unwrap();
        println!("{}", serde_json::to_string_pretty(&payload).unwrap());
        assert_eq!(
            payload,
            MessageDeletedPayload {
                event_time: timestamp("2019-05-08T13:33:51.690308239Z"),
                message: DeletedMessage {
                    id: uuid("bc9106b3-f9b2-4eca-9ba1-72b39b40954e"),
                    channel_id: uuid("9aba50da-f605-4cd0-a428-5e4558cb911e"),
                },
            }
        );
    }

    #[test]
    fn message_updated_test() {
        let data = read_file("testdata/message/message_updated.json");
        let payload = serde_json::from_str::<MessageUpdatedPayload>(&data).unwrap();
        println!("{}", serde_json::to_string_pretty(&payload).unwrap());
        assert_eq!(
            payload,
            MessageUpdatedPayload {
                event_time: timestamp("2019-05-08T13:33:51.690308239Z"),
                message: Message {
                    id: uuid("bc9106b3-f9b2-4eca-9ba1-72b39b40954e"),
                    user: takashi_trap(),
                    channel_id: uuid("9aba50da-f605-4cd0-a428-5e4558cb911e"),
                    text: r#"!{"type": "user", "raw": "@takashi_trap", "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45"} こんにちは"#.to_string(),
                    plain_text: "@takashi_trap こんにちは".to_string(),
                    embedded: vec![
                        embedded_takashi_trap(),
                    ],
                    created_at: timestamp("2019-05-08T13:33:51.632149265Z"),
                    updated_at: timestamp("2019-05-08T13:33:51.632149265Z"),
                }
            }
        )
    }

    #[test]
    fn direct_message_created_test() {
        let data = read_file("testdata/message/direct_message_created.json");
        let payload = serde_json::from_str::<DirectMessageCreatedPayload>(&data).unwrap();
        println!("{}", serde_json::to_string_pretty(&payload).unwrap());
        assert_eq!(
            payload,
            DirectMessageCreatedPayload {
                event_time: timestamp("2019-05-08T13:36:09.421492525Z"),
                message: Message {
                    id: uuid("2d7ff3f5-c313-4f4a-a9bb-0b5f84d2b6f8"),
                    user: takashi_trap(),
                    channel_id: uuid("c5a5a697-3bad-4540-b2da-93dc88181d34"),
                    text: r#"!{"type": "user", "raw": "@takashi_trap", "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45"} こんにちは"#.to_string(),
                    plain_text: "@takashi_trap こんにちは".to_string(),
                    embedded: vec![
                        embedded_takashi_trap(),
                    ],
                    created_at: timestamp("2019-05-08T13:36:09.365393261Z"),
                    updated_at: timestamp("2019-05-08T13:36:09.365393261Z"),
                },
            }
        );
    }

    #[test]
    fn direct_message_deleted_test() {
        let data = read_file("testdata/message/direct_message_deleted.json");
        let payload = serde_json::from_str::<DirectMessageDeletedPayload>(&data).unwrap();
        println!("{}", serde_json::to_string_pretty(&payload).unwrap());
        assert_eq!(
            payload,
            DirectMessageDeletedPayload {
                event_time: timestamp("2019-05-08T13:36:09.421492525Z"),
                message: DeletedDirectMessage {
                    id: uuid("2d7ff3f5-c313-4f4a-a9bb-0b5f84d2b6f8"),
                    user_id: uuid("dfdff0c9-5de0-46ee-9721-2525e8bb3d45"),
                    channel_id: uuid("c5a5a697-3bad-4540-b2da-93dc88181d34"),
                }
            }
        );
    }

    #[test]
    fn direct_message_updated_test() {
        let data = read_file("testdata/message/direct_message_updated.json");
        let payload = serde_json::from_str::<DirectMessageUpdatedPayload>(&data).unwrap();
        println!("{}", serde_json::to_string_pretty(&payload).unwrap());
        assert_eq!(
            payload,
            DirectMessageUpdatedPayload {
                event_time: timestamp("2019-05-08T13:36:09.421492525Z"),
                message: Message {
                    id: uuid("2d7ff3f5-c313-4f4a-a9bb-0b5f84d2b6f8"),
                    user: takashi_trap(),
                    channel_id: uuid("c5a5a697-3bad-4540-b2da-93dc88181d34"),
                    text: r#"!{"type": "user", "raw": "@takashi_trap", "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45"} こんにちは"#.to_string(),
                    plain_text: "@takashi_trap こんにちは".to_string(),
                    embedded: vec![
                        embedded_takashi_trap(),
                    ],
                    created_at: timestamp("2019-05-08T13:36:09.365393261Z"),
                    updated_at: timestamp("2019-05-08T13:36:09.365393261Z"),
                },
            }
        );
    }

    #[test]
    fn bot_message_stamps_updated_test() {
        let data = read_file("testdata/message/bot_message_stamps_updated.json");
        let payload = serde_json::from_str::<BotMessageStampsUpdatedPayload>(&data).unwrap();
        println!("{}", serde_json::to_string_pretty(&payload).unwrap());
        assert_eq!(
            payload,
            BotMessageStampsUpdatedPayload {
                event_time: timestamp("2020-10-17T03:35:34.5326265Z"),
                message_id: uuid("200b6600-b2cd-4c1e-b366-9c40308cc087"),
                stamps: vec![
                    MessageStamp {
                        stamp_id: uuid("1cd58034-8998-4b1c-afe4-fcd591354a97"),
                        user_id: uuid("b80551a5-2768-4d29-ad78-8e0e92330c8d"),
                        count: 22,
                        created_at: timestamp("2020-10-17T03:35:17.89545Z"),
                        updated_at: timestamp("2020-10-17T03:35:34Z"),
                    },
                    MessageStamp {
                        stamp_id: uuid("6fc62b49-dea0-45b8-8c0c-38035082b111"),
                        user_id: uuid("b80551a5-2768-4d29-ad78-8e0e92330c8d"),
                        count: 23,
                        created_at: timestamp("2020-10-17T03:35:17.737127Z"),
                        updated_at: timestamp("2020-10-17T03:35:34Z"),
                    },
                    MessageStamp {
                        stamp_id: uuid("b77fad4e-b63f-42a2-916c-5cfe5af3d8b9"),
                        user_id: uuid("b80551a5-2768-4d29-ad78-8e0e92330c8d"),
                        count: 24,
                        created_at: timestamp("2020-10-17T03:34:56.575099Z"),
                        updated_at: timestamp("2020-10-17T03:35:34Z"),
                    },
                ],
            }
        )
    }

    #[test]
    fn channel_created_test() {
        let data = read_file("testdata/channel/channel_created.json");
        let payload = serde_json::from_str::<ChannelCreatedPayload>(&data).unwrap();
        println!("{}", serde_json::to_string_pretty(&payload).unwrap());
        assert_eq!(
            payload,
            ChannelCreatedPayload {
                event_time: timestamp("2019-05-08T13:45:51.506206852Z"),
                channel: Channel {
                    id: uuid("711afb4c-23e7-46dc-b845-5160f7088ce9"),
                    name: "yamada".to_string(),
                    path: "#gps/yamada".to_string(),
                    parent_id: uuid("ea452867-553b-4808-a14f-a47ee0009ee6"),
                    creator: takashi_trap(),
                    created_at: timestamp("2019-05-08T13:45:51.487718Z"),
                    updated_at: timestamp("2019-05-08T13:45:51.487718Z"),
                },
            }
        );
    }

    #[test]
    fn chennel_topic_changed_test() {
        let data = read_file("testdata/channel/channel_topic_changed.json");
        let payload = serde_json::from_str::<ChannelTopicChangedPayload>(&data).unwrap();
        println!("{}", serde_json::to_string_pretty(&payload).unwrap());
        assert_eq!(
            payload,
            ChannelTopicChangedPayload {
                event_time: timestamp("2019-05-09T11:32:49.505357701Z"),
                channel: Channel {
                    id: uuid("9aba50da-f605-4cd0-a428-5e4558cb911e"),
                    name: "bot".to_string(),
                    path: "#a/bot".to_string(),
                    parent_id: uuid("ea452867-553b-4808-a14f-a47ee0009ee6"),
                    creator: takashi_trap(),
                    created_at: timestamp("2019-04-02T06:31:16.229419Z"),
                    updated_at: timestamp("2019-05-09T11:32:49.475127Z"),
                },
                topic: "トピック".to_string(),
                updater: takashi_trap(),
            }
        );
    }

    #[test]
    fn user_created_test() {
        let data = read_file("testdata/user/user_created.json");
        let payload = serde_json::from_str::<UserCreatedPayload>(&data).unwrap();
        println!("{}", serde_json::to_string_pretty(&payload).unwrap());
        assert_eq!(
            payload,
            UserCreatedPayload {
                event_time: timestamp("2019-05-08T08:31:06.566228282Z"),
                user: User {
                    id: uuid("dfdff0c9-5de0-46ee-9721-2525e8bb3d45"),
                    name: "takashi_trap".to_string(),
                    display_name: "".to_string(),
                    icon_id: uuid("2bc06cda-bdb9-4a68-8000-62f907f36a92"),
                    bot: false,
                }
            }
        )
    }

    #[test]
    fn stamp_created_test() {
        let data = read_file("testdata/stamp/stamp_created.json");
        let payload = serde_json::from_str::<StampCreatedPayload>(&data).unwrap();
        println!("{}", serde_json::to_string_pretty(&payload).unwrap());
        assert_eq!(
            payload,
            StampCreatedPayload {
                event_time: timestamp("2019-05-08T08:31:06.566228282Z"),
                id: uuid("2bc06cda-bdb9-4a68-8000-62f907f36a92"),
                name: "naruhodo".to_string(),
                file_id: uuid("2bc06cda-bdb9-4a68-8000-62f907f36a92"),
                creator: {
                    let mut creator = takashi_trap();
                    creator.display_name.clear();
                    creator
                },
            }
        )
    }

    #[test]
    fn tag_added_test() {
        let data = read_file("testdata/tag/tag_added.json");
        let payload = serde_json::from_str::<TagAddedPayload>(&data).unwrap();
        println!("{}", serde_json::to_string_pretty(&payload).unwrap());
        assert_eq!(
            payload,
            TagAddedPayload {
                event_time: timestamp("2019-05-08T08:31:06.566228282Z"),
                tag_id: uuid("2bc06cda-bdb9-4a68-8000-62f907f36a92"),
                tag: "全強".to_string(),
            }
        );
    }

    #[test]
    fn tag_removed_test() {
        let data = read_file("testdata/tag/tag_removed.json");
        let payload = serde_json::from_str::<TagRemovedPayload>(&data).unwrap();
        println!("{}", serde_json::to_string_pretty(&payload).unwrap());
        assert_eq!(
            payload,
            TagRemovedPayload {
                event_time: timestamp("2019-05-08T08:31:06.566228282Z"),
                tag_id: uuid("2bc06cda-bdb9-4a68-8000-62f907f36a92"),
                tag: "全強".to_string(),
            }
        );
    }
}
