use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

use trace_error::TraceResult;

use capnp::{Error as CapnpError, NotInSchema};

pub type ProtocolResult<T> = TraceResult<T, ProtocolError>;

#[derive(Debug)]
pub enum ProtocolError {
    Unsupported,
    CapnpError(CapnpError),
    NotInSchema(NotInSchema),
}

impl Display for ProtocolError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.description())
    }
}

impl Error for ProtocolError {
    fn description(&self) -> &str {
        match *self {
            ProtocolError::Unsupported => "Unsupported protocol",
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