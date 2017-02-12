//! Default VFS that just uses the real filesystem on the hard disk.

use std::io;
use std::path::Path;
use std::time::SystemTime;
use std::fs;
use std::ops::Deref;

use common::streams::BoxedStream;

use super::{VirtualFS, VirtualMetadata, BoxedMetadata, OpenOptions};

/// Default VFS that just uses the real filesystem on the hard disk
#[derive(Debug, Clone, Copy)]
pub struct DefaultFS;

/// `VirtualMetadata` wrapper for `fs::Metadata`
pub struct DefaultMetadata(pub fs::Metadata);

impl VirtualMetadata for DefaultMetadata {
    fn is_file(&self) -> bool {
        self.0.is_file()
    }

    fn is_dir(&self) -> bool {
        self.0.is_dir()
    }

    fn modified(&self) -> io::Result<SystemTime> {
        self.0.modified()
    }
}

impl Deref for DefaultMetadata {
    type Target = fs::Metadata;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl VirtualFS for DefaultFS {
    fn open_with(&self, path: &Path, options: OpenOptions) -> io::Result<BoxedStream> {
        fs::OpenOptions::new()
            .read(options.read)
            .write(options.write)
            .append(options.append)
            .create(options.create)
            .truncate(options.truncate)
            .create_new(options.create_new)
            .open(path).map(|file| box file as BoxedStream)
    }

    fn metadata(&self, path: &Path) -> io::Result<BoxedMetadata> {
        fs::metadata(path).map(|metadata| box DefaultMetadata(metadata) as BoxedMetadata)
    }
}