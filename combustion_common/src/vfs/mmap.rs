//! Memory mapped buffers as a virtual filesystem

use std::io;
use std::fs;
use std::path::Path;

use memmap;

use ::streams::{BoxedStream, ReadOnlySink};

use super::{VirtualFS, BoxedMetadata, OpenOptions};
use super::default::DefaultMetadata;

/// Read-only memory mapped buffer virtual filesystem
#[derive(Debug, Clone, Copy)]
pub struct MmapFS;

// Internal stream representation which holds the Mmap instance and the associated file handle
#[derive(Debug)]
struct MmapStream {
    file: fs::File,
    mmap: memmap::Mmap,
}

// So the MmapStream can be used in an io::Cursor,
// AsRef is implemented for it which accesses the internal buffer
impl AsRef<[u8]> for MmapStream {
    fn as_ref(&self) -> &[u8] {
        unsafe { self.mmap.as_slice() }
    }
}

impl VirtualFS for MmapFS {
    fn open_with(&self, path: &Path, options: OpenOptions) -> io::Result<BoxedStream> {
        //TODO: Add Write functionality
        if options.write || options.append || options.create || options.create_new {
            return Err(io::Error::new(io::ErrorKind::Other,
                                      "Cannot open write streams for memory mapped files at this time"));
        }

        let file = fs::OpenOptions::new().read(true).open(path)?;

        //let protection = if options.write {
        //    memmap::Protection::ReadWrite
        //} else {
        //    memmap::Protection::Read
        //};

        let mmap = memmap::Mmap::open(&file, memmap::Protection::Read)?;

        let stream = ReadOnlySink::new(io::Cursor::new(MmapStream {
            file: file,
            mmap: mmap,
        }));

        Ok(Box::new(stream))
    }

    fn metadata(&self, path: &Path) -> io::Result<BoxedMetadata> {
        fs::metadata(path).map(|metadata| Box::new(DefaultMetadata(metadata)) as BoxedMetadata)
    }
}