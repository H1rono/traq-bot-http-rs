use std::{env, net::SocketAddr};

use axum::{body::Bytes, extract::State, routing::post, Router};
use http::{HeaderMap, StatusCode};
use tokio::net::TcpListener;

use traq_bot_http::{Event, RequestParser};

#[tokio::main]
async fn main() {
    let verification_token = env::var("VERIFICATION_TOKEN").unwrap();
    let parser = RequestParser::new(&verification_token);
    let app = Router::new().route("/", post(handler)).with_state(parser);
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let server = TcpListener::bind(addr).await.unwrap();
    axum::serve(server, app).await.unwrap();
}

async fn handler(
    State(parser): State<RequestParser>,
    headers: HeaderMap,
    body: Bytes,
) -> StatusCode {
    match parser.parse(&headers, &body) {
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
