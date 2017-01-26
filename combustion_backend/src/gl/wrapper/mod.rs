pub use super::bindings;
pub use super::types;

#[macro_use]
pub mod error;

pub use self::error::*;

pub trait GLObject {
    fn raw(&self) -> types::GLuint;
    fn into_raw(self) -> types::GLuint;
    fn is_valid(&self) -> bool;

    fn check(&self) -> GLResult<()> {
        if self.is_valid() { Ok(()) } else {
            throw!(GLError::InvalidValue)
        }
    }
}

macro_rules! impl_simple_globject {
    ($name:ident, $is:ident $(, { $extra_cond:expr } )*) => {
        impl $crate::gl::wrapper::GLObject for $name {
            #[inline(always)]
            fn raw(&self) -> GLuint { self.0 }

            #[inline(always)]
            fn into_raw(mut self) -> GLuint {
                ::std::mem::replace(&mut self.0, 0)
            }

            #[inline(always)]
            fn is_valid(&self) -> bool {
                $($extra_cond(self) ||)* TRUE == unsafe { $crate::gl::bindings::$is(self.0) }
            }
        }
    }
}

pub mod debug;
pub mod requires;

pub mod vertex_array;
pub mod shader;
pub mod shader_program;
pub mod texture;
pub mod renderbuffer;
pub mod framebuffer;
pub mod buffer;

pub mod uniform;

pub use self::debug::*;
pub use self::requires::*;
pub use self::vertex_array::*;
pub use self::shader::*;
pub use self::shader_program::*;
pub use self::texture::*;
pub use self::renderbuffer::*;
pub use self::framebuffer::*;
pub use self::buffer::*;
pub use self::uniform::*;