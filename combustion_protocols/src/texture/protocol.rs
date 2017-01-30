//! Inner protocol components
use std::str::FromStr;

use phf;

include!(concat!(env!("OUT_DIR"), "/protocols/texture_capnp.rs"));

pub static BLOCKSIZES: phf::Map<&'static str, BlockSize> = phf_map! {
    "4x4"   => BlockSize::B4x4,   "b4x4"   => BlockSize::B4x4,   "B4x4"   => BlockSize::B4x4,
    "5x4"   => BlockSize::B5x4,   "b5x4"   => BlockSize::B5x4,   "B5x4"   => BlockSize::B5x4,
    "5x5"   => BlockSize::B5x5,   "b5x5"   => BlockSize::B5x5,   "B5x5"   => BlockSize::B5x5,
    "6x5"   => BlockSize::B6x5,   "b6x5"   => BlockSize::B6x5,   "B6x5"   => BlockSize::B6x5,
    "6x6"   => BlockSize::B6x6,   "b6x6"   => BlockSize::B6x6,   "B6x6"   => BlockSize::B6x6,
    "8x5"   => BlockSize::B8x5,   "b8x5"   => BlockSize::B8x5,   "B8x5"   => BlockSize::B8x5,
    "8x6"   => BlockSize::B8x6,   "b8x6"   => BlockSize::B8x6,   "B8x6"   => BlockSize::B8x6,
    "10x5"  => BlockSize::B10x5,  "b10x5"  => BlockSize::B10x5,  "B10x5"  => BlockSize::B10x5,
    "10x6"  => BlockSize::B10x6,  "b10x6"  => BlockSize::B10x6,  "B10x6"  => BlockSize::B10x6,
    "8x8"   => BlockSize::B8x8,   "b8x8"   => BlockSize::B8x8,   "B8x8"   => BlockSize::B8x8,
    "10x8"  => BlockSize::B10x8,  "b10x8"  => BlockSize::B10x8,  "B10x8"  => BlockSize::B10x8,
    "10x10" => BlockSize::B10x10, "b10x10" => BlockSize::B10x10, "B10x10" => BlockSize::B10x10,
    "12x10" => BlockSize::B12x10, "b12x10" => BlockSize::B12x10, "B12x10" => BlockSize::B12x10,
    "12x12" => BlockSize::B12x12, "b12x12" => BlockSize::B12x12, "B12x12" => BlockSize::B12x12,
};

impl FromStr for BlockSize {
    type Err = &'static str;

    /// Accepts strings in the form `4x4`, `b4x4`, or `B4x4`
    fn from_str(s: &str) -> Result<BlockSize, Self::Err> {
        BLOCKSIZES.get(s).cloned().ok_or("Invalid BlockSize")
    }
}

impl BlockSize {
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

impl ::std::fmt::Debug for BlockSize {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str(self.to_str())
    }
}

impl ::std::fmt::Display for BlockSize {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{} BlockSize", self.to_str())
    }
}

macro_rules! impl_enum_fmt {
    ($target:ident { $($key:ident => $msg:expr,)* }) => {
        impl ::std::fmt::Display for $target {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str(match *self {
                    $($target::$key => $msg),*
                })
            }
        }

        impl ::std::fmt::Debug for $target {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{} {}", stringify!($target), match *self {
                    $($target::$key => stringify!($key)),*
                })
            }
        }
    }
}

impl_enum_fmt!(DataType {
    UnsignedByte          => "UNSIGNED_BYTE",
    Byte                  => "BYTE",
    UnsignedShort         => "UNSIGNED_SHORT",
    Short                 => "SHORT",
    UnsignedInt           => "UNSIGNED_INT",
    Int                   => "INT",
    Float                 => "FLOAT",
    UnsignedByte332       => "UNSIGNED_BYTE_3_3_2",
    UnsignedByte233Rev    => "UNSIGNED_BYTE_2_3_3_REV",
    UnsignedShort565      => "UNSIGNED_SHORT_5_6_5",
    UnsignedShort565Rev   => "UNSIGNED_SHORT_5_6_5_REV",
    UnsignedShort4444     => "UNSIGNED_SHORT_4_4_4_4",
    UnsignedShort4444Rev  => "UNSIGNED_SHORT_4_4_4_4_REV",
    UnsignedShort5551     => "UNSIGNED_SHORT_5_5_5_1",
    UnsignedShort1555Rev  => "UNSIGNED_SHORT_1_5_5_5_REV",
    UnsignedInt8888       => "UNSIGNED_INT_8_8_8_8",
    UnsignedInt8888Rev    => "UNSIGNED_INT_8_8_8_8_REV",
    UnsignedInt1010102    => "UNSIGNED_INT_10_10_10_2",
    UnsignedInt2101010Rev => "UNSIGNED_INT_2_10_10_10_REV",
    Unspecified           => "Unspecified",
});

impl_enum_fmt!(Channels {
    R     => "Red Channel",
    Rg    => "Red-green Channels",
    Rgb   => "Rgb Channels",
    Rgba  => "Rgba Channels",
});

impl_enum_fmt!(Rgtc {
    Red       => "Unsigned normalized 1-component",
    RedSigned => "Signed normalized   1-component",
    Rg        => "Unsigned normalized 2-component",
    RgSigned  => "Signed normalized   2-component",
});

impl_enum_fmt!(Bptc {
    Rgba             => "RGBA",
    RgbFloatSigned   => "RGB Float Signed",
    RgbFloatUnsigned => "RGB Float Unsigned",
});

impl_enum_fmt!(S3tc {
    Rgb1  => "DXT1 RGB",
    Rgba1 => "DXT1 RGBA",
    Rgba3 => "DXT3 RGBA",
    Rgba5 => "DXT5 RGBA",
});

impl_enum_fmt!(TextureKind {
    Texture1D => "1D Texture",
    Texture2D => "2D Texture",
    Texture3D => "3D Texture",
});