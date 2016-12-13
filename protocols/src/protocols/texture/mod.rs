//! Generic texture protocol

use capnp;

use common::error::*;

pub mod protocol;
pub mod gl;

/// File extension to Combustion texture files
pub const EXTENSION: &'static str = "ctex";

/// Very simple enum for color channel manipulations
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Channels {
    R,
    Rg,
    Rgb,
    Rgba,
}

impl Channels {
    pub fn num_channels(&self) -> usize {
        match *self {
            Channels::R => 1,
            Channels::Rg => 2,
            Channels::Rgb => 3,
            Channels::Rgba => 4
        }
    }
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

impl From<protocol::Raw> for Channels {
    #[inline]
    fn from(raw: protocol::Raw) -> Channels {
        use self::protocol::*;

        match raw {
            Raw::R => Channels::R,
            Raw::Rg => Channels::Rg,
            Raw::Rgb => Channels::Rgb,
            Raw::Rgba => Channels::Rgba
        }
    }
}

impl ::std::fmt::Debug for Channels {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", match *self {
            Channels::R => "Red",
            Channels::Rg => "Red-green",
            Channels::Rgb => "Rgb",
            Channels::Rgba => "Rgba",
        })
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

        write!(f, "{}", match *self {
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

        write!(f, " compression")
    }
}

impl Which {
    /// Get what channel components are represented in this specific format
    pub fn channels(&self) -> Channels {
        use self::protocol::*;

        match *self {
            Which::None(ref raw) => raw.clone().into(),
            Which::Rgtc(ref rgtc) => {
                match rgtc {
                    &Rgtc::Red | &Rgtc::RedSigned => Channels::R,
                    &Rgtc::Rg | &Rgtc::RgSigned => Channels::Rg,
                }
            },
            Which::Bptc(ref bptc) => {
                match bptc {
                    &Bptc::Rgba => Channels::Rgba,
                    _ => Channels::Rgb,
                }
            },
            Which::S3tc(ref s3tc) => {
                match s3tc {
                    &S3tc::Rgb1 => Channels::Rgb,
                    _ => Channels::Rgba
                }
            },
            Which::Astc(_) => Channels::Rgba,
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
            },
            Which::Bptc(ref bptc) => {
                match bptc {
                    &Bptc::RgbFloatSigned => true,
                    _ => false
                }
            },
            _ => false,
        }
    }

    /// Returns true if the stored specific format is floating point
    pub fn float(&self) -> bool {
        use self::protocol::*;

        match *self {
            Which::Bptc(ref bptc) => {
                match bptc {
                    &Bptc::RgbFloatSigned | &Bptc::RgbFloatUnsigned => true,
                    _ => false,
                }
            },
            _ => false,
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
            version: 5, //OpenGL seems to prefer DXT5 on my hardware, so it's a good default
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

    #[inline(always)]
    pub fn num_channels(&self) -> usize {
        self.channels.num_channels()
    }

    /// Create a new uncompressed `SpecificFormat` from `self`
    pub fn none(&self) -> SpecificFormat {
        SpecificFormat {
            which: Which::None(self.channels.into()),
            srgb: self.srgb,
        }
    }

    /// Create a new RGTC `SpecificFormat` from the properties provided in `self`
    pub fn rgtc(&self) -> SpecificFormat {
        use self::protocol::*;

        let rgtc = match self.channels {
            Channels::R => {
                if self.signed { Rgtc::RedSigned } else { Rgtc::Red }
            },
            Channels::Rg => {
                if self.signed { Rgtc::RgSigned } else { Rgtc::Rg }
            },
            _ => error_panic!("Invalid image format for RGTC texture compression"),
        };

        SpecificFormat {
            which: Which::Rgtc(rgtc),
            srgb: false, //this compression method doesn't support sRGB
        }
    }

    /// Create a new S3TC `SpecificFormat` from the properties provided in `self`
    pub fn s3tc(&self) -> SpecificFormat {
        use self::protocol::*;

        assert!([1, 3, 5].contains(&self.version));

        let s3tc = match self.version {
            1 => {
                if self.channels == Channels::Rgba { S3tc::Rgba1 } else { S3tc::Rgb1 }
            }
            3 => S3tc::Rgba3,
            5 => S3tc::Rgba5,
            _ => unreachable!(),
        };

        SpecificFormat {
            which: Which::S3tc(s3tc),
            srgb: self.srgb,
        }
    }

    /// Create a new BPTC `SpecificFormat` from the properties provided in `self`
    pub fn bptc(&self) -> SpecificFormat {
        use self::protocol::*;

        let bptc = if self.float {
            if self.signed { Bptc::RgbFloatSigned } else { Bptc::RgbFloatUnsigned }
        } else {
            Bptc::Rgba
        };

        SpecificFormat {
            which: Which::Bptc(bptc),
            srgb: self.srgb,
        }
    }

    /// Create a new ASTC `SpecificFormat` from the properties provided in `self`
    pub fn astc(&self) -> SpecificFormat {
        let blocksize = self.blocksize.expect_logged("blocksize is not present");

        SpecificFormat {
            which: Which::Astc(blocksize),
            srgb: self.srgb,
        }
    }
}

/// Represents a specific compression format in symbolic form. As in, there are no
/// OpenGL, DirectX or whatever enum values associated with it.
#[derive(Clone)]
pub struct SpecificFormat {
    pub which: Which,
    pub srgb: bool,
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

    pub fn is_compressed(&self) -> bool {
        match self.which {
            Which::None(_) => false,
            _ => true,
        }
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
