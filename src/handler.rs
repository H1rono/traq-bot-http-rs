//! TODO

use std::convert::Infallible;
use std::error::Error as StdError;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::future::{BoxFuture, Ready as ReadyFuture};
use futures::ready;
use http::{Request, Response, StatusCode};
use http_body::Body;
use tower::Service;

use super::Handler;
use crate::macros::all_events;
use crate::{Error, Event, RequestParser};

pub trait Handle<State, Event, Request> {
    type Response;
    type Error;
    type Future: std::future::Future<Output = Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>;

    fn call(&mut self, request: Request) -> Self::Future;
}

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
    E: Into<Box<dyn StdError + Send + Sync + 'static>>,
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
pub struct Sink;

#[must_use]
#[derive(Debug, Clone)]
pub struct EventWithState<State> {
    state: State,
    event: Event,
}

impl<State> From<EventWithState<State>> for (State, Event) {
    fn from(value: EventWithState<State>) -> Self {
        let EventWithState { state, event } = value;
        (state, event)
    }
}

impl<State> From<EventWithState<State>> for Event {
    fn from(value: EventWithState<State>) -> Self {
        let EventWithState { event, .. } = value;
        event
    }
}

#[must_use]
#[derive(Debug, Clone)]
pub struct WithState<State, Service> {
    state: State,
    service: Service,
}

impl<State, Service> tower::Service<Event> for WithState<State, Service>
where
    State: Clone,
    Service: tower::Service<EventWithState<State>>,
{
    type Response = Service::Response;
    type Error = Service::Error;
    type Future = Service::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: Event) -> Self::Future {
        let req = EventWithState {
            state: self.state.clone(),
            event: req,
        };
        self.service.call(req)
    }
}

impl<T> Service<T> for Sink {
    type Response = ();
    type Error = Infallible;
    type Future = ReadyFuture<Result<(), Infallible>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _req: T) -> Self::Future {
        futures::future::ready(Ok(()))
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
}

impl RequestParser {
    pub fn into_handler(self) -> Handler<Sink> {
        Handler::new(self, Sink)
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

impl<S, B> Service<Request<B>> for Handler<S>
where
    S: Service<Event, Response = ()> + Send + 'static,
    S: Clone,
    S::Error: Into<Box<dyn StdError + Send + Sync + 'static>>,
    S::Future: Send,
    B: Body + Send + 'static,
    B::Data: Send,
    B::Error: Into<Box<dyn StdError + Send + Sync + 'static>>,
{
    type Response = Response<String>;
    type Error = Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx).map_err(Error::handler)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
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
