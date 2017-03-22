use super::bindings::types::*;
use super::bindings::*;
use super::{GLObject, GLBindable};

use std::mem;
use std::ptr;
use std::sync::Arc;
use std::os::raw::c_void;

use super::error::*;

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct GLVertexArray(GLuint);

impl_simple_globject!(GLVertexArray, IsVertexArray);

lazy_static! {
    pub static ref DEFAULT_VERTEXARRAY: GLVertexArray = GLVertexArray::default();
}

impl GLBindable for GLVertexArray {
    fn bind(&self) -> GLResult<()> {
        unsafe { BindVertexArray(self.0); }

        check_gl_errors!();

        Ok(())
    }
}

impl GLVertexArray {
    pub fn default() -> GLVertexArray { GLVertexArray(0) }

    pub fn new() -> GLResult<GLVertexArray> {
        let mut vao = 0;

        unsafe { GenVertexArrays(1, &mut vao); }

        check_gl_errors!();

        unsafe { BindVertexArray(vao); }

        check_gl_errors!();

        Ok(GLVertexArray(vao))
    }

    pub fn delete(&mut self) -> GLResult<()> {
        if self.is_valid() {
            unsafe { DeleteVertexArrays(1, &self.0 as *const GLuint); }

            check_gl_errors!();
        }

        Ok(())
    }
}

impl Drop for GLVertexArray {
    fn drop(&mut self) {
        self.delete().expect("Could not drop GLVertexArray")
    }
}
