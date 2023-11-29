//! `#[serde(with = "...")]`で使用するためのモジュール群

#[cfg(feature = "chrono")]
/// タイムスタンプ型[`crate::payloads::types::TimeStamp`]用のモジュール
pub mod timestamp {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use crate::payloads::types::TimeStamp;

    pub fn serialize<S: Serializer>(data: &TimeStamp, serializer: S) -> Result<S::Ok, S::Error> {
        data.serialize(serializer)
    }

    pub fn deserialize<'a, D: Deserializer<'a>>(deserializer: D) -> Result<TimeStamp, D::Error> {
        TimeStamp::deserialize(deserializer)
    }
}

#[cfg(not(feature = "chrono"))]
#[cfg(feature = "time")]
/// タイムスタンプ型[`crate::payloads::types::TimeStamp`]用のモジュール
pub mod timestamp {
    pub use time::serde::rfc3339::{deserialize, serialize};
}

#[cfg(not(feature = "chrono"))]
#[cfg(not(feature = "time"))]
/// タイムスタンプ型[`crate::payloads::types::TimeStamp`]用のモジュール
pub mod timestamp {
    use serde::de::{Error, Visitor};
    use serde::{Deserializer, Serialize, Serializer};

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

    pub fn serialize<S: Serializer>(data: &String, serializer: S) -> Result<S::Ok, S::Error> {
        data.serialize(serializer)
    }

    pub fn deserialize<'a, D: Deserializer<'a>>(deserializer: D) -> Result<String, D::Error> {
        deserializer.deserialize_string(StringVisitor)
    }
}
