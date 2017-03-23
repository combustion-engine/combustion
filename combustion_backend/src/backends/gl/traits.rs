use super::wrapper::types;
use super::wrapper::error::*;

/// Defines common routines for OpenGL objects
pub trait GLObject {
    /// Get the raw OpenGL identifier
    fn raw(&self) -> types::GLuint;
    /// Consume self and return the raw OpenGL identifier
    fn into_raw(self) -> types::GLuint;
    /// Check if the current object is a valid OpenGL object
    fn is_valid(&self) -> bool;

    /// Return a `Result` representing if the current object is a valid OpenGL object
    fn check(&self) -> GLResult<()> {
        if self.is_valid() { Ok(()) } else {
            throw!(GLError::InvalidValue)
        }
    }
}

/// Defines an object which can be made active in the OpenGL context
pub trait GLBindable {
    /// Bind and make active the current object
    fn bind(&self) -> GLResult<()>;
}