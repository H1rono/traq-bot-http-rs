//! traQ BOTのPOSTリクエストをパースするライブラリです。
//! [examples](https://github.com/H1rono/traq-bot-http-rs/blob/main/examples)

mod events;
mod parser;
pub mod payloads;

pub use events::Event;
pub use parser::{ParseError, RequestParser};
