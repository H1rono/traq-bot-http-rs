//! [`Handler`]の実装で使われるヘルパー群です。
//!
//! # Note
//! このモジュール内のアイテムは全て**unstable**です。
//!
//! [`Handler`]: crate::Handler

use std::convert::Infallible;
use std::marker::PhantomData;
use std::task::{Context, Poll};

use futures::future::{BoxFuture, Ready as ReadyFuture};
use http::{Request, Response, StatusCode};
use paste::paste;
use tower::Service;

use super::Handler;
use crate::macros::all_events;
use crate::{Error, Event, RequestParser};

mod future;

#[allow(clippy::module_name_repetitions)]
pub use future::{HandlerCall, WrapErrorFuture};

/// handleされなかった[`Event`]の受け皿となる[`Service`]です。
///
/// [`Event`]: crate::Event
/// [`Service`]: tower::Service
#[must_use]
#[derive(Debug, Clone, Copy, Default, Hash)]
pub struct Sink {
    // ユーザーが直接構築できないように
    _priv: PhantomData<()>,
}

impl Sink {
    fn new() -> Self {
        Self { _priv: PhantomData }
    }
}

impl<T> Service<T> for Sink {
    type Response = ();
    type Error = Infallible;
    type Future = ReadyFuture<Result<(), Infallible>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _request: T) -> Self::Future {
        futures::future::ready(Ok(()))
    }
}

/// [`Event`]と`State`の直積です。
///
/// [`Event`]: crate::Event
#[must_use]
#[derive(Debug, Clone)]
pub struct EventWithState<State> {
    state: State,
    event: Event,
}

impl<State> From<(State, Event)> for EventWithState<State> {
    fn from((state, event): (State, Event)) -> Self {
        Self { state, event }
    }
}

impl<State> From<EventWithState<State>> for (State, Event) {
    fn from(value: EventWithState<State>) -> Self {
        let EventWithState { state, event } = value;
        (state, event)
    }
}

/// 内部の`Service`に`State`を渡す[`Service`]です。
///
/// `WithState::call`の度に`State`がcloneされるため、`State`は[`Clone`]を実装する必要があります。
///
/// [`Service`]: tower::Service
/// [`Clone`]: std::clone::Clone
#[must_use]
#[derive(Debug, Clone)]
pub struct WithState<State, Service> {
    state: State,
    service: Service,
}

impl<State, Srv> Service<Event> for WithState<State, Srv>
where
    Srv: Service<EventWithState<State>>,
    State: Clone,
{
    type Response = Srv::Response;
    type Error = Srv::Error;
    type Future = Srv::Future;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: Event) -> Self::Future {
        let request = EventWithState {
            state: self.state.clone(),
            event: request,
        };
        self.service.call(request)
    }
}

impl<OState, State, Srv> Service<EventWithState<OState>> for WithState<State, Srv>
where
    Srv: Service<EventWithState<State>>,
    State: Clone,
{
    type Response = Srv::Response;
    type Error = Srv::Error;
    type Future = Srv::Future;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: EventWithState<OState>) -> Self::Future {
        let request = EventWithState {
            state: self.state.clone(),
            event: request.event,
        };
        self.service.call(request)
    }
}

macro_rules! all_event_service {
    (
        $( $e:ident ),*
    ) => {
        $( $crate::macros::event_service! {
            #[doc = paste! { concat!(
                "[`", stringify!([< $e:camel Payload >]), "`]をhandleする[`Service`]です。\n\n",
                "[`Service`]: tower::Service\n",
                "[`", stringify!([< $e:camel Payload >]), "`]: ",
                "crate::payloads::", stringify!([< $e:camel Payload >]),
            )}]
            #[must_use]
            #[derive(Debug, Clone)]
            pub $e
        } )*
    };
}

all_events! {all_event_service}

impl<Service> Handler<Service> {
    /// 新しくイベントハンドラを作成します。`service`は以下の条件を満たす必要があります。
    ///
    /// - <code>[Service]<[Event]></code>, [`Clone`]を実装している
    /// - [`'static`]
    /// - `Service::Response`が`()`と等しい
    /// - `Service::Error`が<code>Into<Box<dyn [Error] + [Send] + [Sync] + &#39;static>></code>を実装している
    /// - `Service::Future`が[`Send`]を実装している
    ///
    /// [Service]: tower::Service
    /// [Event]: crate::Event
    /// [`Clone`]: std::clone::Clone
    /// [`'static`]: https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html#trait-bound
    /// [Error]: std::error::Error
    /// [Send]: std::marker::Send
    /// [Sync]: std::marker::Sync
    /// [`Send`]: std::marker::Send
    pub fn new(parser: crate::RequestParser, service: Service) -> Self {
        Self { service, parser }
    }

    /// イベントハンドラに`State`を追加します。`State`は以下の条件を満たす必要があります。
    ///
    /// - [`Clone`], [`Send`]を実装している
    /// - [`'static`]
    ///
    /// # Example
    ///
    /// ```
    /// use std::convert::Infallible;
    ///
    /// use tower::service_fn;
    /// use traq_bot_http::{payloads, RequestParser};
    ///
    /// async fn on_ping(state: i32, payload: payloads::PingPayload) -> Result<(), Infallible> {
    ///     println!("state: {state}, payload: {payload:?}");
    ///     Ok(())
    /// }
    ///
    /// let parser = RequestParser::new("verification_token");
    /// let handler = parser
    ///     .into_handler()
    ///     // これはinvalidです; with_stateはstateを使用するハンドラより後に置く必要があります
    ///     // .with_state(0)
    ///     .on_ping(service_fn(|(state, payload)| on_ping(state, payload)))
    ///     .with_state(0);
    /// # let _ = handler;
    /// ```
    ///
    /// [`Clone`]: std::clone::Clone
    /// [`Send`]: std::marker::Send
    /// [`'static`]: https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html#trait-bound
    pub fn with_state<State>(self, state: State) -> Handler<WithState<State, Service>> {
        let Self { service, parser } = self;
        Handler {
            service: WithState { state, service },
            parser,
        }
    }
}

impl RequestParser {
    /// [イベントハンドラ](crate::Handler)に変換します。
    ///
    /// **Note**: この関数は`tower`featureが有効になっている時のみ提供されます。
    pub fn into_handler(self) -> Handler<Sink> {
        Handler::new(self, Sink::new())
    }
}

macro_rules! all_handler_on_events {
    (
        $( $e:ident ),*
    ) => {
        $crate::macros::handler_on_events! { $(
            #[doc = paste! { concat!(
                "[`", stringify!([< $e:camel Payload >]), "`]をhandleする[`Service`]を登録します。\n\n",
                "引数の型`Service2`は`Service<Req>` traitを実装し、さらに以下の条件を満たす必要があります。\n\n",
                "- [`Clone`], [`Send`]を実装している\n",
                "- [`'static`]\n",
                "- `Req`が次のうちいずれかと等しい\n",
                "  - [`", stringify!([< $e:camel Payload >]), "`]\n",
                "  - `(", stringify!([< $e:camel Payload >]), ",)`\n",
                "  - `(State, ", stringify!([< $e:camel Payload >]), ")` ",
                "(`State`に関しては[`Handler::with_state`]を参照してください)\n",
                "- `Service2::Response`が`()`と等しい\n",
                "- `Service2::Error`が<code>Into<Box<dyn [Error] + Send + Sync + &#39;static>></code>を実装している\n",
                "- `Service2::Future`が[`Send`]を実装している\n\n",
                "[`Service`]: tower::Service\n",
                "[`", stringify!([< $e:camel Payload >]), "`]: ",
                "crate::payloads::", stringify!([< $e:camel Payload >]), "\n",
                "[`Clone`]: std::clone::Clone\n",
                "[`Send`]: std::marker::Send\n",
                "[`'static`]: https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html#trait-bound\n",
                "[`Handler::with_state`]: crate::Handler::with_state\n",
                "[Error]: std::error::Error\n",
            )}]
            pub $e;
        )* }
    };
}

all_events! {all_handler_on_events}

impl<Srv, Body> Service<Request<Body>> for Handler<Srv>
where
    Srv: Service<Event, Response = ()> + Send + 'static,
    Srv: Clone,
    Srv::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
    Srv::Future: Send,
    Body: http_body::Body + Send + 'static,
    Body::Data: Send,
    Body::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
{
    type Response = Response<String>;
    type Error = Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx).map_err(Error::handler)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let parse_request = self.parser.parse_request(req);
        let mut s = self.service.clone();
        // https://docs.rs/tower/latest/tower/trait.Service.html#be-careful-when-cloning-inner-services
        std::mem::swap(&mut self.service, &mut s);
        Box::pin(async move {
            let event = parse_request.await?;
            s.call(event).await.map_err(Error::handler)?;
            Response::builder()
                .status(StatusCode::NO_CONTENT)
                .body(String::new())
                .map_err(Error::handler)
        })
    }
}
