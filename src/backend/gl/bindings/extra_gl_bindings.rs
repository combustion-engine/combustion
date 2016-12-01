#![allow(bad_style)]

use super::gl_bindings::*;

use std::mem;
use std::os::raw;

pub const MAX_TEXTURE_MAX_ANISOTROPY_EXT: types::GLenum = 0x84FF;
pub const TEXTURE_MAX_ANISOTROPY_EXT: types::GLenum = 0x84FE;

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