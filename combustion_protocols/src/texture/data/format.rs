//! Compressed and uncompressed format descriptions both specific and generic

use ::error::{ProtocolResult, ProtocolError};

use ::texture::protocol::{self, Channels, DataType};

pub use ::texture::protocol::BlockSize;

/// DXT versions to use with the S3TC algorithm
pub enum DXTVersion {
    /// DXT1 variant
    DXT1 = 1,
    /// DXT3 variant
    DXT3 = 3,
    /// DXT5 variant
    DXT5 = 5,
}

/// Uncompressed image format
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Uncompressed {
    /// Channels for uncompressed format
    pub channels: Channels,
    /// Data type to store pixel data
    pub data_type: DataType,
}

impl Channels {
    /// Gets the number of channels
    ///
    /// ```ignore
    /// match *self {
    ///     Channels::R => 1,
    ///     Channels::Rg => 2,
    ///     Channels::Rgb => 3,
    ///     Channels::Rgba => 4
    /// }
    /// ```
    pub fn num_channels(&self) -> usize {
        match *self {
            Channels::R => 1,
            Channels::Rg => 2,
            Channels::Rgb => 3,
            Channels::Rgba => 4
        }
    }
}

/// Represents a non-sRGB compression format in symbolic form
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Which {
    /// Uncompressed raw pixel data
    None(Uncompressed),

    /// https://www.opengl.org/wiki/Red_Green_Texture_Compression
    Rgtc(protocol::Rgtc),

    /// https://www.opengl.org/wiki/BPTC_Texture_Compression
    Bptc(protocol::Bptc),

    /// https://www.opengl.org/wiki/S3_Texture_Compression
    S3tc(protocol::S3tc),

    /// https://www.opengl.org/wiki/ASTC_Texture_Compression
    Astc(protocol::BlockSize),
}

impl ::std::fmt::Display for Which {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Which::None(ref tc) => write!(f, "Uncompressed {:?}", tc),
            Which::Rgtc(ref tc) => write!(f, "RGTC {}", tc),
            Which::Bptc(ref tc) => write!(f, "BPTC {}", tc),
            Which::S3tc(ref tc) => write!(f, "S3TC {}", tc),
            Which::Astc(ref tc) => write!(f, "ASTC {}", tc),
        }
    }
}

impl Which {
    /// Get what channel components are represented in this specific format
    pub fn channels(&self) -> Channels {
        use self::protocol::{Rgtc, Bptc, S3tc};

        match *self {
            Which::None(uncompressed) => uncompressed.channels,
            Which::Rgtc(rgtc) => {
                match rgtc {
                    Rgtc::Red | Rgtc::RedSigned => Channels::R,
                    Rgtc::Rg | Rgtc::RgSigned => Channels::Rg,
                }
            },
            Which::Bptc(bptc) => {
                match bptc {
                    Bptc::Rgba => Channels::Rgba,
                    _ => Channels::Rgb,
                }
            },
            Which::S3tc(s3tc) => {
                match s3tc {
                    S3tc::Rgb1 => Channels::Rgb,
                    _ => Channels::Rgba
                }
            },
            Which::Astc(_) => Channels::Rgba,
        }
    }

    /// Returns true if the stored specific format is signed
    pub fn signed(&self) -> bool {
        use self::protocol::{Rgtc, Bptc};

        match *self {
            Which::Rgtc(rgtc) => {
                match rgtc {
                    Rgtc::RedSigned | Rgtc::RgSigned => true,
                    _ => false
                }
            },
            Which::Bptc(bptc) if bptc == Bptc::RgbFloatSigned => true,
            Which::None(uncompressed) => {
                match uncompressed.data_type {
                    DataType::Byte | DataType::Short | DataType::Int | DataType::Float => true,
                    _ => false,
                }
            },
            _ => false,
        }
    }

    /// Returns true if the stored specific format is floating point
    pub fn float(&self) -> bool {
        use self::protocol::Bptc;

        match *self {
            Which::Bptc(bptc) => {
                match bptc {
                    Bptc::RgbFloatSigned | Bptc::RgbFloatUnsigned => true,
                    _ => false,
                }
            },
            Which::None(uncompressed) if uncompressed.data_type == DataType::Float => true,
            _ => false,
        }
    }
}

/// Structure to store random properties until it needs to be converted into a `SpecificFormat`
///
/// Can be used to build up formats
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GenericFormat {
    /// Channels to be used in this format
    pub channels: Channels,
    /// sRGB color space for format
    pub srgb: bool,
    /// Signed format
    pub signed: bool,
    /// Floating point format
    pub float: bool,
    /// Data type for data storage
    pub data_type: DataType,
}

impl Default for GenericFormat {
    fn default() -> GenericFormat {
        GenericFormat {
            channels: Channels::Rgba,
            srgb: false,
            signed: false,
            float: false,
            data_type: DataType::UnsignedByte,
        }
    }
}

impl GenericFormat {
    /// Constructor for `GenericFormat`
    pub fn new(channels: Channels,
               srgb: bool,
               signed: bool,
               float: bool,
               data_type: DataType) -> GenericFormat {
        GenericFormat {
            channels: channels,
            srgb: srgb,
            signed: signed,
            float: float,
            data_type: data_type,
        }
    }

    /// Counts the channels used in the format
    #[inline(always)]
    pub fn num_channels(&self) -> usize {
        self.channels.num_channels()
    }

    /// Create a new uncompressed `SpecificFormat` from `self`
    pub fn none(&self) -> SpecificFormat {
        SpecificFormat {
            which: Which::None(Uncompressed {
                channels: self.channels,
                data_type: self.data_type,
            }),
            srgb: self.srgb,
        }
    }

    /// Create a new RGTC `SpecificFormat` from the properties provided in `self`
    pub fn rgtc(&self) -> ProtocolResult<SpecificFormat> {
        use self::protocol::Rgtc;

        let rgtc = match self.channels {
            Channels::R => {
                if self.signed { Rgtc::RedSigned } else { Rgtc::Red }
            },
            Channels::Rg => {
                if self.signed { Rgtc::RgSigned } else { Rgtc::Rg }
            },
            _ => throw!(ProtocolError::InvalidFormat),
        };

        Ok(SpecificFormat {
            which: Which::Rgtc(rgtc),
            srgb: false, //this compression method doesn't support sRGB
        })
    }

    /// Create a new S3TC `SpecificFormat` from the properties provided in `self`
    pub fn s3tc(&self, version: DXTVersion) -> SpecificFormat {
        use self::protocol::S3tc;

        let s3tc = match version {
            DXTVersion::DXT1 => {
                if self.channels == Channels::Rgba { S3tc::Rgba1 } else { S3tc::Rgb1 }
            }
            DXTVersion::DXT3 => S3tc::Rgba3,
            DXTVersion::DXT5 => S3tc::Rgba5,
        };

        SpecificFormat {
            which: Which::S3tc(s3tc),
            srgb: self.srgb,
        }
    }

    /// Create a new BPTC `SpecificFormat` from the properties provided in `self`
    pub fn bptc(&self) -> SpecificFormat {
        use self::protocol::Bptc;

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
    pub fn astc(&self, blocksize: BlockSize) -> SpecificFormat {
        SpecificFormat {
            which: Which::Astc(blocksize),
            srgb: self.srgb,
        }
    }
}

/// Represents a specific compression format in symbolic form. As in, there are no
/// OpenGL, DirectX or whatever enum values associated with it.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SpecificFormat {
    /// Which format is used
    pub which: Which,
    /// sRGB color space in the format
    pub srgb: bool,
}

impl SpecificFormat {
    /// Consume self and convert specific formats back into generic ones
    pub fn into_generic(self) -> GenericFormat {
        GenericFormat {
            channels: self.which.channels(),
            srgb: self.srgb,
            signed: self.which.signed(),
            float: self.which.float(),
            data_type: match self.which {
                Which::None(uncompressed) => uncompressed.data_type,
                _ => DataType::Unspecified,
            },
        }
    }

    /// Convert specific formats into generic properties
    pub fn to_generic(&self) -> GenericFormat {
        self.clone().into_generic()
    }

    /// Check if this is a compressed format
    pub fn is_compressed(&self) -> bool {
        match self.which {
            Which::None(_) => false,
            _ => true,
        }
    }
}

impl ::std::fmt::Display for SpecificFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        if self.srgb {
            f.write_str("sRGB ")?;
        }

        write!(f, "{} compression", self.which)
    }
}
