use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io;

use trace_error::TraceResult;

use image::ImageError;

use capnp::Error as CapnpError;

use protocols::error::ProtocolError;

pub type AssetResult<T> = TraceResult<T, AssetError>;

#[derive(Debug)]
pub enum AssetError {
    ProtocolError(ProtocolError),
    CapnpError(CapnpError),
    ImageError(ImageError),
    Io(io::Error),
    UnsupportedMedium,
}

impl Display for AssetError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str(self.description())
    }
}

impl Error for AssetError {
    fn description(&self) -> &str {
        match *self {
            AssetError::ProtocolError(ref err) => err.description(),
            AssetError::CapnpError(ref err) => err.description(),
            AssetError::ImageError(ref err) => err.description(),
            AssetError::Io(ref err) => err.description(),
            AssetError::UnsupportedMedium => "Unsupported Asset Medium"
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