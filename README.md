# traq-bot-http-rs

[![Rust](https://github.com/H1rono/traq-bot-http-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/H1rono/traq-bot-http-rs/actions/workflows/rust.yml) [![Release](https://github.com/H1rono/traq-bot-http-rs/actions/workflows/release.yml/badge.svg)](https://github.com/H1rono/traq-bot-http-rs/actions/workflows/release.yml) [![crate](https://img.shields.io/crates/v/traq-bot-http.svg)](https://crates.io/crates/traq-bot-http)

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

## Contributing

Issue, Pull Requestは大歓迎です。
