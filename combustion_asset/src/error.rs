use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

use trace_error::TraceResult;

use protocols::error::ProtocolError;

pub type AssetResult<T> = TraceResult<T, AssetError>;

#[derive(Debug)]
pub enum AssetError {
    ProtocolError(ProtocolError),
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
        }
    }
}