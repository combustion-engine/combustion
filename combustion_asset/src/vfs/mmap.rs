//! Memory mapped buffers as a virtual filesystem

#![allow(missing_docs)]

use std::io;
use std::fs;
use std::path::Path;

use memmap;

use common::streams::{BoxedStream, ReadOnlySink};

use super::{VirtualFS, BoxedMetadata, OpenOptions};
use super::default::DefaultMetadata;

#[derive(Debug, Clone, Copy)]
pub struct MmapFS;

#[derive(Debug)]
struct MmapStream {
    file: fs::File,
    mmap: memmap::Mmap,
}

impl AsRef<[u8]> for MmapStream {
    fn as_ref(&self) -> &[u8] {
        unsafe { self.mmap.as_slice() }
    }
}

impl VirtualFS for MmapFS {
    fn open_with(&self, path: &Path, options: OpenOptions) -> io::Result<BoxedStream> {
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

        Ok(box stream)
    }

    fn metadata(&self, path: &Path) -> io::Result<BoxedMetadata> {
        fs::metadata(path).map(|metadata| box DefaultMetadata(metadata) as BoxedMetadata)
    }
}