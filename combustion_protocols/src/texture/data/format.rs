use ::error::{ProtocolResult, ProtocolError};

use ::texture::protocol::{self, Channels, DataType};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Uncompressed {
    pub format: Channels,
    pub data_type: DataType,
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

/// Represents a non-sRGB compression format in symbolic form
#[derive(Debug, Clone, Copy)]
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
            Which::None(uncompressed) => uncompressed.format,
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
        use self::protocol::*;

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
        use self::protocol::*;

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
#[derive(Debug, Clone, Copy)]
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
    data_type: DataType,
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
            data_type: DataType::UnsignedByte,
        }
    }
}

impl GenericFormat {
    pub fn new(channels: Channels,
               srgb: bool,
               blocksize: Option<protocol::BlockSize>,
               signed: bool,
               float: bool,
               version: u8,
               data_type: DataType) -> GenericFormat {
        GenericFormat {
            channels: channels,
            srgb: srgb,
            blocksize: blocksize,
            signed: signed,
            float: float,
            version: version,
            data_type: data_type,
        }
    }

    #[inline(always)]
    pub fn num_channels(&self) -> usize {
        self.channels.num_channels()
    }

    /// Create a new uncompressed `SpecificFormat` from `self`
    pub fn none(&self) -> SpecificFormat {
        SpecificFormat {
            which: Which::None(Uncompressed {
                format: self.channels,
                data_type: self.data_type,
            }),
            srgb: self.srgb,
        }
    }

    /// Create a new RGTC `SpecificFormat` from the properties provided in `self`
    pub fn rgtc(&self) -> ProtocolResult<SpecificFormat> {
        use self::protocol::*;

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
    pub fn s3tc(&self) -> ProtocolResult<SpecificFormat> {
        use self::protocol::*;

        let s3tc = match self.version {
            1 => {
                if self.channels == Channels::Rgba { S3tc::Rgba1 } else { S3tc::Rgb1 }
            }
            3 => S3tc::Rgba3,
            5 => S3tc::Rgba5,
            _ => throw!(ProtocolError::InvalidFormat),
        };

        Ok(SpecificFormat {
            which: Which::S3tc(s3tc),
            srgb: self.srgb,
        })
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
    pub fn astc(&self) -> ProtocolResult<SpecificFormat> {
        if let Some(blocksize) = self.blocksize {
            Ok(SpecificFormat {
                which: Which::Astc(blocksize),
                srgb: self.srgb,
            })
        } else {
            throw!(ProtocolError::NotPresent);
        }
    }
}

/// Represents a specific compression format in symbolic form. As in, there are no
/// OpenGL, DirectX or whatever enum values associated with it.
#[derive(Debug, Clone, Copy)]
pub struct SpecificFormat {
    pub which: Which,
    pub srgb: bool,
}

impl SpecificFormat {
    /// Consume self and convert specific formats back into generic ones
    pub fn into_generic(self) -> GenericFormat {
        use self::protocol::S3tc;

        GenericFormat {
            channels: self.which.channels(),
            srgb: self.srgb,
            blocksize: match self.which {
                Which::Astc(blocksize) => Some(blocksize),
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
            },
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
