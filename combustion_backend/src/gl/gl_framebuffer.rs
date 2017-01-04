use super::bindings::types::*;
use super::bindings::*;
use super::GLObject;

use std::mem;
use std::ptr;

use super::gl_error::*;
use super::gl_shader::*;
use super::gl_renderbuffer::*;

#[derive(Eq, PartialEq)]
pub struct GLFramebuffer(GLuint);

impl GLObject for GLFramebuffer {
    #[inline(always)]
    fn raw(&self) -> GLuint { self.0 }

    #[inline(always)]
    fn into_raw(mut self) -> GLuint {
        mem::replace(&mut self.0, 0)
    }

    #[inline(always)]
    fn is_valid(&self) -> bool {
        // This one is special since the value 0 is a valid framebuffer
        TRUE == unsafe { IsFramebuffer(self.0) } || self.0 == 0
    }

    #[inline(always)]
    fn check(&self) -> GLResult<()> {
        if self.is_valid() { Ok(()) } else {
            error!("Invalid GLFramebuffer");
            Err(GLError::InvalidValue)
        }
    }
}

//impl_simple_globject!(GLFramebuffer, IsFramebuffer, "GLFramebuffer");

lazy_static! {
    pub static ref DEFAULT_FRAMEBUFFER: GLFramebuffer = GLFramebuffer::default();
}

impl GLFramebuffer {
    pub fn default() -> GLFramebuffer { GLFramebuffer(0) }

    pub fn new() -> GLResult<GLFramebuffer> {
        let mut framebuffer: GLuint = 0;

        unsafe { GenFramebuffers(1, &mut framebuffer as *mut _); }

        check_gl_errors!();

        unsafe { BindFramebuffer(FRAMEBUFFER, framebuffer); }

        check_gl_errors!();

        Ok(GLFramebuffer(framebuffer))
    }

    pub fn bind(&self) -> GLResult<()> {
        try!(self.check());

        unsafe { BindFramebuffer(FRAMEBUFFER, self.0); }

        check_gl_errors!();

        Ok(())
    }

    pub fn is_complete(&self) -> GLResult<bool> {
        try!(self.bind());

        Ok(FRAMEBUFFER_COMPLETE == unsafe { CheckFramebufferStatus(FRAMEBUFFER) })
    }

    pub fn renderbuffer(&mut self, renderbuffer: &GLRenderbuffer) -> GLResult<()> {
        try!(self.bind());

        unsafe {
            FramebufferRenderbuffer(FRAMEBUFFER,
                                    DEPTH_STENCIL_ATTACHMENT,
                                    RENDERBUFFER,
                                    renderbuffer.raw());
        }

        Ok(())
    }

    pub fn delete(&mut self) -> GLResult<()> {
        if self.is_valid() && self.0 != 0 {
            unsafe {
                DeleteFramebuffers(1, &mut self.0 as *mut _);
            }

            check_gl_errors!();
        }

        Ok(())
    }
}

impl Drop for GLFramebuffer {
    fn drop(&mut self) {
        self.delete().expect("Could not drop GLFramebuffer")
    }
}