use super::OpCode;
use failure::{Backtrace, Context, Fail};
use std::fmt;

#[derive(Debug)]
pub struct Error {
    inner: crate::failure::Context<ErrorKind>,
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl From<crate::machine::memory::Error> for Error {
    fn from(inner: crate::machine::memory::Error) -> Error {
        Error {
            inner: Context::new(ErrorKind::MemoryError(inner)),
        }
    }
}

impl From<crate::machine::Error> for Error {
    fn from(inner: crate::machine::Error) -> Error {
        Error {
            inner: Context::new(ErrorKind::MachineInterfaceError(Box::new(inner))),
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }
}

#[derive(Fail, Debug)]
pub enum ErrorKind {
    #[fail(display = "Could not obtain CPU Lock")]
    LockErr,

    #[fail(display = "MemoryError {}", _0)]
    MemoryError(#[fail(cause)] crate::machine::memory::Error),

    #[fail(display = "MachineInterfaceError {}", _0)]
    MachineInterfaceError(#[fail(cause)] Box<crate::machine::Error>),

    #[fail(display = "Advanced PC Out of Range: {:#X?}, {}", _0, _1)]
    PCOutOfRange(u16, u16),

    #[fail(display = "Unknown Op: {}", _0)]
    UnknownOp(u8),

    #[fail(display = "Unimplemented Instruction: {:?}", _0)]
    UnimplementedInstruction(OpCode),

    #[fail(display = "Unimplemented Op: {:?}", _0)]
    UnimplementedOp(OpCode),

    #[fail(display = " OpError: {}", _0)]
    OpError(String),

    #[fail(display = "exit: {}", _0)]
    Exit(u8),
}
