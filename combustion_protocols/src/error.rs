use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

use trace_error::TraceResult;

use capnp::{Error as CapnpError, NotInSchema};

pub type ProtocolResult<T> = TraceResult<T, ProtocolError>;

#[derive(Debug)]
pub enum ProtocolError {
    Unsupported,
    InvalidLength,
    InvalidFormat,
    NotPresent,
    Other(&'static str),
    CapnpError(CapnpError),
    NotInSchema(NotInSchema),
}

impl Display for ProtocolError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str(self.description())
    }
}

impl Error for ProtocolError {
    fn description(&self) -> &str {
        match *self {
            ProtocolError::Unsupported => "Unsupported protocol",
            ProtocolError::InvalidLength => "Length of data is invalid",
            ProtocolError::InvalidFormat => "Invalid format",
            ProtocolError::NotPresent => "Value is not present",
            ProtocolError::Other(description) => description,
            ProtocolError::CapnpError(ref err) => err.description(),
            ProtocolError::NotInSchema(ref err) => err.description(),
        }
    }
}

impl From<CapnpError> for ProtocolError {
    fn from(err: CapnpError) -> ProtocolError {
        ProtocolError::CapnpError(err)
    }
}

impl From<NotInSchema> for ProtocolError {
    fn from(err: NotInSchema) -> ProtocolError {
        ProtocolError::NotInSchema(err)
    }
}