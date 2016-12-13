pub mod bindings;

pub use self::bindings::types as types;

#[macro_use]
pub mod gl_error;

pub use self::gl_error::*;

pub trait GLObject {
    fn raw(&self) -> types::GLuint;
    fn into_raw(self) -> types::GLuint;
    fn is_valid(&self) -> bool;

    #[inline(always)]
    fn check(&self) -> GLResult<()> {
        if self.is_valid() { Ok(()) } else {
            error!("Invalid GLObject");
            Err(GLError::InvalidValue)
        }
    }
}

#[macro_use]
pub mod macros;

pub mod gl_debug;
pub mod gl_buffer;
pub mod gl_vertexarray;
pub mod gl_shader;
pub mod gl_uniform;
pub mod gl_shader_program;
pub mod gl_texture;

pub mod gl_renderbuffer;
pub mod gl_framebuffer;

pub use self::gl_debug::*;
pub use self::gl_vertexarray::*;
pub use self::gl_buffer::*;
pub use self::gl_shader::*;
pub use self::gl_uniform::*;
pub use self::gl_shader_program::*;
pub use self::gl_texture::*;
pub use self::gl_renderbuffer::*;
pub use self::gl_framebuffer::*;