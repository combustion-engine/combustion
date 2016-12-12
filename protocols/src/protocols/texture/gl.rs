//! OpenGL specific texture components

use backend::gl::types::*;
use backend::gl::bindings as glb;

use super::protocol::{BlockSize, Bptc, Rgtc, S3tc};
use super::{Channels, Which, GenericFormat, SpecificFormat};

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
            srgb: srgb,
        }
    }
}

pub trait GLCompressedGenericFormats {
    /// Get a generic format like `RED`, `RG`, `RGB`, `RGBA`
    fn generic(&self) -> GLuint;

    /// Get a compressed generic format equivalent to the compressed version of `generic`
    fn auto(&self) -> GLuint;
}

impl GLCompressedGenericFormats for GenericFormat {
    fn generic(&self) -> GLuint {
        match self.channels {
            Channels::R => glb::RED,
            Channels::Rg => glb::RG,
            Channels::Rgb => glb::RGB,
            Channels::Rgba => glb::RGBA,
        }
    }

    fn auto(&self) -> GLuint {
        match self.channels {
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
}

/// OpenGL extension to `SpecificFormat` to create a `GLenum` value for passing to `glTexImage*` functions
pub trait GLCompressedSpecificFormats {
    /// Get specific internal texture format enum for the given symbolic format
    fn specific(&self) -> GLuint;
}

impl GLCompressedSpecificFormats for SpecificFormat {
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