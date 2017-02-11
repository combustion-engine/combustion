//! `ReadOnlyStream` adapter

use std::io::prelude::*;
use std::io::{self, SeekFrom};
use std::ops::{Deref, DerefMut};

/// This defines a stream adapter that adds `Write` capability to a read-only stream.
///
/// It does this by totally ignoring all writes. Try to avoid using it if possible.
pub struct ReadOnlyStream<R>(R) where R: Seek + Read + 'static;

impl<R> ReadOnlyStream<R> where R: Seek + Read + 'static {
    /// Create a new `ReadOnlyStream` from the given stream.
    pub fn new(stream: R) -> ReadOnlyStream<R> {
        ReadOnlyStream(stream)
    }

    /// Gets a reference to the underlying reader.
    pub fn get_ref(&self) -> &R {
        &self.0
    }

    /// Gets a mutable reference to the underlying reader.
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.0
    }

    /// Unwraps the `ReadOnlyStream` and returns the underlying reader.
    pub fn into_inner(self) -> R {
        self.0
    }
}

impl<R> Deref for ReadOnlyStream<R> where R: Seek + Read + 'static {
    type Target = R;

    fn deref(&self) -> &R {
        &self.0
    }
}

impl<R> DerefMut for ReadOnlyStream<R> where R: Seek + Read + 'static {
    fn deref_mut(&mut self) -> &mut R {
        &mut self.0
    }
}

impl<R> Read for ReadOnlyStream<R> where R: Seek + Read + 'static {
    #[inline(always)]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

impl<R> Seek for ReadOnlyStream<R> where R: Seek + Read + 'static {
    #[inline(always)]
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.0.seek(pos)
    }
}

impl<R> Write for ReadOnlyStream<R> where R: Seek + Read + 'static {
    #[inline(always)]
    fn write(&mut self, _: &[u8]) -> io::Result<usize> {
        Ok(0)
    }

    #[inline(always)]
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<R> BufRead for ReadOnlyStream<R> where R: Seek + Read + BufRead + 'static {
    #[inline(always)]
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.0.fill_buf()
    }

    #[inline(always)]
    fn consume(&mut self, amt: usize) {
        self.0.consume(amt)
    }
}

impl<R> AsRef<R> for ReadOnlyStream<R> where R: Seek + Read + 'static {
    fn as_ref(&self) -> &R {
        self.get_ref()
    }
}

impl<R> AsMut<R> for ReadOnlyStream<R> where R: Seek + Read + 'static {
    fn as_mut(&mut self) -> &mut R {
        self.get_mut()
    }
}