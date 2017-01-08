use super::bindings::types::*;
use super::bindings::*;
use super::GLObject;

use std::mem;
use std::ptr;

use super::error::*;
use super::shader::*;
use super::renderbuffer::*;

#[derive(Eq, PartialEq)]
pub struct GLFramebuffer(GLuint);

#[inline(always)]
fn is_default_framebuffer(framebuffer: &GLFramebuffer) -> bool {
    framebuffer.0 == 0
}

impl_simple_globject!(GLFramebuffer, IsFramebuffer, { is_default_framebuffer });

lazy_static! {
    pub static ref DEFAULT_FRAMEBUFFER: GLFramebuffer = GLFramebuffer::default();
}

impl GLFramebuffer {
    pub fn default() -> GLFramebuffer {
        GLFramebuffer(0)
    }

    pub fn new() -> GLResult<GLFramebuffer> {
        let mut framebuffer: GLuint = 0;

        unsafe { GenFramebuffers(1, &mut framebuffer as *mut _); }

        check_gl_errors!();

        unsafe { BindFramebuffer(FRAMEBUFFER, framebuffer); }

        check_gl_errors!();

        Ok(GLFramebuffer(framebuffer))
    }

    pub fn bind(&self) -> GLResult<()> {
        try_rethrow!(self.check());

        unsafe { BindFramebuffer(FRAMEBUFFER, self.0); }

        check_gl_errors!();

        Ok(())
    }

    pub fn is_complete(&self) -> GLResult<bool> {
        try_rethrow!(self.bind());

        Ok(FRAMEBUFFER_COMPLETE == unsafe { CheckFramebufferStatus(FRAMEBUFFER) })
    }

    pub fn renderbuffer(&mut self, renderbuffer: &GLRenderbuffer) -> GLResult<()> {
        try_rethrow!(self.bind());

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