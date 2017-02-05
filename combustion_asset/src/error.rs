//! Error handling

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io;

use trace_error::TraceResult;

use image::ImageError;

use capnp::Error as CapnpError;

use protocols::error::ProtocolError;

/// Result type for assets
pub type AssetResult<T> = TraceResult<T, AssetError>;

/// Asset error variants
#[derive(Debug)]
pub enum AssetError {
    /// Protocol error
    ProtocolError(ProtocolError),
    /// Cap'N Proto error
    CapnpError(CapnpError),
    /// Image error
    ImageError(ImageError),
    /// I/O error
    Io(io::Error),
    /// Unsupported medium error
    UnsupportedMedium,
    /// Invalid value error
    InvalidValue,
    /// Unimplemented feature
    Unimplemented(&'static str),
}

impl Display for AssetError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            AssetError::Unimplemented(feature) => write!(f, "Unimplemented {}", feature),
            _ => f.write_str(self.description()),
        }
    }
}

impl Error for AssetError {
    fn description(&self) -> &str {
        match *self {
            AssetError::ProtocolError(ref err) => err.description(),
            AssetError::CapnpError(ref err) => err.description(),
            AssetError::ImageError(ref err) => err.description(),
            AssetError::Io(ref err) => err.description(),
            AssetError::UnsupportedMedium => "Unsupported Asset Medium",
            AssetError::InvalidValue => "Invalid Value",
            AssetError::Unimplemented(_) => "Unimplemented",
        }
    }
}

impl From<ProtocolError> for AssetError {
    fn from(err: ProtocolError) -> AssetError {
        match err {
            ProtocolError::CapnpError(err) => AssetError::CapnpError(err),
            _ => AssetError::ProtocolError(err)
        }
    }
}

impl From<ImageError> for AssetError {
    fn from(err: ImageError) -> AssetError {
        match err {
            ImageError::IoError(err) => AssetError::Io(err),
            _ => AssetError::ImageError(err)
        }
    }
}

impl From<io::Error> for AssetError {
    fn from(err: io::Error) -> AssetError {
        AssetError::Io(err)
    }
}

impl From<CapnpError> for AssetError {
    fn from(err: CapnpError) -> AssetError {
        AssetError::CapnpError(err)
    }
}