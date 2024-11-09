#![deny(clippy::pedantic, clippy::cargo)]

//! [![GitHub](https://img.shields.io/github/license/H1rono/traq-bot-http-rs?style=for-the-badge&logo=github)](https://github.com/H1rono/traq-bot-http-rs/blob/main/LICENSE)
//! [![Crates.io](https://img.shields.io/crates/l/traq-bot-http?style=for-the-badge&logo=docsdotrs)](https://crates.io/crates/traq-bot-http)
//! [![GitHub release](https://img.shields.io/github/v/release/H1rono/traq-bot-http-rs?style=for-the-badge&logo=github)](https://github.com/H1rono/traq-bot-http-rs/releases/latest)
//! [![crate](https://img.shields.io/crates/v/traq-bot-http.svg?style=for-the-badge&logo=rust)](https://crates.io/crates/traq-bot-http)
//!
//! traQ BOTのPOSTリクエストをパースするライブラリです。
//!
//! [examples](https://github.com/H1rono/traq-bot-http-rs/blob/main/examples)

mod error;
mod events;
pub(crate) mod macros;
mod parser;
pub mod payloads;

#[cfg(feature = "tower")]
pub mod handler;

pub use error::{Error, ErrorKind, Result};
pub use events::{Event, EventKind};

/// HTTP POSTリクエストのパーサー
#[must_use]
#[derive(Debug, Clone)]
pub struct RequestParser {
    verification_token: String,
}

#[cfg(feature = "tower")]
/// axumライクなhandler APIを提供します。[`handler`]モジュールのドキュメントも併せて読んでください。
///
/// # Example
///
/// ```
/// use tower::service_fn;
/// use traq_bot_http::{payloads, RequestParser};
///
/// async fn on_ping((state, payload): (i32, payloads::PingPayload)) -> Result<(), std::convert::Infallible> {
///     println!("state: {state:?}, ping: {payload:?}");
///     // assert_eq!(state, 0);
///     Ok::<(), std::convert::Infallible>(())
/// }
///
/// let parser = RequestParser::new("traqbotverificationtoken");
/// let handler = parser
///     .into_handler()
///     .on_ping(service_fn(on_ping))
///     .with_state(0i32);
/// # let _ = handler;
/// ```
///
/// # Note
/// この構造体の型パラメータは**unstable**です。`Handler<T>`における`T`は予告なく変化する可能性があります。
///
/// [`handler`]: crate::handler
#[must_use]
#[derive(Debug, Clone)]
pub struct Handler<Service> {
    service: Service,
    parser: RequestParser,
}

#[cfg(test)]
pub(crate) mod test_utils;
