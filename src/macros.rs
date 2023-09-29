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

pub(crate) use event_convert;
pub(crate) use event_converts;
pub(crate) use impl_display;
pub(crate) use impl_from_str;
pub(crate) use payload_impl;
