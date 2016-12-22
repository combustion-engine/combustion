//! OpenGL extensions and such that aren't covered by the standard bindings
//!
//! References:
//!
//! * https://www.opengl.org/registry/specs/EXT/texture_filter_anisotropic.txt
//! * https://www.opengl.org/registry/specs/ARB/texture_compression_bptc.txt
//! * http://www.opengl.org/registry/specs/EXT/texture_compression_s3tc.txt
//! * https://www.opengl.org/registry/specs/KHR/texture_compression_astc_hdr.txt
//!
#![allow(bad_style, clippy)]

use super::gl_bindings::*;

use std::mem;
use std::os::raw;

pub const EXT_texture_filter_anisotropic: &'static str = "GL_EXT_texture_filter_anisotropic";
pub const EXT_texture_compression_s3tc: &'static str = "GL_EXT_texture_compression_s3tc";
pub const KHR_texture_compression_astc_hdr: &'static str = "GL_KHR_texture_compression_astc_hdr";
pub const KHR_texture_compression_astc_ldr: &'static str = "GL_KHR_texture_compression_astc_ldr";

pub const TEXTURE_MAX_ANISOTROPY_EXT: types::GLenum = 0x84FE;
pub const MAX_TEXTURE_MAX_ANISOTROPY_EXT: types::GLenum = 0x84FF;

pub const COMPRESSED_RGB_S3TC_DXT1_EXT: types::GLenum = 0x83F0;
pub const COMPRESSED_RGBA_S3TC_DXT1_EXT: types::GLenum = 0x83F1;
pub const COMPRESSED_RGBA_S3TC_DXT3_EXT: types::GLenum = 0x83F2;
pub const COMPRESSED_RGBA_S3TC_DXT5_EXT: types::GLenum = 0x83F3;

pub const COMPRESSED_SRGB_S3TC_DXT1_EXT: types::GLenum = 0x8C4C;
pub const COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT: types::GLenum = 0x8C4D;
pub const COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT: types::GLenum = 0x8C4E;
pub const COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT: types::GLenum = 0x8C4F;

pub const COMPRESSED_RGBA_ASTC_4x4_KHR: types::GLenum = 0x93B0;
pub const COMPRESSED_RGBA_ASTC_5x4_KHR: types::GLenum = 0x93B1;
pub const COMPRESSED_RGBA_ASTC_5x5_KHR: types::GLenum = 0x93B2;
pub const COMPRESSED_RGBA_ASTC_6x5_KHR: types::GLenum = 0x93B3;
pub const COMPRESSED_RGBA_ASTC_6x6_KHR: types::GLenum = 0x93B4;
pub const COMPRESSED_RGBA_ASTC_8x5_KHR: types::GLenum = 0x93B5;
pub const COMPRESSED_RGBA_ASTC_8x6_KHR: types::GLenum = 0x93B6;
pub const COMPRESSED_RGBA_ASTC_8x8_KHR: types::GLenum = 0x93B7;
pub const COMPRESSED_RGBA_ASTC_10x5_KHR: types::GLenum = 0x93B8;
pub const COMPRESSED_RGBA_ASTC_10x6_KHR: types::GLenum = 0x93B9;
pub const COMPRESSED_RGBA_ASTC_10x8_KHR: types::GLenum = 0x93BA;
pub const COMPRESSED_RGBA_ASTC_10x10_KHR: types::GLenum = 0x93BB;
pub const COMPRESSED_RGBA_ASTC_12x10_KHR: types::GLenum = 0x93BC;
pub const COMPRESSED_RGBA_ASTC_12x12_KHR: types::GLenum = 0x93BD;

pub const COMPRESSED_SRGB8_ALPHA8_ASTC_4x4_KHR: types::GLenum = 0x93D0;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_5x4_KHR: types::GLenum = 0x93D1;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_5x5_KHR: types::GLenum = 0x93D2;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_6x5_KHR: types::GLenum = 0x93D3;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_6x6_KHR: types::GLenum = 0x93D4;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_8x5_KHR: types::GLenum = 0x93D5;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_8x6_KHR: types::GLenum = 0x93D6;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_8x8_KHR: types::GLenum = 0x93D7;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_10x5_KHR: types::GLenum = 0x93D8;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_10x6_KHR: types::GLenum = 0x93D9;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_10x8_KHR: types::GLenum = 0x93DA;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_10x10_KHR: types::GLenum = 0x93DB;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_12x10_KHR: types::GLenum = 0x93DC;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_12x12_KHR: types::GLenum = 0x93DD;

mod storage {
    use super::super::gl_bindings::{FnPtr, missing_fn_panic};
    use std::os::raw;

    pub static mut wglSwapIntervalEXT: FnPtr = FnPtr {
        f: missing_fn_panic as *const raw::c_void,
        is_loaded: false
    };
}

#[allow(non_snake_case)]
pub mod wglSwapIntervalEXT {
    use super::storage;
    use std::os::raw;
    use super::super::gl_bindings::{FnPtr, metaloadfn};

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::wglSwapIntervalEXT.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
        unsafe {
            storage::wglSwapIntervalEXT = FnPtr::new(metaloadfn(&mut loadfn, "wglSwapIntervalEXT", &[]))
        }
    }
}

#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn wglSwapIntervalEXT(interval: types::GLint) -> types::GLint {
    mem::transmute::<_, extern "system" fn(types::GLint) -> types::GLint>(storage::wglSwapIntervalEXT.f)(interval)
}

#[allow(dead_code)]
pub fn load_extras_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const raw::c_void {
    wglSwapIntervalEXT::load_with(|s| loadfn(s));
}