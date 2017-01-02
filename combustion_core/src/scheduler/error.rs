use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::thread;
use std::io;

pub type SchedulerResult<T> = Result<T, SchedulerError>;

#[derive(Debug)]
pub enum SchedulerError {
    Io(io::Error),
}

impl From<io::Error> for SchedulerError {
    fn from(err: io::Error) -> SchedulerError {
        SchedulerError::Io(err)
    }
}

impl Display for SchedulerError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.description())
    }
}

impl Error for SchedulerError {
    fn description(&self) -> &str {
        match *self {
            SchedulerError::Io(ref err) => err.description(),
        }
    }
}