#[cfg(test)]
mod payload_tests {
    use traq_bot_http::payloads::{types::*, *};

    fn read_file(filename: &str) -> String {
        use std::{fs::File, io::Read, path::Path};
        // https://doc.rust-lang.org/rust-by-example/std_misc/file/open.html
        let path = Path::new(filename);
        let mut file = File::open(path).expect("Failed to open file");
        let mut buf = String::new();
        file.read_to_string(&mut buf).expect("Failed to read file");
        buf
    }

    fn takashi_trap() -> User {
        User {
            #[cfg(feature = "uuid")]
            id: uuid::uuid!("dfdff0c9-5de0-46ee-9721-2525e8bb3d45"),
            #[cfg(not(feature = "uuid"))]
            id: "dfdff0c9-5de0-46ee-9721-2525e8bb3d45".to_string(),
            name: "takashi_trap".to_string(),
            display_name: "寺田 健二".to_string(),
            #[cfg(feature = "uuid")]
            icon_id: uuid::uuid!("2bc06cda-bdb9-4a68-8000-62f907f36a92"),
            #[cfg(not(feature = "uuid"))]
            icon_id: "2bc06cda-bdb9-4a68-8000-62f907f36a92".to_string(),
            bot: false,
        }
    }

    fn channel_a_po() -> Channel {
        Channel {
            #[cfg(feature = "uuid")]
            id: uuid::uuid!("f86c925c-3002-4ba5-939a-c92344e534f9"),
            #[cfg(not(feature = "uuid"))]
            id: "f86c925c-3002-4ba5-939a-c92344e534f9".to_string(),
            name: "po".to_string(),
            path: "#a/po".to_string(),
            #[cfg(feature = "uuid")]
            parent_id: uuid::uuid!("ea452867-553b-4808-a14f-a47ee0009ee6"),
            #[cfg(not(feature = "uuid"))]
            parent_id: "ea452867-553b-4808-a14f-a47ee0009ee6".to_string(),
            creator: takashi_trap(),
            created_at: "2018-04-25T12:22:02Z".to_string(),
            updated_at: "2018-04-25T12:22:02Z".to_string(),
        }
    }

    fn embedded_takashi_trap() -> EmbeddedInfo {
        EmbeddedInfo {
            raw: "@takashi_trap".to_string(),
            type_: "user".to_string(),
            #[cfg(feature = "uuid")]
            id: uuid::uuid!("dfdff0c9-5de0-46ee-9721-2525e8bb3d45"),
            #[cfg(not(feature = "uuid"))]
            id: "dfdff0c9-5de0-46ee-9721-2525e8bb3d45".to_string(),
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
                event_time: "2019-05-07T04:50:48.582586882Z".to_string(),
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
                event_time: "2019-05-08T13:49:13.769110201Z".to_string(),
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
                event_time: "2019-05-08T13:49:16.497848449Z".to_string(),
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
                event_time: "2019-05-08T13:33:51.690308239Z".to_string(),
                message: Message {
                    #[cfg(feature = "uuid")]
                    id: uuid::uuid!("bc9106b3-f9b2-4eca-9ba1-72b39b40954e"),
                    #[cfg(not(feature = "uuid"))]
                    id: "bc9106b3-f9b2-4eca-9ba1-72b39b40954e".to_string(),
                    user: takashi_trap(),
                    #[cfg(feature = "uuid")]
                    channel_id: uuid::uuid!("9aba50da-f605-4cd0-a428-5e4558cb911e"),
                    #[cfg(not(feature = "uuid"))]
                    channel_id: "9aba50da-f605-4cd0-a428-5e4558cb911e".to_string(),
                    text: r#"!{"type": "user", "raw": "@takashi_trap", "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45"} こんにちは"#.to_string(),
                    plain_text: "@takashi_trap こんにちは".to_string(),
                    embedded: vec![
                        embedded_takashi_trap(),
                    ],
                    created_at: "2019-05-08T13:33:51.632149265Z".to_string(),
                    updated_at: "2019-05-08T13:33:51.632149265Z".to_string(),
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
                event_time: "2019-05-08T13:33:51.690308239Z".to_string(),
                message: DeletedMessage {
                    #[cfg(feature = "uuid")]
                    id: uuid::uuid!("bc9106b3-f9b2-4eca-9ba1-72b39b40954e"),
                    #[cfg(not(feature = "uuid"))]
                    id: "bc9106b3-f9b2-4eca-9ba1-72b39b40954e".to_string(),
                    #[cfg(feature = "uuid")]
                    channel_id: uuid::uuid!("9aba50da-f605-4cd0-a428-5e4558cb911e"),
                    #[cfg(not(feature = "uuid"))]
                    channel_id: "9aba50da-f605-4cd0-a428-5e4558cb911e".to_string(),
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
                event_time: "2019-05-08T13:33:51.690308239Z".to_string(),
                message: Message {
                    #[cfg(feature = "uuid")]
                    id: uuid::uuid!("bc9106b3-f9b2-4eca-9ba1-72b39b40954e"),
                    #[cfg(not(feature = "uuid"))]
                    id: "bc9106b3-f9b2-4eca-9ba1-72b39b40954e".to_string(),
                    user: takashi_trap(),
                    #[cfg(feature = "uuid")]
                    channel_id: uuid::uuid!("9aba50da-f605-4cd0-a428-5e4558cb911e"),
                    #[cfg(not(feature = "uuid"))]
                    channel_id: "9aba50da-f605-4cd0-a428-5e4558cb911e".to_string(),
                    text: r#"!{"type": "user", "raw": "@takashi_trap", "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45"} こんにちは"#.to_string(),
                    plain_text: "@takashi_trap こんにちは".to_string(),
                    embedded: vec![
                        embedded_takashi_trap(),
                    ],
                    created_at: "2019-05-08T13:33:51.632149265Z".to_string(),
                    updated_at: "2019-05-08T13:33:51.632149265Z".to_string(),
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
                event_time: "2019-05-08T13:36:09.421492525Z".to_string(),
                message: Message {
                    #[cfg(feature = "uuid")]
                    id: uuid::uuid!("2d7ff3f5-c313-4f4a-a9bb-0b5f84d2b6f8"),
                    #[cfg(not(feature = "uuid"))]
                    id: "2d7ff3f5-c313-4f4a-a9bb-0b5f84d2b6f8".to_string(),
                    user: takashi_trap(),
                    #[cfg(feature = "uuid")]
                    channel_id: uuid::uuid!("c5a5a697-3bad-4540-b2da-93dc88181d34"),
                    #[cfg(not(feature = "uuid"))]
                    channel_id: "c5a5a697-3bad-4540-b2da-93dc88181d34".to_string(),
                    text: r#"!{"type": "user", "raw": "@takashi_trap", "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45"} こんにちは"#.to_string(),
                    plain_text: "@takashi_trap こんにちは".to_string(),
                    embedded: vec![
                        embedded_takashi_trap(),
                    ],
                    created_at: "2019-05-08T13:36:09.365393261Z".to_string(),
                    updated_at: "2019-05-08T13:36:09.365393261Z".to_string(),
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
                event_time: "2019-05-08T13:36:09.421492525Z".to_string(),
                message: DeletedDirectMessage {
                    #[cfg(feature = "uuid")]
                    id: uuid::uuid!("2d7ff3f5-c313-4f4a-a9bb-0b5f84d2b6f8"),
                    #[cfg(not(feature = "uuid"))]
                    id: "2d7ff3f5-c313-4f4a-a9bb-0b5f84d2b6f8".to_string(),
                    #[cfg(feature = "uuid")]
                    user_id: uuid::uuid!("dfdff0c9-5de0-46ee-9721-2525e8bb3d45"),
                    #[cfg(not(feature = "uuid"))]
                    user_id: "dfdff0c9-5de0-46ee-9721-2525e8bb3d45".to_string(),
                    #[cfg(feature = "uuid")]
                    channel_id: uuid::uuid!("c5a5a697-3bad-4540-b2da-93dc88181d34"),
                    #[cfg(not(feature = "uuid"))]
                    channel_id: "c5a5a697-3bad-4540-b2da-93dc88181d34".to_string(),
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
                event_time: "2019-05-08T13:36:09.421492525Z".to_string(),
                message: Message {
                    #[cfg(feature = "uuid")]
                    id: uuid::uuid!("2d7ff3f5-c313-4f4a-a9bb-0b5f84d2b6f8"),
                    #[cfg(not(feature = "uuid"))]
                    id: "2d7ff3f5-c313-4f4a-a9bb-0b5f84d2b6f8".to_string(),
                    user: takashi_trap(),
                    #[cfg(feature = "uuid")]
                    channel_id: uuid::uuid!("c5a5a697-3bad-4540-b2da-93dc88181d34"),
                    #[cfg(not(feature = "uuid"))]
                    channel_id: "c5a5a697-3bad-4540-b2da-93dc88181d34".to_string(),
                    text: r#"!{"type": "user", "raw": "@takashi_trap", "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45"} こんにちは"#.to_string(),
                    plain_text: "@takashi_trap こんにちは".to_string(),
                    embedded: vec![
                        embedded_takashi_trap(),
                    ],
                    created_at: "2019-05-08T13:36:09.365393261Z".to_string(),
                    updated_at: "2019-05-08T13:36:09.365393261Z".to_string(),
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
                event_time: "2020-10-17T03:35:34.5326265Z".to_string(),
                #[cfg(feature = "uuid")]
                message_id: uuid::uuid!("200b6600-b2cd-4c1e-b366-9c40308cc087"),
                #[cfg(not(feature = "uuid"))]
                message_id: "200b6600-b2cd-4c1e-b366-9c40308cc087".to_string(),
                stamps: vec![
                    MessageStamp {
                        #[cfg(feature = "uuid")]
                        stamp_id: uuid::uuid!("1cd58034-8998-4b1c-afe4-fcd591354a97"),
                        #[cfg(not(feature = "uuid"))]
                        stamp_id: "1cd58034-8998-4b1c-afe4-fcd591354a97".to_string(),
                        #[cfg(feature = "uuid")]
                        user_id: uuid::uuid!("b80551a5-2768-4d29-ad78-8e0e92330c8d"),
                        #[cfg(not(feature = "uuid"))]
                        user_id: "b80551a5-2768-4d29-ad78-8e0e92330c8d".to_string(),
                        count: 22,
                        created_at: "2020-10-17T03:35:17.89545Z".to_string(),
                        updated_at: "2020-10-17T03:35:34Z".to_string(),
                    },
                    MessageStamp {
                        #[cfg(feature = "uuid")]
                        stamp_id: uuid::uuid!("6fc62b49-dea0-45b8-8c0c-38035082b111"),
                        #[cfg(not(feature = "uuid"))]
                        stamp_id: "6fc62b49-dea0-45b8-8c0c-38035082b111".to_string(),
                        #[cfg(feature = "uuid")]
                        user_id: uuid::uuid!("b80551a5-2768-4d29-ad78-8e0e92330c8d"),
                        #[cfg(not(feature = "uuid"))]
                        user_id: "b80551a5-2768-4d29-ad78-8e0e92330c8d".to_string(),
                        count: 23,
                        created_at: "2020-10-17T03:35:17.737127Z".to_string(),
                        updated_at: "2020-10-17T03:35:34Z".to_string(),
                    },
                    MessageStamp {
                        #[cfg(feature = "uuid")]
                        stamp_id: uuid::uuid!("b77fad4e-b63f-42a2-916c-5cfe5af3d8b9"),
                        #[cfg(not(feature = "uuid"))]
                        stamp_id: "b77fad4e-b63f-42a2-916c-5cfe5af3d8b9".to_string(),
                        #[cfg(feature = "uuid")]
                        user_id: uuid::uuid!("b80551a5-2768-4d29-ad78-8e0e92330c8d"),
                        #[cfg(not(feature = "uuid"))]
                        user_id: "b80551a5-2768-4d29-ad78-8e0e92330c8d".to_string(),
                        count: 24,
                        created_at: "2020-10-17T03:34:56.575099Z".to_string(),
                        updated_at: "2020-10-17T03:35:34Z".to_string(),
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
                event_time: "2019-05-08T13:45:51.506206852Z".to_string(),
                channel: Channel {
                    #[cfg(feature = "uuid")]
                    id: uuid::uuid!("711afb4c-23e7-46dc-b845-5160f7088ce9"),
                    #[cfg(not(feature = "uuid"))]
                    id: "711afb4c-23e7-46dc-b845-5160f7088ce9".to_string(),
                    name: "yamada".to_string(),
                    path: "#gps/yamada".to_string(),
                    #[cfg(feature = "uuid")]
                    parent_id: uuid::uuid!("ea452867-553b-4808-a14f-a47ee0009ee6"),
                    #[cfg(not(feature = "uuid"))]
                    parent_id: "ea452867-553b-4808-a14f-a47ee0009ee6".to_string(),
                    creator: takashi_trap(),
                    created_at: "2019-05-08T13:45:51.487718Z".to_string(),
                    updated_at: "2019-05-08T13:45:51.487718Z".to_string(),
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
                event_time: "2019-05-09T11:32:49.505357701Z".to_string(),
                channel: Channel {
                    #[cfg(feature = "uuid")]
                    id: uuid::uuid!("9aba50da-f605-4cd0-a428-5e4558cb911e"),
                    #[cfg(not(feature = "uuid"))]
                    id: "9aba50da-f605-4cd0-a428-5e4558cb911e".to_string(),
                    name: "bot".to_string(),
                    path: "#a/bot".to_string(),
                    #[cfg(feature = "uuid")]
                    parent_id: uuid::uuid!("ea452867-553b-4808-a14f-a47ee0009ee6"),
                    #[cfg(not(feature = "uuid"))]
                    parent_id: "ea452867-553b-4808-a14f-a47ee0009ee6".to_string(),
                    creator: takashi_trap(),
                    created_at: "2019-04-02T06:31:16.229419Z".to_string(),
                    updated_at: "2019-05-09T11:32:49.475127Z".to_string(),
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
                event_time: "2019-05-08T08:31:06.566228282Z".to_string(),
                user: User {
                    #[cfg(feature = "uuid")]
                    id: uuid::uuid!("dfdff0c9-5de0-46ee-9721-2525e8bb3d45"),
                    #[cfg(not(feature = "uuid"))]
                    id: "dfdff0c9-5de0-46ee-9721-2525e8bb3d45".to_string(),
                    name: "takashi_trap".to_string(),
                    display_name: "".to_string(),
                    #[cfg(feature = "uuid")]
                    icon_id: uuid::uuid!("2bc06cda-bdb9-4a68-8000-62f907f36a92"),
                    #[cfg(not(feature = "uuid"))]
                    icon_id: "2bc06cda-bdb9-4a68-8000-62f907f36a92".to_string(),
                    bot: false,
                }
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
                event_time: "2019-05-08T08:31:06.566228282Z".to_string(),
                #[cfg(feature = "uuid")]
                tag_id: uuid::uuid!("2bc06cda-bdb9-4a68-8000-62f907f36a92"),
                #[cfg(not(feature = "uuid"))]
                tag_id: "2bc06cda-bdb9-4a68-8000-62f907f36a92".to_string(),
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
                event_time: "2019-05-08T08:31:06.566228282Z".to_string(),
                #[cfg(feature = "uuid")]
                tag_id: uuid::uuid!("2bc06cda-bdb9-4a68-8000-62f907f36a92"),
                #[cfg(not(feature = "uuid"))]
                tag_id: "2bc06cda-bdb9-4a68-8000-62f907f36a92".to_string(),
                tag: "全強".to_string(),
            }
        );
    }
}
