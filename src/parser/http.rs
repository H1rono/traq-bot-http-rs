// #![cfg(feature = "http")]

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::Bytes;
use futures::future::Ready;
use futures::ready;
use http_body::Body;
use http_body_util::{combinators::Collect, Collected};
use pin_project_lite::pin_project;

use crate::error::{Error, Result};
use crate::events::{Event, EventKind};
use crate::parser::RequestParser;

pin_project! {
    #[must_use]
    #[project = CollectBodyProject]
    struct CollectBody<B>
    where
        B: Body,
        B: ?Sized,
    {
        #[pin]
        collect: Collect<B>,
    }
}

impl<B> Future for CollectBody<B>
where
    B: Body + ?Sized,
    B::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
{
    type Output = Result<Bytes>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let s = self.project();
        let collected = ready!(s.collect.poll(cx));
        let res = collected
            .map(Collected::to_bytes)
            .map_err(Error::read_body_failed);
        Poll::Ready(res)
    }
}

pin_project! {
    #[must_use]
    #[project = ParseRequestInnerProject]
    struct ParseRequestInner<K, B> {
        #[pin]
        kind: K,
        #[pin]
        body: B,
    }
}

impl<K, B> Future for ParseRequestInner<K, B>
where
    K: Future<Output = Result<EventKind>>,
    B: Future<Output = Result<Bytes>>,
{
    type Output = Result<Event>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let s = self.project();
        let kind = match ready!(s.kind.poll(cx)) {
            Ok(k) => k,
            Err(e) => return Poll::Ready(Err(e)),
        };
        let body = ready!(s.body.poll(cx));
        let res: Result<Event> = {
            let body = body?;
            let body = std::str::from_utf8(&body).map_err(Error::read_body_failed)?;
            super::parse_body(kind, body)
        };
        Poll::Ready(res)
    }
}

pin_project! {
    /// <code>impl [Future]<Output = Result<[Event], [Error]>></code>
    ///
    /// [Future]: std::future::Future
    /// [Event]: crate::Event
    /// [Error]: crate::Error
    #[must_use]
    #[project = ParseRequestProject]
    pub struct ParseRequest<B>
    where
        B: Body,
    {
        #[pin]
        inner: ParseRequestInner<Ready<Result<EventKind>>, CollectBody<B>>
    }
}

impl<B> ParseRequest<B>
where
    B: Body,
{
    fn new(kind: Result<EventKind>, body: B) -> Self {
        use http_body_util::BodyExt;

        let kind = futures::future::ready(kind);
        let inner = ParseRequestInner {
            kind,
            body: CollectBody {
                collect: body.collect(),
            },
        };
        Self { inner }
    }
}

impl<B> Future for ParseRequest<B>
where
    B: Body,
    B::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
{
    type Output = Result<Event>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let s = self.project();
        s.inner.poll(cx)
    }
}

impl RequestParser {
    /// [`http::Request`]をパースします。
    ///
    /// **Note**: この関数は`http`featureが有効になっている時のみ有効です。
    ///
    /// # Arguments
    ///
    /// * `request`: リクエスト全体
    ///
    /// # Example
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let res: Result<(), Box<dyn std::error::Error>> = futures::executor::block_on(async {
    /// use traq_bot_http::{EventKind, RequestParser};
    ///
    /// let verification_token = "verification_token";
    /// let body = r#"{"eventTime": "2019-05-07T04:50:48.582586882Z"}"#.to_string();
    /// let request = http::Request::builder()
    ///     .method(http::Method::POST)
    ///     .header(http::header::CONTENT_TYPE, "application/json")
    ///     .header("X-TRAQ-BOT-TOKEN", verification_token)
    ///     .header("X-TRAQ-BOT-EVENT", "PING")
    ///     .body(body)?;
    /// let parser = RequestParser::new(verification_token);
    /// let event = parser.parse_request(request).await?;
    /// assert_eq!(event.kind(), EventKind::Ping);
    /// # Ok(())
    /// # });
    /// # res
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// [`Error`]のうち、[`Error::kind`]が以下のものを返す可能性があります。
    ///
    /// - [`parse`]で返されるもの
    /// - [`ErrorKind::ReadBodyFailed`] :
    ///     リクエストボディの読み込みに失敗した
    ///
    /// [`Error::kind`]: crate::Error::kind
    /// [`parse`]: crate::RequestParser::parse
    /// [`ErrorKind::ReadBodyFailed`]: crate::ErrorKind::ReadBodyFailed
    pub fn parse_request<B>(&self, request: http::Request<B>) -> ParseRequest<B>
    where
        B: Body,
        B::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
    {
        let (parts, body) = request.into_parts();
        let kind = self.parse_headers(&parts.headers);
        ParseRequest::new(kind, body)
    }
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;
    use http_body_util::BodyExt;

    use super::{CollectBody, ParseRequest};
    use crate::{Event, EventKind};

    #[test]
    fn collect_body() {
        let body_content = "some content";
        let fut = CollectBody {
            collect: body_content.to_string().collect(),
        };
        let collected = block_on(fut).unwrap();
        assert_eq!(collected, body_content.as_bytes());
    }

    #[test]
    fn parse_request_future() {
        let kind = EventKind::Ping;
        let payload = r#"{"eventTime": "2019-05-07T04:50:48.582586882Z"}"#;
        let body = payload.to_string();
        let fut = ParseRequest::new(Ok(kind), body);
        let event = block_on(fut).unwrap();
        assert!(matches!(event, Event::Ping(_)));
    }
}
