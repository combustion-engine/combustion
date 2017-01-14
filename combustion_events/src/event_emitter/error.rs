//! Error utils for event emitters

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::any::Any;
use std::sync::PoisonError;

use common::error::*;

/// Shorthand result type for `EventError`s
pub type EventResult<T> = TraceResult<T, EventError>;

/// Error variants for event emitters
#[derive(Debug)]
pub enum EventError {
    /// Any error, literally. Since listener callbacks are type-erased,
    /// this is the best we can do for now.
    Any(Box<Any>),
    /// Unspecified error. Usually used as a placeholder for other errors.
    Unspecified,
    /// Converted from a `PoisonError<T>` to remove the `T`.
    ///
    /// Still means the same thing, that a lock was poisoned by a panicking thread.
    PoisonError,
}

unsafe impl Send for EventError {}

impl Display for EventError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.description())
    }
}

impl Error for EventError {
    fn description(&self) -> &str {
        match *self {
            EventError::Any(_) => "Any error",
            EventError::Unspecified => "Unspecified error",
            EventError::PoisonError => "Poison Error",
        }
    }
}

impl<T> From<PoisonError<T>> for EventError {
    fn from(_: PoisonError<T>) -> EventError {
        EventError::PoisonError
    }
}