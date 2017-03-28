use std::error::Error;
use std::io;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ffi::{NulError};
use std::string::FromUtf8Error;
use std::sync::PoisonError;

use trace_error::TraceResult;

pub type BackendResult<T> = TraceResult<T, BackendError>;

#[cfg(feature = "gl")]
use ::backends::gl::wrapper::GLError;

#[derive(Debug)]
pub enum BackendError {
    NulError(NulError),
    Io(io::Error),
    FromUtf8Error(FromUtf8Error),
    PoisonError,
    #[cfg(feature = "gl")]
    GLError(GLError)
}

impl From<NulError> for BackendError {
    fn from(err: NulError) -> BackendError {
        BackendError::NulError(err)
    }
}

impl From<io::Error> for BackendError {
    fn from(err: io::Error) -> BackendError {
        BackendError::Io(err)
    }
}

impl From<FromUtf8Error> for BackendError {
    fn from(err: FromUtf8Error) -> BackendError {
        BackendError::FromUtf8Error(err)
    }
}

impl<T> From<PoisonError<T>> for BackendError {
    fn from(_: PoisonError<T>) -> BackendError {
        BackendError::PoisonError
    }
}

impl Display for BackendError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.description())
    }
}

impl Error for BackendError {
    fn description(&self) -> &str {
        match *self {
            BackendError::NulError(ref err) => err.description(),
            BackendError::Io(ref err) => err.description(),
            BackendError::FromUtf8Error(ref err) => err.description(),
            BackendError::PoisonError => "Poison Error",
            #[cfg(feature = "gl")]
            BackendError::GLError(ref err) => err.description(),
        }
    }
}

#[cfg(feature = "gl")]
impl From<GLError> for BackendError {
    fn from(err: GLError) -> BackendError {
        match err {
            GLError::NulError(err) => BackendError::NulError(err),
            GLError::Io(err) => BackendError::Io(err),
            GLError::FromUtf8Error(err) => BackendError::FromUtf8Error(err),
            GLError::PoisonError => BackendError::PoisonError,
            _ => BackendError::GLError(err)
        }
    }
}