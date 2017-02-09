use std::io;
use std::path::Path;
use std::fs;

use super::{VirtualFS, BoxedStream, OpenOptions};

#[derive(Debug)]
pub struct DefaultFS;

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
}