//! I/O Stream misc utilities

use std::io;
use std::io::prelude::*;

/// Attempts to copy an exact number of bytes from the reader to writer
///
/// It's almost like:
///
/// ```ignore
/// io::copy(&mut reader.take(bytes), &mut writer)
/// ```
///
/// but doesn't consume the reader and allows multiple reads of less than ideal size. It will attempt to read data until `read` returns 0 or
/// until it has enough bytes to be done.
pub fn copy_bytes<W, R>(mut reader: &mut R, writer: &mut W, bytes: usize) -> io::Result<usize> where W: Write, R: Read {
    let mut remaining_bytes = bytes;

    // Reuse the same buffer to save on allocations
    let mut buffer: Vec<u8> = vec![0x0; remaining_bytes];

    while remaining_bytes > 0 {
        // Resize the buffer to the amount expected. Since remaining_bytes only ever decreases,
        // we can use the unsafe set_len method to speed things up.
        // The data doesn't need to be explicitly cleared because it will be overwritten.
        unsafe { buffer.set_len(remaining_bytes); }

        let bytes_read = reader.read(&mut buffer)?;

        if bytes_read > 0 {
            // We didn't read in the full buffer, so resize it to the correct amount.
            // Since u8 doesn't really need to be dropped,
            // we can safely use set_len to resize the buffer without dropping
            if bytes_read < remaining_bytes {
                unsafe { buffer.set_len(bytes_read); }
            }

            // Take off the bytes read from remaining
            remaining_bytes -= bytes_read;

            // Copy the buffer to the writer
            writer.write(&buffer)?;
        } else {
            // We've hit the end of the stream, so return the number of bytes read
            return Ok(bytes - remaining_bytes);
        }
    }

    // All bytes have been copied, so return that number
    Ok(bytes)
}