//! Null VFS that will not open any streams

use std::io;
use std::path::Path;
use std::time::SystemTime;

use common::streams::BoxedStream;

use super::{VirtualFS, VirtualMetadata, BoxedMetadata, OpenOptions};

/// Null VFS that will not open any streams
#[derive(Debug, Clone, Copy)]
pub struct NullFS;

/// Empty `VirtualMetadata` type for `NullFS`
pub struct NullMetadata;

impl VirtualMetadata for NullMetadata {
    fn is_file(&self) -> bool { false }
    fn is_dir(&self) -> bool { false }
    fn modified(&self) -> io::Result<SystemTime> {
        Ok(SystemTime::now())
    }
}

impl VirtualFS for NullFS {
    fn open_with(&self, _: &Path, _: OpenOptions) -> io::Result<BoxedStream> {
        Err(io::Error::new(io::ErrorKind::NotFound, "Cannot open streams with NullFS"))
    }

    fn metadata(&self, _: &Path) -> io::Result<BoxedMetadata> {
        Err(io::Error::new(io::ErrorKind::NotFound, "Cannot open streams with NullFS"))
    }
}