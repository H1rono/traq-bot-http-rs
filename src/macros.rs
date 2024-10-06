//! 内部実装で使用されるマクロ集

/// [`Deserialize`]を実装する型に[`serde_json::from_str`]で[`FromStr`]の実装を追加するマクロ
///
/// # Example
///
/// ```ignore
/// #[derive(serde::Deserialize)]
/// struct User {
///     pub name: String,
/// }
///
/// impl_from_str! {User}
/// ```
///
/// expands to:
///
/// ```ignore
/// struct User {
///     pub name: String,
/// }
///
/// impl ::serde::Deserialize for User { ... }
///
/// impl ::std::str::FromStr for User {
///     type Err = ::serde_json::Error;
///     fn from_str(s: &str) -> ::std::result::Result<Self, Self::Error> {
///         ::serde_json::from_str(s)
///     }
/// }
/// ```
///
/// [`Deserialize`]: serde::Deserialize
/// [`FromStr`]: std::str::FromStr
macro_rules! impl_from_str {
    ($t:ty) => {
        impl ::std::str::FromStr for $t {
            type Err = ::serde_json::Error;

            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                ::serde_json::from_str(s)
            }
        }
    };
}

/// [`Serialize`]を実装する型に[`serde_json::to_string`]で[`Display`]の実装を追加するマクロ
///
/// # Example
///
/// ```ignore
/// #[derive(serde::Serialize)]
/// struct User {
///     pub name: String,
/// }
///
/// impl_display! {User}
/// ```
///
/// expands to:
///
/// ```ignore
/// struct User {
///     pub name: String,
/// }
///
/// impl ::serde::Serialize for User { ... }
///
/// impl ::std::fmt::Display for User {
///     fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
///         write!(
///             f,
///             "{}",
///             ::serde_json::to_string(self)
///                 .expect("failed to serialize User")
///         )
///     }
/// }
/// ```
///
/// [`Serialize`]: serde::Serialize
/// [`Display`]: std::fmt::Display
macro_rules! impl_display {
    ($t:ty) => {
        impl ::std::fmt::Display for $t {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(
                    f,
                    "{}",
                    ::serde_json::to_string(self)
                        .expect(concat!("failed to serialize ", stringify!($t)))
                )
            }
        }
    };
}

/// [`Deserialize`]と[`Serialize`]を実装している型に[`FromStr`]と[`Display`]の実装を与えるマクロ
///
/// ref: [`impl_from_str`], [`impl_display`]
///
/// # Example
///
/// ```ignore
/// #[derive(serde::Deserialize, serde::Serialize)]
/// struct User {
///     pub name: String,
/// }
///
/// payload_impl! {User}
/// ```
///
/// expands to:
///
/// ```ignore
/// struct User {
///     pub name: String,
/// }
///
/// impl ::serde::Deserialize for User { ... }
/// impl ::serde::Serialize for User { ... }
///
/// impl ::std::str::FromStr for User { ... }
/// impl ::std::fmt::Display for User { ... }
/// ```
///
/// [`Deserialize`]: serde::Deserialize
/// [`Serialize`]: serde::Serialize
/// [`FromStr`]: std::str::FromStr
/// [`Display`]: std::fmt::Display
/// [`impl_from_str`]: crate::macros::impl_from_str
/// [`impl_display`]: crate::macros::impl_display
macro_rules! payload_impl {
    ($t:ty) => {
        $crate::macros::impl_from_str! {$t}
        $crate::macros::impl_display! {$t}
    };
}

/// イベントの種類名に対応するペイロード型に[`payload_impl`]を適用するマクロ。[`all_events`]と組み合わせること
///
/// # Example
///
/// ```ignore
/// payloads_impl_for_kind! {Ping, Joined}
/// ```
///
/// expands to:
///
/// ```ignore
/// crate::macros::payload_impl! {PingPayload}
/// crate::macros::payload_impl! {JoinedPayload}
/// ```
///
/// [`payload_impl`]: crate::macros::payload_impl
/// [`all_events`]: crate::macros::all_events
macro_rules! payloads_impl_for_kinds {
    ($($t:ty),*) => {
        ::paste::paste! {
            $( $crate::macros::payload_impl! { [< $t Payload >] } )*
        }
    };
}

/// イベントの種類名全てを適用するマクロ
///
/// # Example
///
/// ```ignore
/// macro_rules! events_vec {
///     ( $( $kind:ident ),* ) => {
///         vec![ $( stringify!($kind) ),* ]
///     };
/// }
///
/// let events: Vec<&'static str> = all_events!(events_vec);
/// ```
///
/// expands to:
///
/// ```ignore
/// let events: Vec<&'static str> = vec!["Ping", "Joined", ...];
/// ```
macro_rules! all_events {
    ($n:ident) => {
        $n! {
            Ping,
            Joined,
            Left,
            MessageCreated,
            MessageDeleted,
            MessageUpdated,
            DirectMessageCreated,
            DirectMessageDeleted,
            DirectMessageUpdated,
            BotMessageStampsUpdated,
            ChannelCreated,
            ChannelTopicChanged,
            UserCreated,
            StampCreated,
            TagAdded,
            TagRemoved,
            UserGroupCreated,
            UserGroupUpdated,
            UserGroupDeleted,
            UserGroupMemberAdded,
            UserGroupMemberUpdated,
            UserGroupMemberRemoved,
            UserGroupAdminAdded,
            UserGroupAdminRemoved
        }
    };
}

/// イベントペイロードの型に対して`From<t> for Event`の実装を与えるマクロ
///
/// # Example
///
/// ```ignore
/// event_convert! {Ping}
/// ```
///
/// expands to:
///
/// ```ignore
/// impl ::std::convert::From<PingPayload> for Event {
///     fn from(event: PingPayload) -> Self {
///         Event::Ping(event)
///     }
/// }
/// ```
macro_rules! event_convert {
    ($i:ident) => {
        ::paste::paste! {
            impl ::std::convert::From< [<$i Payload>] > for Event {
                fn from(event: [<$i Payload>]) -> Self {
                    Event::$i(event)
                }
            }
        }
    };
}

/// 複数のイベントペイロードに対して[`event_convert`]を適用するマクロ
///
/// # Example
///
/// ```ignore
/// event_converts! {Ping, Joined}
/// ```
///
/// expands to:
///
/// ```ignore
/// impl ::std::convert::From<PingPayload> for Event { ... }
/// impl ::std::convert::From<JoinedPayload> for Event { ... }
/// ```
///
/// [`event_convert`]: crate::macros::event_convert
macro_rules! event_converts {
    ($($i:ident),*) => {
        $($crate::macros::event_convert! {$i})*
    };
}

/// [`Event`]の値から[`EventKind`]への`match`を提供するマクロ。[`all_events`]と組み合わせること
///
/// # Example
///
/// ```ignore
/// let event: Event = todo!();
/// let kind: EventKind = match_event_to_kind!(event, Ping, Joined);
/// ```
///
/// expands to:
///
/// ```ignore
/// let event: Event = todo!()
/// let kind: EventKind = match event {
///     Event::Ping(_) => EventKind::Ping,
///     Event::Joined(_) => EventKind::Joined,
/// };
/// ```
///
/// [`Event`]: crate::events::Event
/// [`EventKind`]: crate::events::EventKind
/// [`all_events`]: crate::macros::all_events
macro_rules! match_event_to_kind {
    ($v:ident, $($i:ident),*) => {
        ::paste::paste! {
            match $v {
                $( Event::$i(_) => EventKind::$i, )*
            }
        }
    };
}

/// [`EventKind`]の値から対応する文字列リテラルへの`match`を提供するマクロ。[`all_events`]と組み合わせること
///
/// # Example
///
/// ```ignore
/// let kind: EventKind = todo!();
/// let kind_str: &'static str = match_event_kinds_to_str(kind, Ping, Joined);
/// ```
///
/// expands to:
///
/// ```ignore
/// let kind: EventKind = todo!();
/// let kind_str: &'static str = match kind {
///     EventKind::Ping => "PING",
///     EventKind::Joined => "Joined",
/// }
/// ```
///
/// [`EventKind`]: crate::events::EventKind
/// [`all_events`]: crate::macros::all_events
macro_rules! match_event_kinds_to_str {
    ($v:ident, $($i:ident),*) => {
        ::paste::paste! {
            match $v {
                $( EventKind::$i => stringify!([< $i:snake:upper >]), )*
            }
        }
    };
}

/// 文字列リテラルから対応する[`EventKind`]の値への`match`を提供するマクロ。[`all_events`]と組み合わせること
///
/// # Example
///
/// ```ignore
/// let kind_str: &str = "PING";
/// let kind: EventKind = match_str_to_event_kinds!(kind_str, Ping, Joined);
/// ```
///
/// expands to:
///
/// ```ignore
/// let kind_str: &str = "PING"
/// let kind: EventKind = match kind_str {
///     "PING" => EventKind::Ping,
///     "JOINED" => EventKind::Joined,
/// };
/// ```
macro_rules! match_str_to_event_kinds {
    ($v:ident, $($i:ident),*) => {
        ::paste::paste! {
            match $v {
                $( stringify!([< $i:snake:upper >]) => ::core::result::Result::Ok(EventKind::$i), )*
                _ => ::core::result::Result::Err($crate::ErrorKind::BotEventMismatch.into()),
            }
        }
    };
}

macro_rules! error_kinded_with_source {
    (
        $( #[$m:meta] )*
        $v:vis $k:ident
    ) => {
        ::paste::paste! {
            $(#[$m])*
            $v fn [< $k:snake >] <E>(source: E) -> Self
            where
                E: ::std::convert::Into<::std::boxed::Box<
                    dyn ::std::error::Error + Send + Sync + 'static
                >>,
            {
                Self::new($crate::error::ErrorKind::[< $k:camel >], source)
            }
        }
    };
}

pub(crate) use {
    all_events, error_kinded_with_source, event_convert, event_converts, impl_display,
    impl_from_str, match_event_kinds_to_str, match_event_to_kind, match_str_to_event_kinds,
    payload_impl, payloads_impl_for_kinds,
};

#[cfg(test)]
/// ペイロード型に対して`into Event`のテスト
macro_rules! test_event_convert {
    ($group:expr, $i:ident) => {
        ::paste::paste! {
            #[test]
            fn [< $i:snake:lower _convert >]() {
                let data = ::std::fs::read_to_string(concat!(
                    "testdata/",
                    $group,
                    "/",
                    stringify!([< $i:snake:lower >]),
                    ".json"
                ))
                .unwrap();
                let payload: [<$i Payload>] = data.parse().unwrap();
                let event: Event = payload.into();
                assert_eq!(event, Event::$i(data.parse().unwrap()));
            }
        }
    };
}

#[cfg(test)]
/// [`Event::kind`]のテスト
///
/// [`Event::kind`]: crate::events::Event::kind
macro_rules! test_event_to_kind {
    ($group:expr, $i:ident) => {
        ::paste::paste! {
            #[test]
            fn [< $i:snake:lower _kind >]() {
                let data = ::std::fs::read_to_string(concat!(
                    "testdata/",
                    $group,
                    "/",
                    stringify!([< $i:snake:lower >]),
                    ".json"
                ))
                .unwrap();
                let payload: [<$i Payload>] = data.parse().unwrap();
                let event: Event = payload.into();
                assert_eq!(event.kind(), EventKind::$i);
            }
        }
    };
}

#[cfg(test)]
/// `FromStr for EventKind`のテスト
macro_rules! test_event_kind_from_str {
    ($i:ident) => {
        ::paste::paste! {
            #[test]
            fn [< $i:snake:lower _kind_from_str >]() {
                let s = stringify!( [< $i:snake:upper >] );
                let kind: EventKind = s.parse().unwrap();
                assert_eq!(kind, EventKind::$i);
            }
        }
    };
}

#[cfg(test)]
/// `Display for EventKind`のテスト
macro_rules! test_event_kind_to_string {
    ($i:ident) => {
        ::paste::paste! {
            #[test]
            fn [< $i:snake:lower _kind_to_string >]() {
                let kind = EventKind::$i;
                let s = kind.to_string();
                assert_eq!(&s, stringify!([< $i:snake:upper >]))
            }
        }
    };
}

#[cfg(test)]
/// [`RequestParser::parse`]のテスト
///
/// [`RequestParser::parse`]: crate::RequestParser::parse
macro_rules! test_parse_payload {
    ($group:expr, $i:ident) => {
        ::paste::paste! {
            #[test]
            fn [< parse_ $i:snake:lower >]() {
                let parser = $crate::test_utils::make_parser();
                let kind = stringify!([< $i:snake:upper >]);
                let headers = $crate::test_utils::make_headers(kind);
                let body = ::std::fs::read_to_string(concat!(
                    "testdata/",
                    $group,
                    "/",
                    stringify!([< $i:snake:lower >]),
                    ".json"
                ))
                .unwrap();
                let event = parser.parse(&headers, body.as_bytes()).unwrap();
                let payload = ::serde_json::from_str::< $crate::payloads:: [< $i Payload >] >(&body).unwrap();
                assert_eq!(event, Event::$i(payload));
            }
        }
    };
}

#[cfg(test)]
pub(crate) use {
    test_event_convert, test_event_kind_from_str, test_event_kind_to_string, test_event_to_kind,
    test_parse_payload,
};
