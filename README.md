# traq-bot-http-rs

[![Rust](https://github.com/H1rono/traq-bot-http-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/H1rono/traq-bot-http-rs/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/H1rono/traq-bot-http-rs/branch/main/graph/badge.svg?token=UEA9118L9I)](https://codecov.io/gh/H1rono/traq-bot-http-rs)
[![Release](https://github.com/H1rono/traq-bot-http-rs/actions/workflows/release.yml/badge.svg)](https://github.com/H1rono/traq-bot-http-rs/actions/workflows/release.yml)
[![docs.rs](https://img.shields.io/docsrs/traq-bot-http?logo=docsdotrs)](https://docs.rs/traq-bot-http/latest/traq_bot_http/)

[![GitHub](https://img.shields.io/github/license/H1rono/traq-bot-http-rs?logo=github)](https://github.com/H1rono/traq-bot-http-rs/blob/main/LICENSE)
[![Crates.io](https://img.shields.io/crates/l/traq-bot-http?logo=docsdotrs)](https://crates.io/crates/traq-bot-http)
[![GitHub release (with filter)](https://img.shields.io/github/v/release/h1rono/traq-bot-http-rs?logo=github)](https://github.com/H1rono/traq-bot-http-rs/releases/latest)
[![Crates.io](https://img.shields.io/crates/v/traq-bot-http?logo=rust)](https://crates.io/crates/traq-bot-http)

traQ BOTのPOSTリクエストをパースするライブラリです。

## example

`Cargo.toml`

```toml
# ...

[dependencies]
http = "1"
tower = "0.4"
axum = "0.7"
tokio = { version = "1", features = ["full"] }
traq-bot-http = { version = "0.10.2", features = ["tower"] }
```

`main.rs`

```rust
use std::{env, net::SocketAddr};

use axum::{routing::post_service, Router};
use http::StatusCode;
use tokio::net::TcpListener;
use tower::service_fn;

use traq_bot_http::{payloads, RequestParser};

#[tokio::main]
async fn main() {
    let verification_token = env::var("VERIFICATION_TOKEN").unwrap();
    let parser = RequestParser::new(&verification_token);
    let handler = parser
        .into_handler()
        .on_message_created(service_fn(on_message_created));
    let app = Router::new().route(
        "/",
        post_service(handler).handle_error(|_| async { StatusCode::INTERNAL_SERVER_ERROR }),
    );
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let server = TcpListener::bind(addr).await.unwrap();
    axum::serve(server, app).await.unwrap();
}

async fn on_message_created(
    payload: payloads::MessageCreatedPayload,
) -> Result<(), std::convert::Infallible> {
    print!(
        "{}さんがメッセージを投稿しました。\n内容: {}\n",
        payload.message.user.display_name, payload.message.text
    );
    Ok(())
}
```

## Features

feature | 機能 | バージョン
:-- | :-- | :--
`uuid` | ペイロードのUUID値が[`uuid::Uuid`](https://docs.rs/uuid/latest/uuid/struct.Uuid.html)型に | [v0.4.0](https://github.com/H1rono/traq-bot-http-rs/releases/tag/v0.4.0)から
`time` | ペイロードのタイムスタンプ値([RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.6))が[`time::OffsetDateTime`](https://docs.rs/time/latest/time/struct.OffsetDateTime.html)型に | [v0.5.0](https://github.com/H1rono/traq-bot-http-rs/releases/tag/v0.5.0)から
`chrono` | ペイロードのタイムスタンプ値が[`chrono::DateTime<chrono::Utc>`](https://docs.rs/chrono/latest/chrono/struct.DateTime.html)型に | [v0.6.0](https://github.com/H1rono/traq-bot-http-rs/releases/tag/v0.6.0)から
`http` | [`http::Request`](https://docs.rs/http/latest/http/request/struct.Request.html)型のサポート | [v0.10.0](https://github.com/H1rono/traq-bot-http-rs/releases/tag/v0.10.0)から
`tower` | [`Handler`](https://docs.rs/traq-bot-http/latest/traq_bot_http/struct.Handler.html)構造体 | [v0.10.1](https://github.com/H1rono/traq-bot-http-rs/releases/tag/v0.10.1)から

※`time`よりも`chrono`の方が優先されます

## Supported Rust Version

現行の MSRV(Minimum Supported Rust Version) は **1.76.0** です。

将来的にMSRVは変更される可能性がありますが、minorバージョンの更新で導入されます。

## Contributing

Issue, Pull Requestは大歓迎です。
