use capnp;

use common::error::*;

/// File extension to Combustion texture files
pub const EXTENSION: &'static str = "ctex";

pub mod protocol {
    use phf;

    use common::error::*;

    include!(concat!(env!("OUT_DIR"), "/protocols/texture_capnp.rs"));

    pub static BLOCKSIZES: phf::Map<&'static str, BlockSize> = phf_map! {
        "4x4" => BlockSize::B4x4, "b4x4" => BlockSize::B4x4, "B4x4" => BlockSize::B4x4,
        "5x4" => BlockSize::B5x4, "b5x4" => BlockSize::B5x4, "B5x4" => BlockSize::B5x4,
        "5x5" => BlockSize::B5x5, "b5x5" => BlockSize::B5x5, "B5x5" => BlockSize::B5x5,
        "6x5" => BlockSize::B6x5, "b6x5" => BlockSize::B6x5, "B6x5" => BlockSize::B6x5,
        "6x6" => BlockSize::B6x6, "b6x6" => BlockSize::B6x6, "B6x6" => BlockSize::B6x6,
        "8x5" => BlockSize::B8x5, "b8x5" => BlockSize::B8x5, "B8x5" => BlockSize::B8x5,
        "8x6" => BlockSize::B8x6, "b8x6" => BlockSize::B8x6, "B8x6" => BlockSize::B8x6,
        "10x5" => BlockSize::B10x5, "b10x5" => BlockSize::B10x5, "B10x5" => BlockSize::B10x5,
        "10x6" => BlockSize::B10x6, "b10x6" => BlockSize::B10x6, "B10x6" => BlockSize::B10x6,
        "8x8" => BlockSize::B8x8, "b8x8" => BlockSize::B8x8, "B8x8" => BlockSize::B8x8,
        "10x8" => BlockSize::B10x8, "b10x8" => BlockSize::B10x8, "B10x8" => BlockSize::B10x8,
        "10x10" => BlockSize::B10x10, "b10x10" => BlockSize::B10x10, "B10x10" => BlockSize::B10x10,
        "12x10" => BlockSize::B12x10, "b12x10" => BlockSize::B12x10, "B12x10" => BlockSize::B12x10,
        "12x12" => BlockSize::B12x12, "b12x12" => BlockSize::B12x12, "B12x12" => BlockSize::B12x12,
    };

    impl BlockSize {
        /// Accepts strings in the form `4x4`, `b4x4`, or `B4x4`
        pub fn from_str(s: &str) -> BlockSize {
            BLOCKSIZES.get(s).expect_logged("Invalid BlockSize").clone()
        }

        /// Convert symbolic blocksize to `&'static str`
        ///
        /// E.g. `BlockSize::B4x4.to_str() == "4x4"`
        pub fn to_str(&self) -> &'static str {
            match *self {
                BlockSize::B4x4 => "4x4",
                BlockSize::B5x4 => "5x4",
                BlockSize::B5x5 => "5x5",
                BlockSize::B6x5 => "6x5",
                BlockSize::B6x6 => "6x6",
                BlockSize::B8x5 => "8x5",
                BlockSize::B8x6 => "8x6",
                BlockSize::B10x5 => "10x5",
                BlockSize::B10x6 => "10x6",
                BlockSize::B8x8 => "8x8",
                BlockSize::B10x8 => "10x8",
                BlockSize::B10x10 => "10x10",
                BlockSize::B12x10 => "12x10",
                BlockSize::B12x12 => "12x12",
            }
        }
    }
}

/// Very simple enum for color channel manipulations
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Channels {
    R,
    Rg,
    Rgb,
    Rgba,
}

impl From<Channels> for protocol::Raw {
    #[inline]
    fn from(channels: Channels) -> protocol::Raw {
        use self::protocol::*;

        match channels {
            Channels::R => Raw::R,
            Channels::Rg => Raw::Rg,
            Channels::Rgb => Raw::Rgb,
            Channels::Rgba => Raw::Rgba,
        }
    }
}

/// Represents a non-sRGB compression format in symbolic form
#[derive(Clone)]
pub enum Which {
    /// Uncompressed raw pixel data
    None(protocol::Raw),

    /// https://www.opengl.org/wiki/Red_Green_Texture_Compression
    Rgtc(protocol::Rgtc),

    /// https://www.opengl.org/wiki/BPTC_Texture_Compression
    Bptc(protocol::Bptc),

    /// https://www.opengl.org/wiki/S3_Texture_Compression
    S3tc(protocol::S3tc),

    /// https://www.opengl.org/wiki/ASTC_Texture_Compression
    Astc(protocol::BlockSize),
}

impl ::std::fmt::Debug for Which {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use self::protocol::*;

        write!(f, "{} compression", match *self {
            Which::None(ref uncompressed) => {
                match uncompressed {
                    &Raw::R => "Luma (Red) No",
                    &Raw::Rg => "Luma + Alpha (Red-Green) No",
                    &Raw::Rgb => "RGB No",
                    &Raw::Rgba => "RGBA No",
                }
            },
            Which::Rgtc(ref rgtc) => {
                match rgtc {
                    &Rgtc::Red => "Red (Red-Green) Unsigned",
                    &Rgtc::RedSigned => "Red (Red-Green) Signed",
                    &Rgtc::Rg => "Red-Green Unsigned",
                    &Rgtc::RgSigned => "Red-Green Signed",
                }
            },
            Which::Bptc(ref bptc) => {
                match bptc {
                    &Bptc::Rgba => "BPTC RGBA",
                    &Bptc::RgbFloatSigned => "BPTC RGB Float Signed",
                    &Bptc::RgbFloatUnsigned => "BPTC RGB Float Unsigned",
                }
            },
            Which::S3tc(ref s3tc) => {
                match s3tc {
                    &S3tc::Rgb1 => "S3TC DXT1 RGB",
                    &S3tc::Rgba1 => "S3TC DXT1 RGBA",
                    &S3tc::Rgba3 => "S3TC DXT3 RGBA",
                    &S3tc::Rgba5 => "S3TC DXT5 RGBA"
                }
            },
            Which::Astc(_) => "ASTC"
        })?;

        if let Which::Astc(ref blocksize) = *self {
            write!(f, " {}", blocksize.to_str())?;
        }

        Ok(())
    }
}

impl Which {
    /// Get what channel components are represented in this specific format
    pub fn channels(&self) -> Channels {
        use self::protocol::*;

        match *self {
            Which::None(ref uncompressed) => {
                match uncompressed {
                    &Raw::R => Channels::R,
                    &Raw::Rg => Channels::Rg,
                    &Raw::Rgb => Channels::Rgb,
                    &Raw::Rgba => Channels::Rgba,
                }
            }
            Which::Rgtc(ref rgtc) => {
                match rgtc {
                    &Rgtc::Red | &Rgtc::RedSigned => Channels::R,
                    &Rgtc::Rg | &Rgtc::RgSigned => Channels::Rg,
                }
            }
            Which::Bptc(ref bptc) => {
                match bptc {
                    &Bptc::Rgba => Channels::Rgba,
                    _ => Channels::Rgb,
                }
            }
            Which::S3tc(ref s3tc) => {
                match s3tc {
                    &S3tc::Rgb1 => Channels::Rgb,
                    _ => Channels::Rgba
                }
            }
            Which::Astc(_) => Channels::Rgba
        }
    }

    /// Returns true if the stored specific format is signed
    pub fn signed(&self) -> bool {
        use self::protocol::*;

        match *self {
            Which::Rgtc(ref rgtc) => {
                match rgtc {
                    &Rgtc::RedSigned | &Rgtc::RgSigned => true,
                    _ => false
                }
            }
            Which::Bptc(ref bptc) => {
                match bptc {
                    &Bptc::RgbFloatSigned => true,
                    _ => false
                }
            }
            _ => false
        }
    }

    /// Returns true if the stored specific format is floating point
    pub fn float(&self) -> bool {
        use self::protocol::*;

        match *self {
            Which::Bptc(ref bptc) => {
                match bptc {
                    &Bptc::RgbFloatSigned | &Bptc::RgbFloatUnsigned => true,
                    _ => false
                }
            }
            _ => false
        }
    }
}

/// Structure to store random properties until it needs to be converted into a `SpecificFormat`
///
/// Can be used to build up formats
#[derive(Clone)]
pub struct GenericFormat {
    pub channels: Channels,
    pub srgb: bool,
    /// Only applicable to ASTC formats
    pub blocksize: Option<protocol::BlockSize>,
    pub signed: bool,
    pub float: bool,
    /// Only applicable to S3TC/DXT formats.
    ///
    /// **MUST BE 1, 3 or 5**
    pub version: u8,
}

impl Default for GenericFormat {
    fn default() -> GenericFormat {
        GenericFormat {
            channels: Channels::Rgba,
            srgb: false,
            blocksize: None,
            signed: false,
            float: false,
            version: 5 //OpenGL seems to prefer DXT5 on my hardware, so it's a good default
        }
    }
}

impl GenericFormat {
    pub fn new(channels: Channels,
               srgb: bool,
               blocksize: Option<protocol::BlockSize>,
               signed: bool,
               float: bool,
               version: u8) -> GenericFormat {
        GenericFormat {
            channels: channels,
            srgb: srgb,
            blocksize: blocksize,
            signed: signed,
            float: float,
            version: version,
        }
    }

    /// Create a new uncompressed `SpecificFormat` from `self`
    fn none(&self) -> SpecificFormat {
        use self::protocol::*;

        SpecificFormat {
            which: Which::None(self.channels.into()),
            srgb: self.srgb
        }
    }

    /// Create a new RGTC `SpecificFormat` from the properties provided in `self`
    fn rgtc(&self) -> SpecificFormat {
        use self::protocol::*;

        let rgtc = match self.channels {
            Channels::R => {
                if self.signed { Rgtc::RedSigned } else { Rgtc::Red }
            }
            Channels::Rg => {
                if self.signed { Rgtc::RgSigned } else { Rgtc::Rg }
            }
            _ => error_panic!("Invalid image format for RGTC texture compression")
        };

        SpecificFormat {
            which: Which::Rgtc(rgtc),
            srgb: false //this compression method doesn't support sRGB
        }
    }

    /// Create a new S3TC `SpecificFormat` from the properties provided in `self`
    fn s3tc(&self) -> SpecificFormat {
        use self::protocol::*;

        assert!([1, 3, 5].contains(&self.version));

        let s3tc = match self.version {
            1 => {
                if self.channels == Channels::Rgba { S3tc::Rgba1 } else { S3tc::Rgb1 }
            }
            3 => S3tc::Rgba3,
            5 => S3tc::Rgba5,
            _ => unreachable!()
        };

        SpecificFormat {
            which: Which::S3tc(s3tc),
            srgb: self.srgb
        }
    }

    /// Create a new BPTC `SpecificFormat` from the properties provided in `self`
    fn bptc(&self) -> SpecificFormat {
        use self::protocol::*;

        let bptc = if self.float {
            if self.signed { Bptc::RgbFloatSigned } else { Bptc::RgbFloatUnsigned }
        } else {
            Bptc::Rgba
        };

        SpecificFormat {
            which: Which::Bptc(bptc),
            srgb: self.srgb
        }
    }

    /// Create a new ASTC `SpecificFormat` from the properties provided in `self`
    fn astc(&self) -> SpecificFormat {
        use self::protocol::*;

        let blocksize = self.blocksize.expect_logged("blocksize is not present");

        SpecificFormat {
            which: Which::Astc(blocksize),
            srgb: self.srgb
        }
    }
}

/// Represents a specific compression format in symbolic form. As in, there are no
/// OpenGL, DirectX or whatever enum values associated with it.
#[derive(Clone)]
pub struct SpecificFormat {
    pub which: Which,
    pub srgb: bool
}

impl SpecificFormat {
    /// Consume self and convert specific formats back into generic ones
    pub fn into_generic(self) -> GenericFormat {
        use self::protocol::*;

        GenericFormat {
            channels: self.which.channels(),
            srgb: self.srgb,
            blocksize: match self.which {
                Which::Astc(blocksize) => Some(blocksize.clone()),
                _ => None
            },
            signed: self.which.signed(),
            float: self.which.float(),
            version: match self.which {
                Which::S3tc(s3tc) => {
                    match s3tc {
                        S3tc::Rgb1 | S3tc::Rgba1 => 1,
                        S3tc::Rgba3 => 3,
                        S3tc::Rgba5 => 5
                    }
                },
                _ => 0
            }
        }
    }

    /// Convert specific formats into generic properties
    pub fn to_generic(&self) -> GenericFormat {
        self.clone().into_generic()
    }

    /// Write specific format to Cap'N Proto texture structure
    pub fn write_texture<'a>(&self, mut builder: &mut self::protocol::texture::Builder<'a>) {
        {
            let mut compression = builder.borrow().get_compression();

            match self.which.clone() {
                Which::None(uncompressed) => compression.set_none(uncompressed),
                Which::Rgtc(rgtc) => compression.set_rgtc(rgtc),
                Which::Bptc(bptc) => compression.set_bptc(bptc),
                Which::S3tc(s3tc) => compression.set_s3tc(s3tc),
                Which::Astc(astc) => compression.set_astc(astc),
            }
        }

        builder.set_srgb(self.srgb);
    }

    /// Read in specific format from Cap'N Proto texture structure
    pub fn read_texture<'a>(reader: &self::protocol::texture::Reader<'a>) -> Result<SpecificFormat, capnp::NotInSchema> {
        use self::protocol::*;
        use self::protocol::texture::compression::Which as PWhich;

        let which = {
            let compression = reader.borrow().get_compression();

            match compression.which()? {
                PWhich::None(none) => Which::None(none?),
                PWhich::Rgtc(rgtc) => Which::Rgtc(rgtc?),
                PWhich::Bptc(bptc) => Which::Bptc(bptc?),
                PWhich::S3tc(s3tc) => Which::S3tc(s3tc?),
                PWhich::Astc(astc) => Which::Astc(astc?),
            }
        };

        Ok(SpecificFormat {
            which: which,
            srgb: reader.get_srgb()
        })
    }
}

impl ::std::fmt::Debug for SpecificFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        if self.srgb {
            write!(f, "sRGB ")?;
        }

        write!(f, "{:?}", self.which)
    }
}

pub mod gl {
    use backend::gl::types::*;
    use backend::gl::bindings as glb;

    use super::protocol::{BlockSize, Bptc, Rgtc, S3tc, Raw};
    use super::{Channels, GenericFormat, Which, SpecificFormat};

    /// OpenGL extension to `SpecificFormat` to convert raw `GLenum` texture formats into the symbolic `SpecificFormat`
    pub trait GLSpecificFormatExt {
        /// Create symbolic `SpecificFormat` from provided `GLenum` value
        fn from_raw_gl(format: GLenum) -> SpecificFormat;
    }

    impl GLSpecificFormatExt for SpecificFormat {
        fn from_raw_gl(format: GLenum) -> SpecificFormat {
            let (which, srgb) = match format {
                glb::COMPRESSED_RED_RGTC1 => (Which::Rgtc(Rgtc::Red), false),
                glb::COMPRESSED_SIGNED_RED_RGTC1 => (Which::Rgtc(Rgtc::RedSigned), false),
                glb::COMPRESSED_RG_RGTC2 => (Which::Rgtc(Rgtc::Rg), false),
                glb::COMPRESSED_SIGNED_RG_RGTC2 => (Which::Rgtc(Rgtc::RgSigned), false),

                glb::COMPRESSED_RGB_BPTC_SIGNED_FLOAT => (Which::Bptc(Bptc::RgbFloatSigned), false),
                glb::COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT => (Which::Bptc(Bptc::RgbFloatUnsigned), false),
                glb::COMPRESSED_SRGB_ALPHA_BPTC_UNORM => (Which::Bptc(Bptc::Rgba), true),
                glb::COMPRESSED_RGBA_BPTC_UNORM => (Which::Bptc(Bptc::Rgba), false),

                glb::COMPRESSED_RGB_S3TC_DXT1_EXT => (Which::S3tc(S3tc::Rgb1), false),
                glb::COMPRESSED_RGBA_S3TC_DXT1_EXT => (Which::S3tc(S3tc::Rgba1), false),
                glb::COMPRESSED_RGBA_S3TC_DXT3_EXT => (Which::S3tc(S3tc::Rgba3), false),
                glb::COMPRESSED_RGBA_S3TC_DXT5_EXT => (Which::S3tc(S3tc::Rgba5), false),

                glb::COMPRESSED_SRGB_S3TC_DXT1_EXT => (Which::S3tc(S3tc::Rgb1), true),
                glb::COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT => (Which::S3tc(S3tc::Rgba1), true),
                glb::COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT => (Which::S3tc(S3tc::Rgba3), true),
                glb::COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT => (Which::S3tc(S3tc::Rgba5), true),

                glb::COMPRESSED_RGBA_ASTC_4x4_KHR => (Which::Astc(BlockSize::B4x4), false),
                glb::COMPRESSED_RGBA_ASTC_5x4_KHR => (Which::Astc(BlockSize::B5x4), false),
                glb::COMPRESSED_RGBA_ASTC_5x5_KHR => (Which::Astc(BlockSize::B5x5), false),
                glb::COMPRESSED_RGBA_ASTC_6x5_KHR => (Which::Astc(BlockSize::B6x5), false),
                glb::COMPRESSED_RGBA_ASTC_6x6_KHR => (Which::Astc(BlockSize::B6x6), false),
                glb::COMPRESSED_RGBA_ASTC_8x5_KHR => (Which::Astc(BlockSize::B8x5), false),
                glb::COMPRESSED_RGBA_ASTC_8x6_KHR => (Which::Astc(BlockSize::B8x6), false),
                glb::COMPRESSED_RGBA_ASTC_8x8_KHR => (Which::Astc(BlockSize::B8x8), false),
                glb::COMPRESSED_RGBA_ASTC_10x5_KHR => (Which::Astc(BlockSize::B10x5), false),
                glb::COMPRESSED_RGBA_ASTC_10x6_KHR => (Which::Astc(BlockSize::B10x6), false),
                glb::COMPRESSED_RGBA_ASTC_10x8_KHR => (Which::Astc(BlockSize::B10x8), false),
                glb::COMPRESSED_RGBA_ASTC_10x10_KHR => (Which::Astc(BlockSize::B10x10), false),
                glb::COMPRESSED_RGBA_ASTC_12x10_KHR => (Which::Astc(BlockSize::B12x10), false),
                glb::COMPRESSED_RGBA_ASTC_12x12_KHR => (Which::Astc(BlockSize::B12x12), false),

                glb::COMPRESSED_SRGB8_ALPHA8_ASTC_4x4_KHR => (Which::Astc(BlockSize::B4x4), true),
                glb::COMPRESSED_SRGB8_ALPHA8_ASTC_5x4_KHR => (Which::Astc(BlockSize::B5x4), true),
                glb::COMPRESSED_SRGB8_ALPHA8_ASTC_5x5_KHR => (Which::Astc(BlockSize::B5x5), true),
                glb::COMPRESSED_SRGB8_ALPHA8_ASTC_6x5_KHR => (Which::Astc(BlockSize::B6x5), true),
                glb::COMPRESSED_SRGB8_ALPHA8_ASTC_6x6_KHR => (Which::Astc(BlockSize::B6x6), true),
                glb::COMPRESSED_SRGB8_ALPHA8_ASTC_8x5_KHR => (Which::Astc(BlockSize::B8x5), true),
                glb::COMPRESSED_SRGB8_ALPHA8_ASTC_8x6_KHR => (Which::Astc(BlockSize::B8x6), true),
                glb::COMPRESSED_SRGB8_ALPHA8_ASTC_8x8_KHR => (Which::Astc(BlockSize::B8x8), true),
                glb::COMPRESSED_SRGB8_ALPHA8_ASTC_10x5_KHR => (Which::Astc(BlockSize::B10x5), true),
                glb::COMPRESSED_SRGB8_ALPHA8_ASTC_10x6_KHR => (Which::Astc(BlockSize::B10x6), true),
                glb::COMPRESSED_SRGB8_ALPHA8_ASTC_10x8_KHR => (Which::Astc(BlockSize::B10x8), true),
                glb::COMPRESSED_SRGB8_ALPHA8_ASTC_10x10_KHR => (Which::Astc(BlockSize::B10x10), true),
                glb::COMPRESSED_SRGB8_ALPHA8_ASTC_12x10_KHR => (Which::Astc(BlockSize::B12x10), true),
                glb::COMPRESSED_SRGB8_ALPHA8_ASTC_12x12_KHR => (Which::Astc(BlockSize::B12x12), true),

                _ => panic!("Unknown texture format: {:x}", format),
            };

            SpecificFormat {
                which: which,
                srgb: srgb
            }
        }
    }

    /// OpenGL extension to `SpecificFormat` to create a `GLenum` value for passing to `glTexImage*` functions
    pub trait GLCompressedFormats {
        /// Get a generic format like `RED`, `RG`, `RGB`, `RGBA`
        fn generic(&self) -> GLuint;

        /// Get a compressed generic format equivalent to the compressed version of `generic`
        fn auto(&self) -> GLuint;

        /// Get specific internal texture format enum for the given symbolic format
        fn specific(&self) -> GLuint;
    }

    impl GLCompressedFormats for SpecificFormat {
        fn generic(&self) -> GLuint {
            match self.which.channels() {
                Channels::R => glb::RED,
                Channels::Rg => glb::RG,
                Channels::Rgb => glb::RGB,
                Channels::Rgba => glb::RGBA,
            }
        }

        fn auto(&self) -> GLuint {
            match self.which.channels() {
                Channels::R => glb::COMPRESSED_RED,
                Channels::Rg => glb::COMPRESSED_RG,
                Channels::Rgb => {
                    if self.srgb { glb::COMPRESSED_SRGB } else { glb::COMPRESSED_RGB }
                },
                Channels::Rgba => {
                    if self.srgb { glb::COMPRESSED_SRGB_ALPHA } else { glb::COMPRESSED_RGBA }
                },
            }
        }

        fn specific(&self) -> GLuint {
            use super::protocol::*;

            match self.which {
                Which::None(ref raw) => {
                    match raw {
                        &Raw::R => glb::R8,
                        &Raw::Rg => glb::RG8,
                        &Raw::Rgb => glb::RGB8,
                        &Raw::Rgba => glb::RGBA8,
                    }
                }
                Which::Rgtc(ref rgtc) => {
                    match rgtc {
                        &Rgtc::Red => glb::COMPRESSED_RED_RGTC1,
                        &Rgtc::RedSigned => glb::COMPRESSED_SIGNED_RED_RGTC1,
                        &Rgtc::Rg => glb::COMPRESSED_RG_RGTC2,
                        &Rgtc::RgSigned => glb::COMPRESSED_SIGNED_RG_RGTC2,
                    }
                }
                Which::Bptc(ref bptc) => {
                    match bptc {
                        &Bptc::Rgba => {
                            if self.srgb { glb::COMPRESSED_SRGB_ALPHA_BPTC_UNORM } else { glb::COMPRESSED_RGBA_BPTC_UNORM }
                        }
                        &Bptc::RgbFloatSigned => glb::COMPRESSED_RGB_BPTC_SIGNED_FLOAT,
                        &Bptc::RgbFloatUnsigned => glb::COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT,
                    }
                }
                Which::S3tc(ref s3tc) => {
                    if self.srgb {
                        match s3tc {
                            &S3tc::Rgb1 => glb::COMPRESSED_SRGB_S3TC_DXT1_EXT,
                            &S3tc::Rgba1 => glb::COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT,
                            &S3tc::Rgba3 => glb::COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT,
                            &S3tc::Rgba5 => glb::COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT,
                        }
                    } else {
                        match s3tc {
                            &S3tc::Rgb1 => glb::COMPRESSED_RGB_S3TC_DXT1_EXT,
                            &S3tc::Rgba1 => glb::COMPRESSED_RGBA_S3TC_DXT1_EXT,
                            &S3tc::Rgba3 => glb::COMPRESSED_RGBA_S3TC_DXT3_EXT,
                            &S3tc::Rgba5 => glb::COMPRESSED_RGBA_S3TC_DXT5_EXT,
                        }
                    }
                }
                Which::Astc(ref blocksize) => {
                    if self.srgb {
                        match blocksize {
                            &BlockSize::B4x4 => glb::COMPRESSED_SRGB8_ALPHA8_ASTC_4x4_KHR,
                            &BlockSize::B5x4 => glb::COMPRESSED_SRGB8_ALPHA8_ASTC_5x4_KHR,
                            &BlockSize::B5x5 => glb::COMPRESSED_SRGB8_ALPHA8_ASTC_5x5_KHR,
                            &BlockSize::B6x5 => glb::COMPRESSED_SRGB8_ALPHA8_ASTC_6x5_KHR,
                            &BlockSize::B6x6 => glb::COMPRESSED_SRGB8_ALPHA8_ASTC_6x6_KHR,
                            &BlockSize::B8x5 => glb::COMPRESSED_SRGB8_ALPHA8_ASTC_8x5_KHR,
                            &BlockSize::B8x6 => glb::COMPRESSED_SRGB8_ALPHA8_ASTC_8x6_KHR,
                            &BlockSize::B10x5 => glb::COMPRESSED_SRGB8_ALPHA8_ASTC_10x5_KHR,
                            &BlockSize::B10x6 => glb::COMPRESSED_SRGB8_ALPHA8_ASTC_10x6_KHR,
                            &BlockSize::B8x8 => glb::COMPRESSED_SRGB8_ALPHA8_ASTC_8x8_KHR,
                            &BlockSize::B10x8 => glb::COMPRESSED_SRGB8_ALPHA8_ASTC_10x8_KHR,
                            &BlockSize::B10x10 => glb::COMPRESSED_SRGB8_ALPHA8_ASTC_10x10_KHR,
                            &BlockSize::B12x10 => glb::COMPRESSED_SRGB8_ALPHA8_ASTC_12x10_KHR,
                            &BlockSize::B12x12 => glb::COMPRESSED_SRGB8_ALPHA8_ASTC_12x12_KHR,
                        }
                    } else {
                        match blocksize {
                            &BlockSize::B4x4 => glb::COMPRESSED_RGBA_ASTC_4x4_KHR,
                            &BlockSize::B5x4 => glb::COMPRESSED_RGBA_ASTC_5x4_KHR,
                            &BlockSize::B5x5 => glb::COMPRESSED_RGBA_ASTC_5x5_KHR,
                            &BlockSize::B6x5 => glb::COMPRESSED_RGBA_ASTC_6x5_KHR,
                            &BlockSize::B6x6 => glb::COMPRESSED_RGBA_ASTC_6x6_KHR,
                            &BlockSize::B8x5 => glb::COMPRESSED_RGBA_ASTC_8x5_KHR,
                            &BlockSize::B8x6 => glb::COMPRESSED_RGBA_ASTC_8x6_KHR,
                            &BlockSize::B10x5 => glb::COMPRESSED_RGBA_ASTC_10x5_KHR,
                            &BlockSize::B10x6 => glb::COMPRESSED_RGBA_ASTC_10x6_KHR,
                            &BlockSize::B8x8 => glb::COMPRESSED_RGBA_ASTC_8x8_KHR,
                            &BlockSize::B10x8 => glb::COMPRESSED_RGBA_ASTC_10x8_KHR,
                            &BlockSize::B10x10 => glb::COMPRESSED_RGBA_ASTC_10x10_KHR,
                            &BlockSize::B12x10 => glb::COMPRESSED_RGBA_ASTC_12x10_KHR,
                            &BlockSize::B12x12 => glb::COMPRESSED_RGBA_ASTC_12x12_KHR,
                        }
                    }
                }
            }
        }
    }
}