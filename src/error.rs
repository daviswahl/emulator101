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
        EmulatorError { inner }
    }
}

impl From<crate::machine::cpu::Error> for EmulatorError {
    fn from(inner: crate::machine::cpu::Error) -> EmulatorError {
        EmulatorError {
            inner: Context::new(EmulatorErrorKind::CPUError(inner)),
        }
    }
}

impl From<crate::machine::memory::Error> for EmulatorError {
    fn from(inner: crate::machine::memory::Error) -> EmulatorError {
        EmulatorError {
            inner: Context::new(EmulatorErrorKind::MemoryError(inner)),
        }
    }
}

impl From<crate::machine::MachineError> for EmulatorError {
    fn from(inner: crate::machine::MachineError) -> EmulatorError {
        EmulatorError {
            inner: Context::new(EmulatorErrorKind::MachineError(inner)),
        }
    }
}
#[derive(Fail, Debug)]
pub enum EmulatorErrorKind {
    #[fail(display = "{}", _0)]
    CPUError(#[fail(cause)] crate::machine::cpu::Error),
    #[fail(display = "{}", _0)]
    GameError(#[fail(cause)] ggez::GameError),

    #[fail(display = "Failed to obtain lock")]
    LockError(String),

    #[fail(display = "Memory Error: {}", _0)]
    MemoryError(#[fail(cause)] crate::machine::memory::Error),

    #[fail(display = "Machine Error: {}", _0)]
    MachineError(#[fail(cause)] crate::machine::MachineError),

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
