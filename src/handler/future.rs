use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::ready;
use pin_project_lite::pin_project;

use crate::error::{Error, Result};

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
