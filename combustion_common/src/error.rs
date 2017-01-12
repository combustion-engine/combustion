//! Error utilities
//!
//! `Trace` and `TraceResult` should usually be used in place of `Result` using the macros
//! `throw!`, `try_throw!`, and `try_rethrow!`
//!
//! Example:
//!
//! ```no_run
//! #[macro_use]
//! extern crate combustion_common as common;
//!
//! use std::error::Error;
//! use std::fmt::{Display, Formatter, Result as FmtResult};
//! use std::io;
//! use std::fs::File;
//!
//! use common::error::*;
//!
//! pub type MyResultType<T> = TraceResult<T, MyErrorType>;
//!
//! #[derive(Debug)]
//! pub enum MyErrorType {
//!     Io(io::Error),
//!     ErrorOne,
//!     ErrorTwo,
//!     //etc
//! }
//!
//! impl Display for MyErrorType {
//!     fn fmt(&self, f: &mut Formatter) -> FmtResult {
//!         write!(f, "{}", self.description())
//!     }
//! }
//!
//! impl Error for MyErrorType {
//!     fn description(&self) -> &str {
//!         match *self {
//!             MyErrorType::Io(ref err) => err.description(),
//!             MyErrorType::ErrorOne => "Error One",
//!             MyErrorType::ErrorTwo => "Error Two",
//!         }
//!     }
//! }
//!
//! impl From<io::Error> for MyErrorType {
//!     fn from(err: io::Error) -> MyErrorType {
//!         MyErrorType::Io(err)
//!     }
//! }
//!
//! fn basic() -> MyResultType<i32> {
//!     //Something may throw
//!     throw!(MyErrorType::ErrorOne);
//!
//!     // Or return an Ok value
//!     Ok(42)
//! }
//!
//! fn example() -> MyResultType<()> {
//!     // Note the use of try_rethrow! for TraceResult results
//!     let meaning = try_rethrow!(basic());
//!
//!     // Prints 42 if `basic` succeeds
//!     println!("{}", meaning);
//!
//!     // Note the use of try_throw! for non-TraceResult results
//!     let some_file = try_throw!(File::open("example.txt"));
//!
//!     Ok(())
//! }
//!
//! fn main() {
//!     match example() {
//!         Ok(_) => println!("Success!"),
//!         // Here, err is the Trace<E>, which can be printed normally,
//!         // showing both the error and the backtrace.
//!         Err(err) => println!("Error: {}", err)
//!     }
//! }
//!
//! ```

use std::error::Error;
use std::ops::Deref;
use std::fmt::{Display, Formatter, Result as FmtResult};

use backtrace::{BacktraceFmt, DefaultBacktraceFmt, SourceBacktrace};

/// Alias to aid in usage with `Result`
pub type TraceResult<T, E> = Result<T, Trace<E>>;

/// Trace error that encapsulates a backtrace alongside an error value.
///
/// Trace itself does not implement `Error`, so they cannot be nested.
#[derive(Debug)]
pub struct Trace<E: Error> {
    error: E,
    backtrace: Box<SourceBacktrace>,
}

impl<E: Error> Trace<E> {
    /// Creates a new `Trace` from the given error and backtrace
    #[inline]
    pub fn new(error: E, backtrace: Box<SourceBacktrace>) -> Trace<E> {
        Trace { error: error, backtrace: backtrace }
    }

    /// Consume self and return the inner error value
    #[inline]
    pub fn into_error(self) -> E {
        self.error
    }

    /// Get a reference to the inner backtrace
    #[inline]
    pub fn backtrace(&self) -> &SourceBacktrace {
        &*self.backtrace
    }

    /// Format the error and backtrace
    pub fn format<Fmt: BacktraceFmt>(&self, header: bool, reverse: bool) -> String {
        format!("{}\n{}", self.error, self.backtrace.format::<Fmt>(header, reverse))
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

#[cfg(feature = "parallel")]
pub mod parallel {
    //! Extensions and type aliases to make parallel and future operations easier with traces

    use futures::Future;
    use super::*;

    /// Future type with a `Trace<E>` error
    pub type TraceFuture<T, E> = Future<Item = T, Error = Trace<E>>;
}

/// Creates a new `Result::Err(Trace<E>)` and immediately returns it
#[macro_export]
macro_rules! throw {
    ($err:expr) => {
        return ::std::result::Result::Err($crate::error::Trace::new(
            ::std::convert::From::from($err),
            ::std::boxed::Box::new($crate::backtrace::SourceBacktrace::new(line!(), file!()))
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
pub fn _assert_trace_result<T, E: Error>(res: TraceResult<T, E>) -> TraceResult<T, E> {
    res
}

/// Like `try_throw!`, but designed for `TraceResult`s, as it keeps the previous trace.
///
/// This macro will try to call `Trace::convert` on the trace to convert the inner error if necessary,
/// similarly to `try!`
#[macro_export]
macro_rules! try_rethrow {
    ($res:expr) => (match $crate::error::_assert_trace_result($res) {
        ::std::result::Result::Ok(val) => val,
        ::std::result::Result::Err(err) => {
            return ::std::result::Result::Err(err.convert())
        }
    })
}