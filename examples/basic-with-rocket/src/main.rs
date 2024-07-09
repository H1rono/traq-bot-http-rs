use rocket::data::{FromData, Outcome};
use traq_bot_http::{Event, RequestParser};

#[derive(Debug, Clone)]
struct BotEvent(Event);

#[rocket::launch]
fn launch() -> _ {
    let verification_token = std::env::var("VERIFICATION_TOKEN").unwrap();
    let parser = RequestParser::new(&verification_token);
    rocket::build()
        .mount("/", rocket::routes![handler])
        .manage(parser)
}

#[rocket::post("/", data = "<event>")]
async fn handler(event: BotEvent) {
    let BotEvent(event) = event;
    println!("イベントの種類: {:?}", event.kind());
    let Event::MessageCreated(payload) = event else {
        return;
    };
    print!(
        "{}さんがメッセージを投稿しました。\n内容: {}\n",
        payload.message.user.display_name, payload.message.text
    );
}

#[rocket::async_trait]
impl<'r> FromData<'r> for BotEvent {
    type Error = ();

    async fn from_data(req: &'r rocket::Request<'_>, data: rocket::Data<'r>) -> Outcome<'r, Self> {
        use rocket::http::Status;

        let Some(parser) = req.rocket().state::<RequestParser>() else {
            eprintln!("[BotEvent::from_data] server does not manage traq_bot_http::RequestParser");
            return Outcome::Error((Status::InternalServerError, ()));
        };
        let headers = req.headers().iter().collect::<Vec<_>>();
        let headers = headers.iter().map(|h| (h.name(), h.value()));
        let body = match <&[u8] as FromData>::from_data(req, data).await {
            Outcome::Success(s) => s,
            Outcome::Error((status, err)) => {
                eprintln!("[<&[u8] as rocket::data::FromData>::from_data] {err}");
                return Outcome::Error((status, ()));
            }
            Outcome::Forward(f) => return Outcome::Forward(f),
        };
        let event = match parser.parse(headers, body) {
            Ok(e) => e,
            Err(err) => {
                eprintln!("[traq_bot_http::RequestParser::parse] {err}");
                return Outcome::Error((Status::BadRequest, ()));
            }
        };
        Outcome::Success(Self(event))
    }
}
