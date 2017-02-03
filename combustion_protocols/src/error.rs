use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::Utf8Error;

use trace_error::TraceResult;

use capnp::{Error as CapnpError, NotInSchema};
use base64::Base64Error;

pub type ProtocolResult<T> = TraceResult<T, ProtocolError>;

#[derive(Debug)]
pub enum ProtocolError {
    Unsupported,
    InvalidLength,
    InvalidFormat,
    NotPresent,
    Utf8Error(Utf8Error),
    Other(&'static str),
    CapnpError(CapnpError),
    NotInSchema(NotInSchema),
    Base64Error(Base64Error),
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
            ProtocolError::Utf8Error(ref err) => err.description(),
            ProtocolError::Other(description) => description,
            ProtocolError::CapnpError(ref err) => err.description(),
            ProtocolError::NotInSchema(ref err) => err.description(),
            ProtocolError::Base64Error(ref err) => err.description(),
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

impl From<Base64Error> for ProtocolError {
    fn from(err: Base64Error) -> ProtocolError {
        match err {
            Base64Error::Utf8(err) => ProtocolError::Utf8Error(err),
            _ => ProtocolError::Base64Error(err)
        }
    }
}