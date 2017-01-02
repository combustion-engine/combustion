use std::error::Error;
use std::any::{TypeId, Any};
use std::ffi::NulError;
use std::str::Utf8Error;
use std::string::FromUtf8Error;
use std::sync::PoisonError;
use std::io;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub use ::common::error::*;

use ::backend::gl::GLError;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    GLError(GLError),
    Io(io::Error),
    FromUtf8Error(FromUtf8Error),
    Utf8Error(Utf8Error),
    NulError(NulError),
    PoisonError(TypeId, Box<Error + 'static>),
    InvalidScene
}

impl From<GLError> for AppError {
    fn from(err: GLError) -> AppError {
        match err {
            GLError::Io(err) => AppError::Io(err),
            GLError::FromUtf8Error(err) => AppError::FromUtf8Error(err),
            GLError::NulError(err) => AppError::NulError(err),
            _ => AppError::GLError(err)
        }
    }
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> AppError {
        AppError::Io(err)
    }
}

impl<T: 'static> From<PoisonError<T>> for AppError {
    fn from(err: PoisonError<T>) -> AppError {
        AppError::PoisonError(TypeId::of::<T>(), Box::from(err))
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.description())
    }
}

impl Error for AppError {
    fn description(&self) -> &str {
        match *self {
            AppError::GLError(ref err) => err.description(),
            AppError::Io(ref err) => err.description(),
            AppError::Utf8Error(ref err) => err.description(),
            AppError::FromUtf8Error(ref err) => err.description(),
            AppError::NulError(ref err) => err.description(),
            AppError::PoisonError(_, ref err) => err.description(),
            AppError::InvalidScene => "Invalid Scene",
        }
    }
}