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

macro_rules! error_with_source {
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

#[cfg(feature = "tower")]
macro_rules! event_service {
    (
        $( #[$m:meta] )*
        $v:vis $e:ident
    ) => { ::paste::paste! {
        $( #[$m] )*
        $v struct
        [< $e:camel Service >] <Service, Fallback, Req>
        {
            _req: ::std::marker::PhantomData<Req>,
            inner: Service,
            fallback: Fallback,
        }

        impl<State, Service, Fallback> ::tower::Service<$crate::handler::EventWithState<State>>
        for [< $e:camel Service >] <Service, Fallback, $crate::payloads::[< $e:camel Payload >] >
        where
            Service: ::tower::Service<
                $crate::payloads::[< $e:camel Payload >],
                Response = (),
            >,
            Service::Error: ::std::convert::Into<::std::boxed::Box<
                dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static,
            >>,
            Fallback: ::tower::Service<$crate::handler::EventWithState<State>, Response = ()>,
            Fallback::Error: ::std::convert::Into<::std::boxed::Box<
                dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static,
            >>,
        {
            type Response = ();
            type Error = $crate::Error;
            type Future = ::futures::future::Either<
                $crate::handler::WrapErrorFuture<Service::Future, Service::Error>,
                $crate::handler::WrapErrorFuture<Fallback::Future, Fallback::Error>,
            >;

            fn poll_ready(&mut self, cx: &mut ::std::task::Context<'_>)
            -> ::std::task::Poll<::std::result::Result<(), Self::Error>>
            {
                if let ::std::result::Result::Err(e)
                    = ::futures::ready!(self.inner.poll_ready(cx))
                {
                    return ::std::task::Poll::Ready(::std::result::Result::Err(
                        $crate::Error::handler(e)
                    ));
                }
                if let ::std::result::Result::Err(e)
                    = ::futures::ready!(self.fallback.poll_ready(cx))
                {
                    return ::std::task::Poll::Ready(::std::result::Result::Err(
                        $crate::Error::handler(e)
                    ));
                }
                ::std::task::Poll::Ready(::std::result::Result::Ok(()))
            }

            fn call(&mut self, req: $crate::handler::EventWithState<State>) -> Self::Future {
                let (state, event) = req.into();
                match event {
                    $crate::Event::[< $e:camel >] (e) => {
                        ::futures::future::Either::Left($crate::handler::WrapErrorFuture {
                            _error: ::std::marker::PhantomData,
                            inner: self.inner.call(e),
                        })
                    },
                    event => {
                        ::futures::future::Either::Right($crate::handler::WrapErrorFuture {
                            _error: ::std::marker::PhantomData,
                            inner: self.fallback.call((state, event).into())
                        })
                    }
                }
            }
        }

        impl<State, Service, Fallback> ::tower::Service<EventWithState<State>>
        for [< $e:camel Service >] <Service, Fallback, (State, $crate::payloads::[< $e:camel Payload >] )>
        where
            Service: ::tower::Service<
                (State, $crate::payloads::[< $e:camel Payload >]),
                Response = (),
            >,
            Service::Error: ::std::convert::Into<::std::boxed::Box<
                dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static,
            >>,
            Fallback: ::tower::Service<EventWithState<State>, Response = ()>,
            Fallback::Error: ::std::convert::Into<::std::boxed::Box<
                dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static,
            >>,
        {
            type Response = ();
            type Error = $crate::Error;
            type Future = ::futures::future::Either<
                $crate::handler::WrapErrorFuture<Service::Future, Service::Error>,
                $crate::handler::WrapErrorFuture<Fallback::Future, Fallback::Error>,
            >;

            fn poll_ready(&mut self, cx: &mut ::std::task::Context<'_>)
            -> ::std::task::Poll<::std::result::Result<(), Self::Error>>
            {
                if let ::std::result::Result::Err(e)
                    = ::futures::ready!(self.inner.poll_ready(cx))
                {
                    return ::std::task::Poll::Ready(::std::result::Result::Err(
                        $crate::Error::handler(e)
                    ));
                }
                if let ::std::result::Result::Err(e)
                    = ::futures::ready!(self.fallback.poll_ready(cx))
                {
                    return ::std::task::Poll::Ready(::std::result::Result::Err(
                        $crate::Error::handler(e)
                    ));
                }
                ::std::task::Poll::Ready(::std::result::Result::Ok(()))
            }

            fn call(&mut self, req: EventWithState<State>) -> Self::Future {
                let (state, event) = req.into();
                match event {
                    $crate::Event::[< $e:camel >] (e) => {
                        ::futures::future::Either::Left($crate::handler::WrapErrorFuture {
                            _error: ::std::marker::PhantomData,
                            inner: self.inner.call((state, e)),
                        })
                    },
                    event => {
                        ::futures::future::Either::Right($crate::handler::WrapErrorFuture {
                            _error: ::std::marker::PhantomData,
                            inner: self.fallback.call((state, event).into())
                        })
                    }
                }
            }
        }
    }};
}

#[cfg(feature = "tower")]
macro_rules! handler_on_events {
    ($(
        $( #[$m:meta] )*
        $v:vis $e:ident;
    )+) => { ::paste::paste! {
        impl<Service1> $crate::Handler<Service1> {
            $(
                $( #[$m] )*
                $v fn [< on_ $e:snake:lower >] <Service2, Req> (self, service: Service2)
                -> $crate::Handler<$crate::handler::[< $e:camel Service >] <Service2, Service1, Req>>
                where
                    Service2: ::tower::Service<Req>,
                {
                    let Self {
                        service: fallback,
                        parser,
                    } = self;
                    $crate::Handler {
                        service: $crate::handler::[< $e:camel Service >] {
                            _req: ::std::marker::PhantomData,
                            inner: service,
                            fallback,
                        },
                        parser,
                    }
                }
            )+
        }
    }};
}

pub(crate) use {
    all_events, error_with_source, event_convert, event_converts, impl_display, impl_from_str,
    match_event_kinds_to_str, match_event_to_kind, match_str_to_event_kinds, payload_impl,
    payloads_impl_for_kinds,
};

#[cfg(feature = "tower")]
pub(crate) use {event_service, handler_on_events};

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
/// [`ErrorKind`] into [`Error`]および[`Error::kind`]のテストを生成するマクロ
///
/// [`ErrorKind`]: crate::error::ErrorKind
/// [`Error`]: crate::error::Error
/// [`Error::kind`]: crate::error::Error::kind
macro_rules! test_error_kind_convert {
    ($kind:ident) => {
        ::paste::paste! {
            #[test]
            fn [< error_kind_ $kind:snake:lower >]() {
                let kind = ErrorKind::$kind;
                let error = Error::from(kind);
                assert_eq!(error.kind(), kind);
            }
        }
    };
}

#[cfg(test)]
/// [`ErrorKind`]のvariant全てを列挙するマクロ
///
/// [`ErrorKind`]: crate::error::ErrorKind
macro_rules! all_error_kinds {
    ($n:ident) => {
        $n! {
            ContentTypeNotFound,
            ReadContentTypeFailed,
            ContentTypeMismatch,
            BotTokenNotFound,
            ReadBotTokenFailed,
            BotTokenMismatch,
            BotEventNotFound,
            ReadBotEventFailed,
            BotEventMismatch,
            ReadBodyFailed,
            ParseBodyFailed,
            Handler
        }
    };
}

#[cfg(test)]
pub(crate) use {
    all_error_kinds, test_error_kind_convert, test_event_convert, test_event_kind_from_str,
    test_event_kind_to_string, test_event_to_kind, test_parse_payload,
};
