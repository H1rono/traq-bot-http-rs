//! [`Handler`]の実装で使われるヘルパー群です。
//!
//! # Note
//! このモジュール内のアイテムは全て**unstable**です。
//!
//! [`Handler`]: crate::Handler

use std::convert::Infallible;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::future::{BoxFuture, Ready as ReadyFuture};
use futures::ready;
use http::{Request, Response, StatusCode};
use tower::Service;

use super::Handler;
use crate::macros::all_events;
use crate::{Error, Event, RequestParser};

pin_project_lite::pin_project! {
    #[must_use]
    #[project = WrapErrorFutureProject]
    #[derive(Debug)]
    pub struct WrapErrorFuture<F, E> {
        _error: PhantomData<E>,
        #[pin]
        inner: F,
    }
}

impl<F, E> std::future::Future for WrapErrorFuture<F, E>
where
    F: std::future::Future<Output = Result<(), E>>,
    E: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
{
    type Output = crate::error::Result<()>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let s = self.project();
        let res = ready!(s.inner.poll(cx));
        Poll::Ready(res.map_err(Error::handler))
    }
}

#[must_use]
#[derive(Debug, Clone, Copy, Default, Hash)]
pub struct Sink {
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
        $(
            $crate::macros::event_service! {
                #[must_use]
                #[derive(Debug, Clone)]
                pub $e
            }
        )*
    };
}

all_events! {all_event_service}

impl<Service> Handler<Service> {
    pub fn new(parser: crate::RequestParser, service: Service) -> Self {
        Self { service, parser }
    }

    pub fn with_state<State>(self, state: State) -> Handler<WithState<State, Service>> {
        let Self { service, parser } = self;
        Handler {
            service: WithState { state, service },
            parser,
        }
    }
}

impl RequestParser {
    pub fn into_handler(self) -> Handler<Sink> {
        Handler::new(self, Sink::new())
    }
}

macro_rules! all_handler_on_events {
    (
        $( $e:ident ),*
    ) => {
        $crate::macros::handler_on_events! {
            $( pub $e; )*
        }
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
        // FIXME: このclone消せる
        // req.parts.headersからEventKindはasyncなしに判定できるので
        let mut s = self.clone();
        // https://docs.rs/tower/latest/tower/trait.Service.html#be-careful-when-cloning-inner-services
        std::mem::swap(self, &mut s);
        Box::pin(async move {
            let event = s.parser.parse_request(req).await?;
            s.service.call(event).await.map_err(Error::handler)?;
            Response::builder()
                .status(StatusCode::NO_CONTENT)
                .body(String::new())
                .map_err(Error::handler)
        })
    }
}
