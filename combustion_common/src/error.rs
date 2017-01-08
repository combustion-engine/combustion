//! Error utilities
//!
//! `Trace` and `TraceResult` should usually be used in place of `Result` using the macros
//! `throw!`, `try_throw!`, and `try_rethrow!`

use std::sync::Arc;
use std::error::Error;
use std::ops::Deref;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use backtrace::{BacktraceFmt, DefaultBacktraceFmt, LineBacktrace};

use tinyfiledialogs::*;

pub type TraceResult<T, E> = Result<T, Trace<E>>;

/// Trace error that encapsulates a backtrace alongside an error value.
///
/// Trace itself does not implement `Error`, so they cannot be nested.
#[derive(Debug)]
pub struct Trace<E: Error> {
    error: E,
    backtrace: Arc<LineBacktrace>,
}

impl<E: Error> Trace<E> {
    /// Creates a new `Trace` from the given error and backtrace
    #[inline]
    pub fn new(error: E, backtrace: Arc<LineBacktrace>) -> Trace<E> {
        Trace { error: error, backtrace: backtrace }
    }

    /// Consume self and return the inner error value
    #[inline]
    pub fn into_error(self) -> E {
        self.error
    }

    /// Get a reference to the inner backtrace
    #[inline]
    pub fn backtrace(&self) -> &LineBacktrace {
        &*self.backtrace
    }

    /// Format the error and backtrace
    pub fn format<Fmt: BacktraceFmt>(&self, header: bool, reverse: bool) -> String {
        format!("{}\n{}", self.description(), self.backtrace.format::<Fmt>(header, reverse))
    }

    /// Convert the inner error of type `E` into type `O`
    #[inline]
    pub fn convert<O: Error>(self) -> Trace<O> where O: From<E> {
        Trace {
            error: From::from(self.error),
            backtrace: self.backtrace
        }
    }
}

impl<E: Error> Deref for Trace<E> {
    type Target = E;

    #[inline]
    fn deref(&self) -> &E {
        &self.error
    }
}

impl<E: Error> Display for Trace<E> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.format::<DefaultBacktraceFmt>(true, false))
    }
}

/// Creates a new `Result::Err(Trace<E>)` and immediately returns it
#[macro_export]
macro_rules! throw {
    ($err:expr) => {
        return ::std::result::Result::Err($crate::error::Trace::new(
            ::std::convert::From::from($err),
            ::std::sync::Arc::new($crate::backtrace::LineBacktrace::new(line!(), file!()))
        ))
    }
}

/// Like `try!`, but invokes `throw!` on the error value if it exists, converting it to `Result::Err(Trace<E>)`
///
/// Note that the backtrace will only go as far as the location this macro was invoked
///
/// This macro will try to call `From::from` on the error to convert it if necessary, just like `try!`
#[macro_export]
macro_rules! try_throw {
    ($res:expr) => (match $res {
        ::std::result::Result::Ok(val) => val,
        ::std::result::Result::Err(err) => { throw!(err) }
    })
}

#[doc(hidden)]
#[inline(always)]
pub fn _assert_traceable_result<T, E: Error>(res: TraceResult<T, E>) -> TraceResult<T, E> {
    res
}

/// Like `try_throw!`, but designed for expression that are `TraceResult`s already,
/// as it keeps the previous trace.
///
/// This macro will try to call `From::from` on the error to convert it if necessary, just like `try!`
#[macro_export]
macro_rules! try_rethrow {
    ($res:expr) => (match $crate::error::_assert_traceable_result($res) {
        ::std::result::Result::Ok(val) => val,
        ::std::result::Result::Err(traceable_err) => {
            return ::std::result::Result::Err(traceable_err.convert())
        }
    })
}

#[inline(never)]
#[cold]
fn unwrap_failed<E: Debug>(msg: &str, error: E) -> ! {
    panic!("{}: {:?}", msg, error)
}

#[inline(never)]
#[cold]
fn expect_failed(msg: &str) -> ! {
    panic!("{}", msg)
}

pub trait ResultExt<T, E> {
    fn expect_logged(self, msg: &str) -> T;
    fn expect_logged_box(self, msg: &str) -> T;
}

impl<T, E> ResultExt<T, E> for Result<T, E> where E: Debug {
    #[inline]
    fn expect_logged(self, msg: &str) -> T {
        match self {
            Ok(t) => t,
            Err(e) => {
                error!("{}\n\n\nDetails:\n\n{:?}", msg, e);
                unwrap_failed(msg, e)
            },
        }
    }

    #[inline]
    fn expect_logged_box(self, msg: &str) -> T {
        match self {
            Ok(t) => t,
            Err(e) => {
                let formatted = format!("{}\n\n\nDetails:\n\n{:?}", msg, e);
                error!(formatted);
                message_box(MessageBox::Ok, "Combustion Error", formatted.as_str(), Some(Icon::Error), None);
                unwrap_failed(msg, e)
            },
        }
    }
}

pub trait OptionExt<T> {
    fn expect_logged(self, msg: &str) -> T;
    fn expect_logged_box(self, msg: &str) -> T;
}

impl<T> OptionExt<T> for Option<T> {
    #[inline]
    fn expect_logged(self, msg: &str) -> T {
        match self {
            Some(val) => val,
            None => {
                error!(msg);
                expect_failed(msg)
            },
        }
    }

    #[inline]
    fn expect_logged_box(self, msg: &str) -> T {
        match self {
            Some(val) => val,
            None => {
                error!(msg);
                let _ = message_box(MessageBox::Ok, "Combustion Error", msg, Some(Icon::Error), Some(BoxButton::OkYes));
                expect_failed(msg)
            },
        }
    }
}