use super::bindings::types::*;
use super::bindings::*;
use super::GLObject;

use std::mem;
use std::ptr;

use super::gl_error::*;
use super::gl_shader::*;

//pub enum GLRenderbufferTarget {}

#[derive(Eq, PartialEq)]
pub struct GLRenderbuffer(GLuint);

impl_simple_globject!(GLRenderbuffer, IsRenderbuffer, "GLRenderBuffer");

impl GLRenderbuffer {
    pub fn new() -> GLResult<GLRenderbuffer> {
        let mut buffer: GLuint = 0;

        unsafe { GenRenderbuffers(1, &mut buffer as *mut _); }

        check_errors!();

        unsafe { BindRenderbuffer(RENDERBUFFER, buffer); }

        check_errors!();

        Ok(GLRenderbuffer(buffer))
    }

    pub fn bind(&self) -> GLResult<()> {
        unsafe { BindRenderbuffer(RENDERBUFFER, self.0); }

        check_errors!();

        Ok(())
    }

    pub fn set_storage(&mut self, width: usize, height: usize) -> GLResult<()> {
        try!(self.bind());

        unsafe {
            RenderbufferStorage(RENDERBUFFER,
                                DEPTH24_STENCIL8,
                                width as GLsizei,
                                height as GLsizei);
        }

        check_errors!();

        Ok(())
    }

    pub fn delete(&mut self) -> GLResult<()> {
        if self.is_valid() {
            unsafe { DeleteRenderbuffers(1, &self.0 as *const _); }

            check_errors!();
        }

        Ok(())
    }
}

impl Drop for GLRenderbuffer {
    fn drop(&mut self) {
        self.delete().expect("Could not drop GLRenderbuffer")
    }
}