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
