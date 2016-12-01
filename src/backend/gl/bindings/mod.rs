use std::os::raw::c_void;

pub mod gl_bindings;
pub mod extra_gl_bindings;

pub use self::gl_bindings::*;
pub use self::extra_gl_bindings::*;

#[allow(dead_code)]
pub fn load_all_with<F>(mut loadfn: F) where F: FnMut(&str) -> *const c_void {
    load_with(|module| loadfn(module));
    load_extras_with(|module| loadfn(module));
}