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
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(data: &String, serializer: S) -> Result<S::Ok, S::Error> {
        data.serialize(serializer)
    }

    pub fn deserialize<'a, D: Deserializer<'a>>(deserializer: D) -> Result<String, D::Error> {
        String::deserialize(deserializer)
    }
}
