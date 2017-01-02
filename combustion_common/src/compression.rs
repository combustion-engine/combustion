use std::io::prelude::*;
use std::io;

use lz4::*;

/// Wrapper around a LZ4 `Encoder` object that will call `finish` on `drop`,
/// which isn't always recommended.
pub struct AutoEncoder<W: Write>(Option<Encoder<W>>);

impl<W: Write> AutoEncoder<W> {
    /// Short for `into_encoder().finish()`
    pub fn finish(mut self) -> (W, io::Result<()>) {
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
        match self.0.take() {
            Some(encoder) => {
                let (_, result) = encoder.finish();

                result.expect("Error finishing compressed stream");
            }
            None => {}
        }
    }
}

/// Generic options for in-memory compression
pub struct CompressionOptions {
    pub blockmode: BlockMode,
    pub blocksize: BlockSize,
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

    /// Get the compressed size (in bytes) of the data.
    #[inline]
    pub fn len(&self) -> usize {
        self.data.get_ref().len()
    }

    /// Create a reader that decompresses the data on demand.
    ///
    /// The lifetime of the reader shall not exceed the lifetime of the `CompressedMemory` instance.
    pub fn create_reader<'a>(&'a self) -> io::Result<Box<Read + 'a>> {
        let cursor: io::Cursor<&'a [u8]> = io::Cursor::new(self.data.get_ref().as_slice());

        Ok(Box::new(Decoder::new(cursor)?))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io;
    use std::iter::repeat;
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