//! Error routines for logging systems

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io;

use trace_error::TraceResult;

/// Convenience type for logger results
pub type LogResult<T> = TraceResult<T, LogError>;

/// Logger errors
#[derive(Debug)]
pub enum LogError {
    /// I/O Errors
    Io(io::Error)
}

impl Display for LogError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str(self.description())
    }
}

impl Error for LogError {
    fn description(&self) -> &str {
        match *self {
            LogError::Io(ref err) => err.description(),
        }
    }
}

impl From<io::Error> for LogError {
    fn from(err: io::Error) -> LogError {
        LogError::Io(err)
    }
}