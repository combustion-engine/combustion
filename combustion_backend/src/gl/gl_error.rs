use super::bindings::types::*;
use super::bindings::*;

use std::error::Error;
use std::io;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ffi::{NulError};
use std::string::FromUtf8Error;
use std::sync::atomic::{Ordering, AtomicBool, ATOMIC_BOOL_INIT};

use image::ImageError;

pub type GLResult<T> = Result<T, GLError>;

/// `GLError` represents all errors that could be encountered while handling OpenGL calls,
/// including pointer errors, UTF-8 errors, and IO errors.
#[derive(Debug)]
pub enum GLError {
    //OpenGL errors
    InvalidEnum,
    InvalidValue,
    InvalidOperation,
    StackOverflow,
    StackUnderflow,
    OutOfMemory,
    InvalidFramebufferOperation,
    ContextLost,
    //Rust errors
    UnknownError(GLenum),
    NulError(NulError),
    Io(io::Error),
    FromUtf8Error(FromUtf8Error),
    //Errors from the `image` library
    Image(ImageError),
    //Errors from this program
    MissingScene,
    Unsupported,
    IncompleteFramebuffer,
}

static mut CHECK_DISABLED: AtomicBool = ATOMIC_BOOL_INIT;

#[macro_export]
macro_rules! check_gl_errors {
    () => {if let Err(err) = GLError::check() {
        error!("GLError ({:?})", err);
        return Err(err.into());
    }};

    ($ret:expr) => { if let Err(err) = GLError::check() {
        error!("GLError ({:?})", err);
        return Err(err.into());
   } else {
        $ret
   }};
}

impl GLError {
    /// Check if there are any errors in the OpenGL error queue
    ///
    /// If check was disabled, this functions returns `Ok(())` immediately
    pub fn check() -> GLResult<()> {
        if unsafe { !CHECK_DISABLED.load(Ordering::SeqCst) } {
            //Get last error from OpenGL
            let err = unsafe { GetError() };

            //If there was an error, match it to the known OpenGL error kinds
            if err != NO_ERROR {
                return Err(match err {
                    INVALID_ENUM => GLError::InvalidEnum,
                    INVALID_VALUE => GLError::InvalidValue,
                    INVALID_OPERATION => GLError::InvalidOperation,
                    STACK_OVERFLOW => GLError::StackOverflow,
                    STACK_UNDERFLOW => GLError::StackUnderflow,
                    OUT_OF_MEMORY => GLError::OutOfMemory,
                    INVALID_FRAMEBUFFER_OPERATION => GLError::InvalidFramebufferOperation,
                    CONTEXT_LOST => GLError::ContextLost,
                    _ => GLError::UnknownError(err)
                });
            }
        }

        Ok(())
    }

    /// Disable the `check` function, causing it to return `Ok(())` instantly every time.
    ///
    /// The only real reason to do this is to improve performance in very hot loops,
    /// just don't forget to re-enable it and check as soon as possible.
    pub unsafe fn disable_check() {
        CHECK_DISABLED.store(true, Ordering::SeqCst);
    }

    /// Enable the `check` function, resuming its normal behavior after it had been disabled
    pub unsafe fn enable_check() {
        CHECK_DISABLED.store(false, Ordering::SeqCst);
    }
}

impl From<NulError> for GLError {
    fn from(err: NulError) -> GLError {
        GLError::NulError(err)
    }
}

impl From<io::Error> for GLError {
    fn from(err: io::Error) -> GLError {
        GLError::Io(err)
    }
}

impl From<FromUtf8Error> for GLError {
    fn from(err: FromUtf8Error) -> GLError {
        GLError::FromUtf8Error(err)
    }
}

impl From<ImageError> for GLError {
    fn from(err: ImageError) -> GLError {
        //Forward the IO error directly into the GLError
        if let ImageError::IoError(io_err) = err {
            GLError::Io(io_err)
        } else {
            GLError::Image(err)
        }
    }
}

impl From<GLError> for io::Error {
    fn from(err: GLError) -> io::Error {
        match err {
            GLError::Io(io_err) => io_err,
            _ => io::Error::new(io::ErrorKind::Other, err)
        }
    }
}

impl Display for GLError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.description())
    }
}

impl Error for GLError {
    fn description(&self) -> &str {
        match *self {
            GLError::NulError(ref err) => err.description(),
            GLError::Io(ref err) => err.description(),
            GLError::FromUtf8Error(ref err) => err.description(),
            GLError::InvalidEnum => "Invalid Enum",
            GLError::InvalidValue => "Invalid Value",
            GLError::InvalidOperation => "Invalid Operation",
            GLError::StackOverflow => "Stack Overflow",
            GLError::StackUnderflow => "Stack Underflow",
            GLError::OutOfMemory => "Out of Memory",
            GLError::InvalidFramebufferOperation => "Invalid Framebuffer Operation",
            GLError::ContextLost => "GPU Context Lost",
            GLError::UnknownError(_) => "Unknown Error",
            GLError::MissingScene => "Missing scene",
            GLError::Image(ref err) => err.description(),
            GLError::Unsupported => "Unsupported",
            GLError::IncompleteFramebuffer => "Incomplete Framebuffer"
        }
    }
}