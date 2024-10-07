//! エラー型の定義

use std::fmt;

use crate::macros::error_with_source;

/// リクエスト処理時に発生しうるエラー型です。発生したエラーの種類は[`Error::kind`]を参照してください。
///
/// ## Example
/// ```
/// use traq_bot_http::RequestParser;
/// use http::HeaderMap;
///
/// let verification_token = "verification_token";
/// let parser = RequestParser::new(verification_token);
/// let headers = HeaderMap::new();
/// let body = b"";
/// let parsed = parser.parse(&headers, body);
/// assert!(parsed.is_err());
/// let error = parsed.unwrap_err();
/// println!("{error}");
/// ```
///
/// [`Error::kind`]: crate::error::Error::kind
#[must_use]
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

/// 発生したエラーの種類です。 ([non-exhaustive](https://doc.rust-lang.org/reference/attributes/type_system.html))
#[allow(clippy::module_name_repetitions)]
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ErrorKind {
    /// Content-Typeがヘッダーに含まれていない
    ContentTypeNotFound,
    /// Content-Typeの値を読み取れなかった
    ReadContentTypeFailed,
    /// Content-Typeの値がapplication/jsonで始まっていない
    ContentTypeMismatch,
    /// X-TRAQ-BOT-TOKENがヘッダーに含まれていない
    BotTokenNotFound,
    /// X-TRAQ-BOT-TOKENの値を読み取れなかった
    ReadBotTokenFailed,
    /// X-TRAQ-BOT-TOKENの値がverification_tokenと等しくない
    BotTokenMismatch,
    /// X-TRAQ-BOT-EVENTがヘッダーに含まれていない
    BotEventNotFound,
    /// X-TRAQ-BOT-EVENTの値を読み取れなかった
    ReadBotEventFailed,
    /// X-TRAQ-BOT-EVENTの値がイベント名のいずれでもない
    BotEventMismatch,
    /// リクエストボディの値を読み取れなかった
    ReadBodyFailed,
    /// リクエストボディの値をパースできなかった
    ParseBodyFailed,
}

/// type alias
pub type Result<T, E = Error> = std::result::Result<T, E>;

impl Error {
    /// 対応する[`ErrorKind`]を返します。
    ///
    /// ## Example
    /// ```
    /// use traq_bot_http::{Error, ErrorKind};
    ///
    /// let error_kind = ErrorKind::ContentTypeNotFound;
    /// let error = Error::from(error_kind);
    /// assert_eq!(error.kind(), error_kind);
    /// ```
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl Error {
    pub(crate) fn new<E>(kind: ErrorKind, source: E) -> Self
    where
        E: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
    {
        Self {
            kind,
            source: Some(source.into()),
        }
    }

    error_with_source! {#[allow(dead_code)] pub(crate) ContentTypeNotFound}
    error_with_source! {pub(crate) ReadContentTypeFailed}
    error_with_source! {#[allow(dead_code)] pub(crate) ContentTypeMismatch}
    error_with_source! {#[allow(dead_code)] pub(crate) BotTokenNotFound}
    error_with_source! {pub(crate) ReadBotTokenFailed}
    error_with_source! {#[allow(dead_code)] pub(crate) BotTokenMismatch}
    error_with_source! {#[allow(dead_code)] pub(crate) BotEventNotFound}
    error_with_source! {pub(crate) ReadBotEventFailed}
    error_with_source! {pub(crate) BotEventMismatch}
    error_with_source! {pub(crate) ReadBodyFailed}
    error_with_source! {pub(crate) ParseBodyFailed}
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self { kind, source: None }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.kind.as_str())
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        let s = self.source.as_deref()?;
        Some(s as &(dyn std::error::Error + 'static))
    }
}

impl ErrorKind {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::ContentTypeNotFound => "Content-Type is not set",
            Self::ReadContentTypeFailed => "Failed to read Content-Type value",
            Self::ContentTypeMismatch => "Content-Type value is wrong; it must be application/json",
            Self::BotTokenNotFound => "X-TRAQ-BOT-TOKEN is not set",
            Self::ReadBotTokenFailed => "Failed to read X-TRAQ-BOT-TOKEN value",
            Self::BotTokenMismatch => "X-TRAQ-BOT-TOKEN value is wrong",
            Self::BotEventNotFound => "X-TRAQ-BOT-EVENT is not set",
            Self::ReadBotEventFailed => "Failed to read X-TRAQ-BOT-EVENT value",
            Self::BotEventMismatch => "X-TRAQ-BOT-EVENT value is wrong",
            Self::ReadBodyFailed => "Failed to read request body",
            Self::ParseBodyFailed => "Failed to parse request body",
        }
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::all_error_kinds;

    fn assert_send_sync_static<T: Send + Sync + 'static>() {}
    fn assert_display<T: std::fmt::Display>() {}
    fn assert_error<T: std::error::Error>() {}

    /// `A into B`
    fn assert_convert<A, B>()
    where
        A: Into<B>,
    {
    }

    #[test]
    fn error_impl() {
        assert_send_sync_static::<Error>();
        assert_error::<Error>();
        assert_convert::<ErrorKind, Error>();
    }

    #[test]
    fn error_kind_impl() {
        assert_send_sync_static::<ErrorKind>();
        assert_display::<ErrorKind>();
    }

    macro_rules! tests_error_kind_convert {
        ($( $kind:ident ),*) => {
            $(
                $crate::macros::test_error_kind_convert! {$kind}
            )*
        };
    }

    all_error_kinds! {tests_error_kind_convert}
}
