use crate::failure::Backtrace;
use crate::failure::Context;
use crate::failure::Fail;

#[derive(Debug)]
pub struct EmulatorError {
    inner: Context<EmulatorErrorKind>,
}

impl Fail for EmulatorError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

use std::fmt;
impl fmt::Display for EmulatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}
impl EmulatorError {
    pub fn kind(&self) -> &EmulatorErrorKind {
        self.inner.get_context()
    }
}

impl From<EmulatorErrorKind> for EmulatorError {
    fn from(kind: EmulatorErrorKind) -> EmulatorError {
        EmulatorError {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<EmulatorErrorKind>> for EmulatorError {
    fn from(inner: Context<EmulatorErrorKind>) -> EmulatorError {
        EmulatorError { inner: inner }
    }
}

#[derive(Fail, Debug)]
pub enum EmulatorErrorKind {
    #[fail(display = "CPUError {}", _0)]
    CPUError(String),
    #[fail(display = "{}", _0)]
    GameError(#[fail(cause)] ggez::GameError),

    #[fail(display = "Failed to obtain lock")]
    LockError(String),
    #[fail(display = "{}", _0)]
    UnknownError(String),
}

impl From<ggez::GameError> for EmulatorError {
    fn from(err: ggez::GameError) -> Self {
        EmulatorErrorKind::GameError(err).into()
    }
}

use std::error::Error;
use std::sync;
impl<T> From<sync::PoisonError<T>> for EmulatorError {
    fn from(err: sync::PoisonError<T>) -> Self {
        EmulatorErrorKind::LockError(err.description().to_owned()).into()
    }
}
