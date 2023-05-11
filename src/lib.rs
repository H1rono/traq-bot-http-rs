//! [![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge)](https://github.com/H1rono/traq-bot-http-rs/blob/main/LICENSE)
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
