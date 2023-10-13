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

macro_rules! payload_impl {
    ($t:ty) => {
        $crate::macros::impl_from_str! {$t}
        $crate::macros::impl_display! {$t}
    };
}

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

macro_rules! event_converts {
    ($($i:ident),*) => {
        $($crate::macros::event_convert! {$i})*
    };
}

macro_rules! match_event_to_kind {
    ($v:ident, $($i:ident),*) => {
        ::paste::paste! {
            match $v {
                $( Event::$i(_) => EventKind::$i, )*
            }
        }
    };
}

macro_rules! match_event_kinds_to_str {
    ($v:ident, $($i:ident),*) => {
        ::paste::paste! {
            match $v {
                $( $i => stringify!([< $i:snake:upper >]), )*
            }
        }
    };
}

macro_rules! match_str_to_event_kinds {
    ($v:ident, $($i:ident),*) => {
        ::paste::paste! {
            match $v {
                $( stringify!([< $i:snake:upper >]) => ::core::result::Result::Ok($i), )*
                _ => ::core::result::Result::Err($crate::ParseError::BotEventMismatch),
            }
        }
    };
}

pub(crate) use event_convert;
pub(crate) use event_converts;
pub(crate) use impl_display;
pub(crate) use impl_from_str;
pub(crate) use match_event_kinds_to_str;
pub(crate) use match_event_to_kind;
pub(crate) use match_str_to_event_kinds;
pub(crate) use payload_impl;

#[cfg(test)]
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
pub(crate) use {
    test_event_convert, test_event_kind_from_str, test_event_kind_to_string, test_event_to_kind,
};
