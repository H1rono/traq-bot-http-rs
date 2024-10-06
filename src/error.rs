use std::fmt;

use crate::macros::error_kinded_with_source;

#[must_use]
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ErrorKind {
    ContentTypeNotFound,
    ReadContentTypeFailed,
    ContentTypeMismatch,
    BotTokenNotFound,
    ReadBotTokenFailed,
    BotTokenMismatch,
    BotEventNotFound,
    ReadBotEventFailed,
    BotEventMismatch,
    ReadBodyFailed,
    ParseBodyFailed,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl Error {
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

    error_kinded_with_source! {pub(crate) ContentTypeNotFound}
    error_kinded_with_source! {pub(crate) ReadContentTypeFailed}
    error_kinded_with_source! {pub(crate) ContentTypeMismatch}
    error_kinded_with_source! {pub(crate) BotTokenNotFound}
    error_kinded_with_source! {pub(crate) ReadBotTokenFailed}
    error_kinded_with_source! {pub(crate) BotTokenMismatch}
    error_kinded_with_source! {pub(crate) BotEventNotFound}
    error_kinded_with_source! {pub(crate) ReadBotEventFailed}
    error_kinded_with_source! {pub(crate) BotEventMismatch}
    error_kinded_with_source! {pub(crate) ReadBodyFailed}
    error_kinded_with_source! {pub(crate) ParseBodyFailed}
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
    pub(crate) fn as_str(&self) -> &'static str {
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

impl std::error::Error for ErrorKind {}
