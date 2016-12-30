use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub type SystemResult<T> = Result<T, SystemError>;

#[derive(Debug)]
pub enum SystemError {
    WouldCycle,
    MissingDependentSystem(String),
    DuplicateSystem(String),
}

impl Display for SystemError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.description())
    }
}

impl Error for SystemError {
    fn description(&self) -> &str {
        match *self {
            SystemError::WouldCycle => "Dependency Would Cycle",
            SystemError::MissingDependentSystem(_) => "Missing Dependent System",
            SystemError::DuplicateSystem(_) => "Duplicate System",
        }
    }
}