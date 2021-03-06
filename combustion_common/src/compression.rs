//! Compressed memory and other compression utils

use std::io::prelude::*;
use std::io;

use lz4::*;

/// Wrapper around a LZ4 `Encoder` object that will call `finish` on `drop`,
/// which isn't always recommended.
pub struct AutoEncoder<W: Write>(Option<Encoder<W>>);

impl<W: Write> AutoEncoder<W> {
    /// Short for `into_encoder().finish()`
    pub fn finish(self) -> (W, io::Result<()>) {
        self.into_encoder().finish()
    }

    /// Consume self and return the original `Encoder`
    #[inline]
    pub fn into_encoder(mut self) -> Encoder<W> {
        self.0.take().unwrap()
    }
}

impl<W: Write> From<Encoder<W>> for AutoEncoder<W> {
    #[inline]
    fn from(e: Encoder<W>) -> AutoEncoder<W> {
        AutoEncoder(Some(e))
    }
}

impl<W: Write> Write for AutoEncoder<W> {
    #[inline]
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        self.0.as_mut().unwrap().write(buffer)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.0.as_mut().unwrap().flush()
    }
}

impl<W: Write> Drop for AutoEncoder<W> {
    fn drop(&mut self) {
        if let Some(encoder) = self.0.take() {
            let (_, result) = encoder.finish();

            result.expect("Error finishing compressed stream");
        }
    }
}

/// Generic options for in-memory compression
#[derive(Debug, Clone, Copy)]
pub struct CompressionOptions {
    /// Blockmode for the compressed stream
    pub blockmode: BlockMode,
    /// Blocksize for the compressed stream
    pub blocksize: BlockSize,
    /// Compression level of the compressed stream
    pub level: u32
}

impl CompressionOptions {
    /// Create options with given level and default `blockmode` and `blocksize`
    pub fn with_level(level: u32) -> CompressionOptions {
        CompressionOptions {
            blockmode: BlockMode::Linked,
            blocksize: BlockSize::Default,
            level: level,
        }
    }
}

/// Stores a chunk of compressed data in memory so it can be decompressed on demand,
/// but otherwise take up less space when not being used.
#[derive(Clone)]
pub struct CompressedMemory {
    data: io::Cursor<Vec<u8>>,
}

impl CompressedMemory {
    /// Compress data read from `reader` with the given `options`.
    ///
    /// This is not lazy, and will consume the `reader` until end.
    pub fn from_reader<R: Read>(mut reader: R, options: CompressionOptions) -> io::Result<CompressedMemory> {
        let buffer = io::Cursor::new(Vec::<u8>::new());

        let mut encoder = EncoderBuilder::new()
            .level(options.level)
            .block_size(options.blocksize)
            .block_mode(options.blockmode)
            .auto_flush(true)
            //Since this is stored in memory, there really isn't a need for checksums in the data
            .checksum(ContentChecksum::NoChecksum)
            .build(buffer)?;

        io::copy(&mut reader, &mut encoder)?;

        let (buffer, result) = encoder.finish();

        result?;

        Ok(CompressedMemory { data: buffer })
    }

    /// Compress data from `buffer` with the given `options`
    pub fn from_buffer(buffer: &[u8], options: CompressionOptions) -> io::Result<CompressedMemory> {
        CompressedMemory::from_reader(io::Cursor::new(buffer), options)
    }

    /// Copies compressed data from `reader` into `CompressedMemory`
    pub fn from_compressed_reader<R: Read>(mut reader: R) -> io::Result<CompressedMemory> {
        let mut buffer = io::Cursor::new(Vec::new());

        io::copy(&mut reader, &mut buffer)?;

        Ok(CompressedMemory { data: buffer })
    }

    /// Copies compressed data from `buffer` into `CompressedMemory`
    pub fn from_compressed_buffer(buffer: &[u8]) -> io::Result<CompressedMemory> {
        CompressedMemory::from_compressed_reader(io::Cursor::new(buffer))
    }

    /// Get the compressed size (in bytes) of the data.
    #[inline]
    pub fn len(&self) -> usize {
        self.data.get_ref().len()
    }

    /// Check if buffer is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.get_ref().is_empty()
    }

    /// Create a reader that decompresses the data on the fly.
    ///
    /// The lifetime of the reader shall not exceed the lifetime of the `CompressedMemory` instance.
    #[allow(needless_lifetimes)]
    pub fn create_reader<'a>(&'a self) -> io::Result<Box<Read + 'a>> {
        let cursor: io::Cursor<&'a [u8]> = io::Cursor::new(self.data.get_ref().as_slice());

        Ok(Box::new(Decoder::new(cursor)?))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io;
    use rand::*;

    #[test]
    fn in_memory() {
        let size = 1 << 16;
        let sample_size = 1 << 10;
        let mut source = StdRng::from_seed(&[0x1234]);

        let mut data = Vec::with_capacity(size);

        let sample_data: Vec<u8> = source.gen_iter().take(sample_size).collect();

        for i in 0..size {
            data.push(sample_data[i % sample_size as usize]);
        }

        let compressed = CompressedMemory::from_reader(io::Cursor::new(data.clone()), CompressionOptions::with_level(16))
            .expect("Could not compress data");

        let mut new_data = io::Cursor::new(Vec::with_capacity(size));
        let mut reader = compressed.create_reader().unwrap();
        io::copy(&mut reader, &mut new_data).unwrap();

        assert_eq!(data.as_slice(), new_data.get_ref().as_slice());
    }
}