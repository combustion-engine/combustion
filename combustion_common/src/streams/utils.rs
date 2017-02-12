//! I/O Stream misc utilities

use std::io;
use std::io::prelude::*;

/// Like `Read::read_exact`, but allows incomplete reads if the stream ends.
/// Partial reads and interupted reads are just fine and it will just try again.
///
/// Returns the number of bytes actually read
pub fn try_read_exact<R: Read>(reader: &mut R, mut buf: &mut [u8]) -> io::Result<usize> {
    let expected_bytes = buf.len();

    while !buf.is_empty() {
        match reader.read(buf) {
            Ok(0) => break,
            Ok(n) => {
                let tmp = buf;
                buf = &mut tmp[n..];
            },
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        }
    }

    Ok(expected_bytes - buf.len())
}

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
    // Reuse the same buffer to save on allocations
    let mut buffer = vec![0x0; bytes];

    let bytes_read = try_read_exact(&mut reader, &mut buffer)?;

    writer.write_all(&buffer[..bytes_read])?;

    Ok(bytes_read)
}

/// Variation of `copy_bytes` that uses a custom buffer size instead
/// of allocating `bytes` bytes of space for the buffer.
///
/// ## Panics
///
/// Panics if `bufsize` is zero.
pub fn copy_bytes_bufsize<W, R>(mut reader: &mut R, writer: &mut W, bytes: usize, bufsize: usize) -> io::Result<usize> where W: Write, R: Read {
    if bufsize == 0 {
        panic!("Invalid buffer size");
    }

    // Get minimum of bytes and bufsize,
    // so bufsize is always less than or equal to bytes
    let bufsize = if bytes < bufsize { bytes } else { bufsize };

    let mut remaining_bytes = bytes;

    let mut buffer = vec![0x0; bufsize];

    while remaining_bytes > 0 {
        // Don't bother trying to read in more than necessary,
        // so get the minimum amount we should read in
        let min = if remaining_bytes < bufsize { remaining_bytes } else { bufsize };

        let min_buffer = &mut buffer[..min];

        let bytes_read = try_read_exact(&mut reader, min_buffer)?;

        // End of stream
        if bytes_read == 0 {
            break;
        }

        writer.write_all(min_buffer)?;

        remaining_bytes -= bytes_read;
    }

    Ok(bytes - remaining_bytes)
}

#[cfg(test)]
mod test {
    use super::*;

    use std::io::Cursor;

    #[test]
    fn test_try_read_exact() {
        let mut reader = Cursor::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        let mut small_buffer = [0; 5];
        let mut large_buffer = [0; 50];

        let bytes_read = try_read_exact(&mut reader, &mut small_buffer).unwrap();

        assert_eq!(bytes_read, 5);
        assert_eq!(small_buffer, [1, 2, 3, 4, 5]);

        let bytes_read = try_read_exact(&mut reader, &mut large_buffer).unwrap();

        assert_eq!(bytes_read, 5);
        assert_eq!(large_buffer[..5], [6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_copy_bytes() {
        let mut reader = Cursor::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let mut writer = Cursor::new(Vec::new());

        let bytes_copied = copy_bytes(&mut reader, &mut writer, 10).unwrap();

        assert_eq!(bytes_copied, 10);
        assert_eq!(reader.get_ref(), writer.get_ref());
    }

    #[test]
    fn test_copy_bytes_smaller() {
        let mut reader = Cursor::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let mut writer = Cursor::new(Vec::new());

        let bytes_copied = copy_bytes(&mut reader, &mut writer, 5).unwrap();

        assert_eq!(bytes_copied, 5);
        assert_eq!(writer.get_ref(), &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_copy_bytes_larger() {
        let mut reader = Cursor::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let mut writer = Cursor::new(Vec::new());

        let bytes_copied = copy_bytes(&mut reader, &mut writer, 15).unwrap();

        assert_eq!(bytes_copied, 10);
        assert_eq!(reader.get_ref(), writer.get_ref());
    }

    #[test]
    fn test_copy_bytes_buf() {
        let mut reader = Cursor::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let mut writer = Cursor::new(Vec::new());

        let bytes_copied = copy_bytes_bufsize(&mut reader, &mut writer, 5, 15).unwrap();

        assert_eq!(bytes_copied, 5);
        assert_eq!(writer.get_ref(), &[1, 2, 3, 4, 5]);
    }
}