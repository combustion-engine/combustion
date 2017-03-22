use super::bindings::types::*;
use super::bindings::*;
use super::{GLObject, GLBindable};

use std::mem;
use std::ptr;

use super::error::*;
use super::shader::*;

//pub enum GLRenderbufferTarget {}

#[derive(Eq, PartialEq)]
pub struct GLRenderbuffer(GLuint);

impl_simple_globject!(GLRenderbuffer, IsRenderbuffer);

impl GLBindable for GLRenderbuffer {
    fn bind(&self) -> GLResult<()> {
        unsafe { BindRenderbuffer(RENDERBUFFER, self.0); }

        check_gl_errors!();

        Ok(())
    }
}

impl GLRenderbuffer {
    pub fn new() -> GLResult<GLRenderbuffer> {
        let mut buffer: GLuint = 0;

        unsafe { GenRenderbuffers(1, &mut buffer as *mut _); }

        check_gl_errors!();

        unsafe { BindRenderbuffer(RENDERBUFFER, buffer); }

        check_gl_errors!();

        Ok(GLRenderbuffer(buffer))
    }

    pub fn set_storage(&mut self, width: usize, height: usize) -> GLResult<()> {
        try_rethrow!(self.bind());

        unsafe {
            RenderbufferStorage(RENDERBUFFER,
                                DEPTH24_STENCIL8,
                                width as GLsizei,
                                height as GLsizei);
        }

        check_gl_errors!();

        Ok(())
    }

    pub fn delete(&mut self) -> GLResult<()> {
        if self.is_valid() {
            unsafe { DeleteRenderbuffers(1, &self.0 as *const _); }

            check_gl_errors!();
        }

        Ok(())
    }
}

impl Drop for GLRenderbuffer {
    fn drop(&mut self) {
        self.delete().expect("Could not drop GLRenderbuffer")
    }
}