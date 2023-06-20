//! `#[serde(with = "...")]`で使用するためのモジュール群

use serde::de::{Error, Visitor};

struct StringVisitor;

impl<'de> Visitor<'de> for StringVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a string")
    }

    fn visit_string<E: Error>(self, v: String) -> Result<Self::Value, E> {
        Ok(v)
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        Ok(String::from(v))
    }
}

#[cfg(feature = "time")]
/// タイムスタンプ型[`crate::payloads::types::TimeStamp`]用のモジュール
pub mod time {
    pub use time::serde::rfc3339::{deserialize, serialize};
}

#[cfg(not(feature = "time"))]
/// タイムスタンプ型[`crate::payloads::types::TimeStamp`]用のモジュール
pub mod time {
    use serde::{Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(data: &String, serializer: S) -> Result<S::Ok, S::Error> {
        data.serialize(serializer)
    }

    pub fn deserialize<'a, D: Deserializer<'a>>(deserializer: D) -> Result<String, D::Error> {
        deserializer.deserialize_string(super::StringVisitor)
    }
}

    struct StringVisitor;

    impl<'de> Visitor<'de> for StringVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "a string")
        }

        fn visit_string<E: Error>(self, v: String) -> Result<Self::Value, E> {
            Ok(v)
        }

        fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
            Ok(String::from(v))
        }
    }
}
