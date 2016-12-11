//! Inner protocol components

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