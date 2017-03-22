use super::wrapper::types;
use super::wrapper::error::*;

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

pub trait GLBindable {
    fn bind(&self) -> GLResult<()>;
}