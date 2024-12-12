use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::ready;
use http::{Response, StatusCode};
use http_body::Body;
use pin_project_lite::pin_project;
use tower_service::Service;

use crate::error::{Error, Result};
use crate::events::Event;
use crate::parser::ParseRequest;

pin_project! {
    /// <code>impl [Future]<Output = Result<(), [Error]>></code>
    ///
    /// `F: Future<Output = Result<(), E>>`を受け取り、エラー型`E`を[`Error`]に変換した[`Future`]を返します。
    /// 以下のコードと同様です。
    ///
    /// ```ignore
    /// use futures::TryFutureExt;
    ///
    /// async fn f() -> Result<(), E> { ... }
    ///
    /// let wrap_error = f().map_err(|e| -> traq_bot_http::Error { ... });
    /// ```
    ///
    /// [Future]: std::future::Future
    /// [Error]: crate::error::Error
    /// [`Future`]: std::future::Future
    /// [`Error`]: crate::error::Error
    #[must_use]
    #[project = WrapErrorFutureProject]
    #[derive(Debug)]
    pub struct WrapErrorFuture<F, E> {
        _error: PhantomData<E>,
        #[pin]
        inner: F,
    }
}

impl<F, E> WrapErrorFuture<F, E> {
    pub(crate) fn new(inner: F) -> Self {
        Self {
            _error: PhantomData,
            inner,
        }
    }
}

impl<F, E> Future for WrapErrorFuture<F, E>
where
    F: Future<Output = Result<(), E>>,
    E: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
{
    type Output = Result<()>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let s = self.project();
        let res = ready!(s.inner.poll(cx));
        Poll::Ready(res.map_err(Error::handler))
    }
}

pin_project! {
    #[must_use]
    #[project = HandlerCallParseRequestProject]
    struct HandlerCallParseRequest<B, S>
    where
        B: Body,
    {
        #[pin]
        parse_request: ParseRequest<B>,
        service: S,
    }
}

impl<B, S> Future for HandlerCallParseRequest<B, S>
where
    B: Body,
    B::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
    S: Service<Event>,
{
    type Output = Result<S::Future>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let s = self.project();
        let event = match ready!(s.parse_request.poll(cx)) {
            Ok(e) => e,
            Err(e) => return Poll::Ready(Err(e)),
        };
        let call = s.service.call(event);
        Poll::Ready(Ok(call))
    }
}

pin_project! {
    #[must_use]
    #[project = HandlerCallServiceCallProject]
    struct HandlerCallServiceCall<F> {
        #[pin]
        inner: F,
    }
}

impl<F> Future for HandlerCallServiceCall<F>
where
    F: Future<Output = Result<(), Error>>,
{
    type Output = Result<Response<String>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let s = self.project();
        if let Err(e) = ready!(s.inner.poll(cx)) {
            return Poll::Ready(Err(e));
        }
        let res = Response::builder()
            .status(StatusCode::NO_CONTENT)
            .body(String::new())
            .map_err(Error::handler);
        Poll::Ready(res)
    }
}

pin_project! {
    #[must_use]
    #[project = HandlerCallInnerProject]
    #[project_replace = HandlerCallInnerProjectReplace]
    enum HandlerCallInner<B, S>
    where
        B: Body,
        S: Service<Event>,
    {
        ParseRequest {
            #[pin]
            inner: HandlerCallParseRequest<B, S>,
        },
        ServiceCall {
            #[pin]
            inner: HandlerCallServiceCall<S::Future>,
        },
    }
}

impl<B, S> Future for HandlerCallInner<B, S>
where
    B: Body,
    B::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
    S: Service<Event, Response = (), Error = Error>,
{
    type Output = Result<Response<String>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let s = self.as_mut().project();
        let service_call = match s {
            HandlerCallInnerProject::ParseRequest { inner } => match ready!(inner.poll(cx)) {
                Ok(c) => c,
                Err(e) => return Poll::Ready(Err(e)),
            },
            HandlerCallInnerProject::ServiceCall { inner } => return inner.poll(cx),
        };
        self.project_replace(HandlerCallInner::ServiceCall {
            inner: HandlerCallServiceCall {
                inner: service_call,
            },
        });
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

pin_project! {
    /// <code><[Handler] as [Service]<[Request]<[B]>>>::[Future]</code>
    ///
    /// [Handler]: crate::Handler
    /// [Service]: tower::Service
    /// [Request]: http::Request
    /// [B]: http_body::Body
    /// [Future]: crate::Handler::Future
    #[must_use]
    #[project = HandlerCallProject]
    pub struct HandlerCall<B, S>
    where
        B: Body,
        S: Service<Event>,
    {
        #[pin]
        inner: HandlerCallInner<B, S>,
    }
}

impl<B, S> HandlerCall<B, S>
where
    B: Body,
    S: Service<Event>,
{
    pub(super) fn new(parse_request: ParseRequest<B>, service: S) -> Self {
        Self {
            inner: HandlerCallInner::ParseRequest {
                inner: HandlerCallParseRequest {
                    parse_request,
                    service,
                },
            },
        }
    }
}

impl<B, S> Future for HandlerCall<B, S>
where
    B: Body,
    B::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
    S: Service<Event, Response = (), Error = Error>,
{
    type Output = Result<Response<String>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let s = self.project();
        s.inner.poll(cx)
    }
}
