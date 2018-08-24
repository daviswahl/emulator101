use crate::machine::cpu;
use crate::machine::memory;
use std::any::Any;
use std::error;
use std::sync;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "MemoryError {}", _0)]
    MemoryError(#[fail(cause)] memory::Error),
    #[fail(display = "CPUEror {}", _0)]
    CPUError(#[fail(cause)] cpu::Error),
    #[fail(display = "LockErr")]
    LockErr,
    #[fail(display = "{}", _0)]
    GameError(#[fail(cause)] ggez::GameError),

    #[fail(display = "{}", _0)]
    ForeignError(String),
}

impl error::Error for Box<Error> {}

impl<T> From<sync::PoisonError<T>> for Error {
    fn from(_err: sync::PoisonError<T>) -> Self {
        Error::LockErr
    }
}

impl From<cpu::Error> for Error {
    fn from(err: cpu::Error) -> Self {
        Error::CPUError(err)
    }
}

impl From<memory::Error> for Error {
    fn from(err: memory::Error) -> Self {
        Error::MemoryError(err)
    }
}

impl From<ggez::GameError> for Error {
    fn from(err: ggez::GameError) -> Self {
        Error::GameError(err)
    }
}

impl From<Box<Any + Send>> for Error {
    fn from(_err: Box<Any + Send>) -> Self {
        Error::ForeignError("Foreign error".to_string())
    }
}
