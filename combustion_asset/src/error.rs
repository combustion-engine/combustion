//! Error handling

use std::error::Error;
use std::ffi::NulError;
use std::str::Utf8Error;
use std::string::FromUtf8Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io;

use trace_error::TraceResult;

#[cfg(feature = "flate2")]
use flate2;
#[cfg(feature = "zip")]
use zip;

#[cfg(feature = "json")]
use json;
#[cfg(feature = "yaml")]
use yaml;
#[cfg(feature = "bincode")]
use bincode;
#[cfg(feature = "cbor")]
use cbor;
#[cfg(feature = "toml")]
use toml;

use image::ImageError;

use capnp::Error as CapnpError;

use assimp::error::AiError;

use protocols::error::ProtocolError;

/// Result type for assets
pub type AssetResult<T> = TraceResult<T, AssetError>;

/// Serialization and Deserialization errors for Toml
#[cfg(feature = "toml")]
pub mod toml_error {
    use std::error::Error;
    use std::fmt::{Display, Formatter, Result as FmtResult};

    use toml;

    /// Serialization and Deserialization errors for Toml
    #[derive(Debug)]
    pub enum TomlError {
        /// Toml Deserialization error
        De(toml::de::Error),
        /// Toml Serialization error
        Ser(toml::ser::Error),
    }

    impl Display for TomlError {
        fn fmt(&self, f: &mut Formatter) -> FmtResult {
            match *self {
                TomlError::De(ref de) => de.fmt(f),
                TomlError::Ser(ref ser) => ser.fmt(f),
            }
        }
    }

    impl Error for TomlError {
        fn description(&self) -> &str {
            match *self {
                TomlError::De(ref de) => de.description(),
                TomlError::Ser(ref ser) => ser.description(),
            }
        }
    }
}

/// Asset error variants
#[derive(Debug)]
pub enum AssetError {
    /// Protocol error
    ProtocolError(ProtocolError),
    /// Cap'N Proto error
    CapnpError(CapnpError),
    /// Image error
    ImageError(ImageError),
    /// Assimp error
    AssimpError(AiError),
    /// I/O error
    Io(io::Error),
    /// Unsupported medium error
    UnsupportedMedium,
    /// Invalid value error
    InvalidValue,
    /// Unimplemented feature
    Unimplemented(&'static str),
    /// Some other custom error
    Other(String),
    /// UTF-8 encoding error
    Utf8Error(Utf8Error),
    /// From UTF-8 Error
    FromUtf8Error(FromUtf8Error),
    /// Null error
    NulError(NulError),
    /// Unsupported format of some kind
    UnsupportedFormat,
    /// Flate2 DataError
    #[cfg(feature = "flate2")]
    Flate2DataError(flate2::DataError),
    /// Zip errors
    #[cfg(feature = "zip")]
    ZipError(zip::result::ZipError),
    /// JSON errors
    #[cfg(feature = "json")]
    JsonError(json::Error),
    /// YAML errors
    #[cfg(feature = "yaml")]
    YamlError(yaml::Error),
    /// CBOR errors
    #[cfg(feature = "cbor")]
    CBORError(cbor::Error),
    /// Bincode errors
    #[cfg(feature = "bincode")]
    BincodeError(bincode::Error),
    /// Toml errors
    #[cfg(feature = "toml")]
    TomlError(toml_error::TomlError),
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
            AssetError::Utf8Error(ref err) => err.description(),
            AssetError::FromUtf8Error(ref err) => err.description(),
            AssetError::NulError(ref err) => err.description(),
            AssetError::AssimpError(ref err) => err.description(),
            AssetError::UnsupportedFormat => "Unsupported Format",
            #[cfg(feature = "flate2")]
            AssetError::Flate2DataError(ref err) => err.description(),
            #[cfg(feature = "zip")]
            AssetError::ZipError(ref err) => err.description(),
            #[cfg(feature = "json")]
            AssetError::JsonError(ref err) => err.description(),
            #[cfg(feature = "yaml")]
            AssetError::YamlError(ref err) => err.description(),
            #[cfg(feature = "cbor")]
            AssetError::CBORError(ref err) => err.description(),
            #[cfg(feature = "bincode")]
            AssetError::BincodeError(ref err) => err.description(),
            #[cfg(feature = "toml")]
            AssetError::TomlError(ref err) => err.description(),
            AssetError::Other(ref value) => value.as_str(),
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

impl From<NulError> for AssetError {
    fn from(err: NulError) -> AssetError {
        AssetError::NulError(err)
    }
}

impl From<Utf8Error> for AssetError {
    fn from(err: Utf8Error) -> AssetError {
        AssetError::Utf8Error(err)
    }
}

impl From<FromUtf8Error> for AssetError {
    fn from(err: FromUtf8Error) -> AssetError {
        AssetError::FromUtf8Error(err)
    }
}

impl From<AiError> for AssetError {
    fn from(err: AiError) -> AssetError {
        match err {
            AiError::Utf8Error(err) => AssetError::Utf8Error(err),
            AiError::NulError(err) => AssetError::NulError(err),
            AiError::Io(err) => AssetError::Io(err),
            _ => AssetError::AssimpError(err)
        }
    }
}

#[cfg(feature = "flate2")]
impl From<flate2::DataError> for AssetError {
    fn from(err: flate2::DataError) -> AssetError {
        AssetError::Flate2DataError(err)
    }
}

#[cfg(feature = "zip")]
impl From<zip::result::ZipError> for AssetError {
    fn from(err: zip::result::ZipError) -> AssetError {
        match err {
            zip::result::ZipError::Io(err) => AssetError::Io(err),
            _ => AssetError::ZipError(err)
        }
    }
}

#[cfg(feature = "json")]
impl From<json::Error> for AssetError {
    fn from(err: json::Error) -> AssetError {
        AssetError::JsonError(err)
    }
}

#[cfg(feature = "yaml")]
impl From<yaml::Error> for AssetError {
    fn from(err: yaml::Error) -> AssetError {
        AssetError::YamlError(err)
    }
}

#[cfg(feature = "cbor")]
impl From<cbor::Error> for AssetError {
    fn from(err: cbor::Error) -> AssetError {
        match err {
            cbor::Error::Io(err) => AssetError::Io(err),
            cbor::Error::FromUtf8(err) => AssetError::FromUtf8Error(err),
            _ => AssetError::CBORError(err),
        }
    }
}

#[cfg(feature = "bincode")]
impl From<bincode::Error> for AssetError {
    fn from(err: bincode::Error) -> AssetError {
        AssetError::BincodeError(err)
    }
}

#[cfg(feature = "toml")]
impl From<toml::de::Error> for AssetError {
    fn from(err: toml::de::Error) -> AssetError {
        AssetError::TomlError(toml_error::TomlError::De(err))
    }
}

#[cfg(feature = "toml")]
impl From<toml::ser::Error> for AssetError {
    fn from(err: toml::ser::Error) -> AssetError {
        AssetError::TomlError(toml_error::TomlError::Ser(err))
    }
}