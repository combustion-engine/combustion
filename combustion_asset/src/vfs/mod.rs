use std::io::prelude::*;
use std::io;
use std::path::Path;
use std::fmt::Debug;

pub mod default;

pub trait Stream: Debug + Read + Seek + Write + 'static {}

impl<T> Stream for T where T: Debug + Read + Seek + Write + 'static {}

pub type BoxedStream = Box<Stream>;

/// Options to open a data stream with
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct OpenOptions {
    /// Read capability
    pub read: bool,
    /// Write capability
    pub write: bool,
    /// Create an entry in the filesystem if one does not already exist
    ///
    /// This usually requires the `write` property to be set to `true`
    pub create: bool,
    /// Appends data to the stream when writing to it
    pub append: bool,
    /// Destroy the previous entry data before writing
    pub truncate: bool,
    /// Overwrite any existing entry with a new one
    pub create_new: bool,
}

/// Represents a virtual filesystem that can open read/write streams
///
/// It doesn't matter if the stream came from the real disk filesystem, or from
/// inside a TAR archive, or even over the network, this provides a uniform interface
/// for opening them.
///
/// Additionally functionality may be added in the future, such as querying for entry metadata
/// or creating directories.
pub trait VirtualFS: Debug + Send + Sync + 'static {
    /// Open a read stream
    fn open(&self, path: &Path) -> io::Result<BoxedStream> {
        self.open_with(path, OpenOptions {
            read: true,
            ..Default::default()
        })
    }

    /// Open a read/write stream
    fn open_write(&self, path: &Path) -> io::Result<BoxedStream> {
        self.open_with(path, OpenOptions {
            read: true,
            write: true,
            ..Default::default()
        })
    }

    /// Open a read/write stream and create the entry if one does not exist
    fn open_or_create(&self, path: &Path) -> io::Result<BoxedStream> {
        self.open_with(path, OpenOptions {
            read: true,
            write: true,
            create: true,
            ..Default::default()
        })
    }

    /// Open a stream with the given `OpenOptions`
    fn open_with(&self, path: &Path, options: OpenOptions) -> io::Result<BoxedStream>;
}

/// A Boxed `VirtualFS` instance
pub type BoxedFS = Box<VirtualFS + Send + Sync>;