//! `LazyBuffer` stream adapter

use std::io::prelude::*;
use std::io::{self, Cursor, SeekFrom};

use ::num_utils::round_multiple;
use ::streams::utils::copy_bytes;

/// The `LazyBuffer` lazily buffers data from a forward-only reader so it can be seek-ed to.
pub struct LazyBuffer<R: Read> {
    /// Underlying read stream
    reader: R,
    /// Flag for if the underlying read stream has reached its end
    complete: bool,
    /// `Read + Write + Seek` buffer
    buffer: Cursor<Vec<u8>>,
}

impl<R: Read> LazyBuffer<R> {
    /// Create a new `LazyBuffer` from a read-only stream
    pub fn new(reader: R) -> LazyBuffer<R> {
        LazyBuffer {
            reader: reader,
            complete: false,
            buffer: Cursor::new(Vec::new()),
        }
    }

    /// Consumes self and returns the inner reader
    pub fn into_inner(self) -> R {
        self.reader
    }

    /// Get reference to underlying reader
    pub fn get_ref(&self) -> &R {
        &self.reader
    }

    /// Get mutable reference to underlying reader
    ///
    /// It is **NOT** recommended to `read` from the underlying reader
    /// while it is inside the `LazyBuffer`. It will cause undefined behavior
    /// and data loss from the stream.
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.reader
    }

    /// Checks if the underlying reader has been fully buffered
    pub fn is_complete(&self) -> bool {
        self.complete
    }
}

impl<R: Read> Read for LazyBuffer<R> {
    fn read(&mut self, mut buffer: &mut [u8]) -> io::Result<usize> {
        let current_pos = self.buffer.position();
        let current_len = self.buffer.get_ref().len() as u64;

        let buffer_len = buffer.len() as u64;

        let until_end = current_len - current_pos;

        // If the internal buffer has enough left to read from,
        // or the underlying reader, read from the buffer
        if buffer_len <= until_end || self.complete {
            self.buffer.read(buffer)
        } else {
            // Seek to the required position to buffer the next data
            self.seek(SeekFrom::Start(current_len + until_end))?;

            // Seek back to the current position
            self.buffer.set_position(current_pos);

            self.buffer.read(buffer)
        }
    }
}

impl<R: Read> Seek for LazyBuffer<R> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        if self.complete {
            // If the buffering has been complete, new data cannot be read in, so there is no point in trying. Just seek internally.
            self.buffer.seek(pos)
        } else {
            match pos {
                // For Start positions, additional data only needs to be read in if the internal buffer has a
                // smaller length than the absolute position being seek-ed to
                SeekFrom::Start(position) => {
                    let len = self.buffer.get_ref().len() as u64;

                    // If the desired seek position is larger than we have, read in more data
                    if len < position {
                        // Round up to some multiple of 8 for a good even number of bytes
                        let bytes = round_multiple(position - len, 8);

                        self.buffer.seek(SeekFrom::End(0))?;

                        let copied = copy_bytes(&mut self.reader, &mut self.buffer, bytes as usize)? as u64;

                        if copied < bytes {
                            // Everything available has been read in
                            self.complete = true;
                        }
                    }

                    self.buffer.set_position(position);

                    Ok(position)
                },
                SeekFrom::Current(current_offset) => {
                    if current_offset <= 0 {
                        // For relative offset seek positions, negative and zero seeks can be handled internally
                        self.buffer.seek(SeekFrom::Current(current_offset))
                    } else {
                        let current_pos = self.buffer.position();

                        self.seek(SeekFrom::Start(current_pos + current_offset as u64))
                    }
                },
                // If we're seeking to the end, we have to read in the entire contents
                SeekFrom::End(end_offset) => {
                    let mut buffer = Vec::new();

                    // Read in remaining data
                    self.reader.read_to_end(&mut buffer)?;

                    // Just append it onto the internal buffer
                    self.buffer.get_mut().extend_from_slice(&buffer);

                    // Everything has been read in
                    self.complete = true;

                    // Seek the internal buffer
                    self.buffer.seek(SeekFrom::End(end_offset))
                }
            }
        }
    }
}