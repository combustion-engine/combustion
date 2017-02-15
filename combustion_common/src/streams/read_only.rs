//! `ReadOnlySink` adapter

use std::io::prelude::*;
use std::io::{self, SeekFrom};

/// This defines a stream adapter that adds `Write` capability to a read-only stream.
///
/// It does this by totally ignoring all writes. Try to avoid using it if possible.
///
/// This is loosely modeled after `std::io::Sink`, but with `Read + Seek` capabilities.
#[derive(Debug)]
pub struct ReadOnlySink<R>(R) where R: Seek + Read + 'static;

impl<R> ReadOnlySink<R> where R: Seek + Read + 'static {
    /// Create a new `ReadOnlySink` from the given stream.
    pub fn new(stream: R) -> ReadOnlySink<R> {
        ReadOnlySink(stream)
    }

    /// Gets a reference to the underlying reader.
    pub fn get_ref(&self) -> &R {
        &self.0
    }

    /// Gets a mutable reference to the underlying reader.
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.0
    }

    /// Unwraps the `ReadOnlySink` and returns the underlying reader.
    pub fn into_inner(self) -> R {
        self.0
    }
}

impl<R> Read for ReadOnlySink<R> where R: Seek + Read + 'static {
    #[inline(always)]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

impl<R> Seek for ReadOnlySink<R> where R: Seek + Read + 'static {
    #[inline(always)]
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.0.seek(pos)
    }
}

impl<R> Write for ReadOnlySink<R> where R: Seek + Read + 'static {
    #[inline(always)]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Ok(buf.len())
    }

    #[inline(always)]
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<R> BufRead for ReadOnlySink<R> where R: Seek + Read + BufRead + 'static {
    #[inline(always)]
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.0.fill_buf()
    }

    #[inline(always)]
    fn consume(&mut self, amt: usize) {
        self.0.consume(amt)
    }
}