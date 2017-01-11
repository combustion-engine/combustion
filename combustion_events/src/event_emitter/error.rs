use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::any::Any;

use common::error::*;

pub type EventResult<T> = TraceResult<T, EventError>;

#[derive(Debug)]
pub enum EventError {
    Any(Box<Any>),
    Unspecified,
}

impl Display for EventError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult{
        write!(f, "{}", self.description())
    }
}

impl Error for EventError {
    fn description(&self) -> &str {
        match *self {
            EventError::Any(_) => "Any error",
            EventError::Unspecified => "Unspecified error",
        }
    }
}