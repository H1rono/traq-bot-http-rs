//! [`Handler`]の実装で使われるヘルパー群です。
//!
//! # Note
//! このモジュール内のアイテムは全て**unstable**です。
//!
//! [`Handler`]: crate::Handler

use std::marker::PhantomData;
use std::task::{Context, Poll};

use futures::future::Ready as ReadyFuture;
use http::{Request, Response};
use paste::paste;
use tower_service::Service;

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
    type Error = Error;
    type Future = ReadyFuture<Result<(), Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    #[inline]
    fn call(&mut self, _request: T) -> Self::Future {
        futures::future::ready(Ok(()))
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
    Srv: Service<(State, Event)>,
    State: Clone,
{
    type Response = Srv::Response;
    type Error = Srv::Error;
    type Future = Srv::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    #[inline]
    fn call(&mut self, request: Event) -> Self::Future {
        self.service.call((self.state.clone(), request))
    }
}

impl<OState, State, Srv> Service<(OState, Event)> for WithState<State, Srv>
where
    Srv: Service<(State, Event)>,
    State: Clone,
{
    type Response = Srv::Response;
    type Error = Srv::Error;
    type Future = Srv::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    #[inline]
    fn call(&mut self, (_, request): (OState, Event)) -> Self::Future {
        self.service.call((self.state.clone(), request))
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
    // TODO: これpubにしたい
    /// 新しくイベントハンドラを作成します。`service`は以下の条件を満たす必要があります。
    ///
    /// - <code>[Service]<[Event]></code>, [`Clone`]を実装している
    /// - `Service::Response`が`()`と等しい
    /// - `Service::Error`が<code>Into<Box<dyn [Error] + [Send] + [Sync] + &#39;static>></code>を実装している
    ///
    /// [Service]: tower::Service
    /// [Event]: crate::Event
    /// [`Clone`]: std::clone::Clone
    /// [Error]: std::error::Error
    /// [Send]: std::marker::Send
    /// [Sync]: std::marker::Sync
    fn new(parser: crate::RequestParser, service: Service) -> Self {
        Self { service, parser }
    }

    /// イベントハンドラに`State`を追加します。`State`は以下の条件を満たす必要があります。
    ///
    /// - [`Clone`]を実装している
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
                "- [`Clone`]を実装している\n",
                "- `Req`が次のうちいずれかと等しい\n",
                "  - [`", stringify!([< $e:camel Payload >]), "`]\n",
                "  - `(", stringify!([< $e:camel Payload >]), ",)`\n",
                "  - `(State, ", stringify!([< $e:camel Payload >]), ")` ",
                "(`State`に関しては[`Handler::with_state`]を参照してください)\n",
                "- `Service2::Response`が`()`と等しい\n",
                "- `Service2::Error`が<code>Into<Box<dyn [Error] + [Send] + [Sync] + &#39;static>></code>を実装している\n\n",
                "[`Service`]: tower::Service\n",
                "[`", stringify!([< $e:camel Payload >]), "`]: ",
                "crate::payloads::", stringify!([< $e:camel Payload >]), "\n",
                "[`Clone`]: std::clone::Clone\n",
                "[`Handler::with_state`]: crate::Handler::with_state\n",
                "[Error]: std::error::Error\n",
                "[Send]: std::marker::Send\n",
                "[Sync]: std::marker::Sync\n",
            )}]
            pub $e;
        )* }
    };
}

all_events! {all_handler_on_events}

impl<Srv, Body> Service<Request<Body>> for Handler<Srv>
where
    Srv: Service<Event, Response = (), Error = Error>,
    Srv: Clone,
    Body: http_body::Body,
    Body::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
{
    type Response = Response<String>;
    type Error = Error;
    type Future = HandlerCall<Body, Srv>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx).map_err(Error::handler)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let parse_request = self.parser.parse_request(req);
        let mut s = self.service.clone();
        // https://docs.rs/tower/latest/tower/trait.Service.html#be-careful-when-cloning-inner-services
        std::mem::swap(&mut self.service, &mut s);
        HandlerCall::new(parse_request, s)
    }
}
