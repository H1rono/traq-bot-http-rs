# traq-bot-http-rs

traQ BOTのPOSTリクエストをパースするライブラリ(の予定)です。

## example (想定)

`Cargo.toml`

```toml
# ...

[dependencies]
axum = "0.6"
tokio = { version = "1.0", features = ["full"] }
traq_bot_http = "0.1"
```

`main.rs`

```rust
use std::{env::var as getenv, net::SocketAddr};

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
    let verification_token = getenv("VERIFICATION_TOKEN").unwrap();
    let parser = RequestParser::new(verification_token);
    let app = Router::new()
        .route("/", post(handler))
        .with_state(parser);
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(State(parser): State<RequestParser>, headers: HeaderMap, body: Bytes) -> StatusCode {
    match parser.parse(headers, &body) {
        Ok(Event::MessageCreated(payload)) => {
            print!("{}さんがメッセージを投稿しました。\n内容: {}\n", payload.message.user.display_name, payload.message.text);
            StatusCode::NO_CONTENT
        },
        Ok(_) => {
            StatusCode::NO_CONTENT
        },
        Err(err) => {
            eprintln!("ERROR: {:?}", err);
            StatusCode::SERVER_INTERNAL_ERROR
        },
    }
}
```

## ライブラリ構造 (想定)

```rust
mod traq_bot_http {
    pub enum Event {
        Ping(PingPayload),
        Joined(JoinedPayload),
        ...
    }
    pub mod payloads {
        #[derive(Debug, serde::Deserialize)]
        pub struct PingPayload { ... }
        #[derive(Debug, serde::Deserialize)]
        pub struct JoinedPayload { ... }
        ...
    }
    pub struct RequestParser { ... }
    impl RequestParser {
        fn new(verification_token: String) -> Self { ... }
        fn parse(&self, headers: http::header::HeaderMap, body: &[u8]) -> Result<Event, Error> { ... }
    }
    pub enum Error {
        ContentTypeMismatch,
        VerificationTokenMissing,
        VerificationTokenDifferent,
        EventHeaderMissing,
        UnexpectedEventHeader,
    }
    impl std::error::Error for Error { ... }
}
```

## Contributing

Issue, Pull Requestは大歓迎です。
