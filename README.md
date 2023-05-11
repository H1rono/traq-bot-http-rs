# traq-bot-http-rs

[![Rust](https://github.com/H1rono/traq-bot-http-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/H1rono/traq-bot-http-rs/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/H1rono/traq-bot-http-rs/branch/main/graph/badge.svg?token=UEA9118L9I)](https://codecov.io/gh/H1rono/traq-bot-http-rs)
[![Release](https://github.com/H1rono/traq-bot-http-rs/actions/workflows/release.yml/badge.svg)](https://github.com/H1rono/traq-bot-http-rs/actions/workflows/release.yml)

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/H1rono/traq-bot-http-rs/blob/main/LICENSE)
[![GitHub release](https://img.shields.io/github/v/release/H1rono/traq-bot-http-rs?logo=github)](https://github.com/H1rono/traq-bot-http-rs/releases/latest)
[![crate](https://img.shields.io/crates/v/traq-bot-http.svg?logo=rust)](https://crates.io/crates/traq-bot-http)

traQ BOTのPOSTリクエストをパースするライブラリです。

## example

`Cargo.toml`

```toml
# ...

[dependencies]
axum = "0.6"
tokio = { version = "1.0", features = ["full"] }
traq-bot-http = "0.4"
```

`main.rs`

```rust
use std::{env, net::SocketAddr};

use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::post,
    Router,
};

use traq_bot_http::{Event, RequestParser};

#[tokio::main]
async fn main() {
    let verification_token = env::var("VERIFICATION_TOKEN").unwrap();
    let parser = RequestParser::new(&verification_token);
    let app = Router::new().route("/", post(handler)).with_state(parser);
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(
    State(parser): State<RequestParser>,
    headers: HeaderMap,
    body: Bytes,
) -> StatusCode {
    match parser.parse(headers, &body) {
        Ok(Event::MessageCreated(payload)) => {
            print!(
                "{}さんがメッセージを投稿しました。\n内容: {}\n",
                payload.message.user.display_name, payload.message.text
            );
            StatusCode::NO_CONTENT
        }
        Ok(_) => StatusCode::NO_CONTENT,
        Err(err) => {
            eprintln!("ERROR: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
```

## Features

feature | 機能 | バージョン
:-- | :-- | :--
`uuid` | ペイロードのUUID値が[`uuid::Uuid`](https://docs.rs/uuid/latest/uuid/struct.Uuid.html)型に | [v0.4.0](https://github.com/H1rono/traq-bot-http-rs/releases/tag/v0.4.0)から

## Contributing

Issue, Pull Requestは大歓迎です。
