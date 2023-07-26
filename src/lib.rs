//! [![GitHub](https://img.shields.io/github/license/H1rono/traq-bot-http-rs?style=for-the-badge&logo=github)](https://github.com/H1rono/traq-bot-http-rs/blob/main/LICENSE)
//! [![Crates.io](https://img.shields.io/crates/l/traq-bot-http?style=for-the-badge&logo=docsdotrs)](https://crates.io/crates/traq-bot-http)
//! [![GitHub release](https://img.shields.io/github/v/release/H1rono/traq-bot-http-rs?style=for-the-badge&logo=github)](https://github.com/H1rono/traq-bot-http-rs/releases/latest)
//! [![crate](https://img.shields.io/crates/v/traq-bot-http.svg?style=for-the-badge&logo=rust)](https://crates.io/crates/traq-bot-http)
//!
//! traQ BOTのPOSTリクエストをパースするライブラリです。
//!
//! [examples](https://github.com/H1rono/traq-bot-http-rs/blob/main/examples)

mod events;
mod parser;
pub mod payloads;

pub use events::Event;
pub use parser::{ParseError, RequestParser};
