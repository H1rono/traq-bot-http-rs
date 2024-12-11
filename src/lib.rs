#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]

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
pub mod parser;
pub mod payloads;

#[cfg(feature = "tower")]
pub mod handler;

use std::sync::Arc;

pub use error::{Error, ErrorKind, Result};
pub use events::{Event, EventKind};

/// HTTP POSTリクエストのパーサー
#[must_use]
#[derive(Debug, Clone)]
pub struct RequestParser {
    inner: Arc<parser::Inner>,
}

#[cfg(feature = "tower")]
/// イベントハンドラです。
///
/// # Example
///
/// ```
/// use std::convert::Infallible;
///
/// use tower::service_fn;
/// use traq_bot_http::{payloads, RequestParser};
///
/// async fn on_ping((state, payload): (i32, payloads::PingPayload)) -> Result<(), Infallible> {
///     println!("state: {state:?}, ping: {payload:?}");
///     // assert_eq!(state, 0);
///     Ok(())
/// }
///
/// let parser = RequestParser::new("verification_token");
/// let handler = parser
///     .into_handler()
///     .on_ping(service_fn(on_ping))
///     .with_state(0i32);
/// # let _ = handler;
/// ```
///
/// # Composing Handler
///
/// [`Handler`] はメソッドチェーンにより構成されます。使用可能なメソッドは以下の通りです。
///
/// - [`.on_*<S>(S)`]
///     - `*`には [`EventKind`] の variant が `snake_case` で入ります。
///     - 例: [`Handler::on_message_created`]
/// - [`.with_state<S>(S)`]
///
/// 適切に構成された [`Handler`] は [`Service`] trait を実装します。各メソッドのドキュメントを参照してください。
///
/// **[`Handler`] の構成時にはコンパイルエラーが出ない可能性があります** 。
/// [`Service`] trait を使用するライブラリ (axum 等) の条件も確認してください。
///
/// # Note
///
/// この構造体の型パラメータは **unstable** です。`Handler<T>`における`T`は予告なく変化する可能性があります。
///
/// [`handler`]: crate::handler
/// [`Service`]: tower::Service
/// [`.on_*<S>(S)`]: crate::Handler::on_ping
/// [`.with_state<S>(S)`]: crate::Handler::with_state
#[must_use]
#[derive(Debug, Clone)]
pub struct Handler<Service> {
    service: Service,
    parser: RequestParser,
}

#[cfg(test)]
pub(crate) mod test_utils;
