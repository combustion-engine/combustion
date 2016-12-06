use super::bindings::types::*;
use super::bindings::*;
use super::GLObject;

use std::mem;
use std::ptr;
use std::sync::Arc;
use std::os::raw::c_void;

use super::gl_error::*;

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct GLVertexArray(GLuint);

impl_simple_globject!(GLVertexArray, IsVertexArray, "GLVertexArray");

lazy_static! {
    pub static ref DEFAULT_VERTEXARRAY: GLVertexArray = GLVertexArray::default();
}

impl GLVertexArray {
    pub fn default() -> GLVertexArray { GLVertexArray(0) }

    pub fn new() -> GLResult<GLVertexArray> {
        let mut vao = 0;

        unsafe { GenVertexArrays(1, &mut vao); }

        check_errors!();

        unsafe { BindVertexArray(vao); }

        check_errors!();

        Ok(GLVertexArray(vao))
    }

    pub fn bind(&self) -> GLResult<()> {
        unsafe { BindVertexArray(self.0); }

        check_errors!();

        Ok(())
    }

    pub fn delete(&mut self) -> GLResult<()> {
        if self.is_valid() {
            unsafe { DeleteVertexArrays(1, &self.0 as *const GLuint); }

            check_errors!();
        }

        Ok(())
    }
}

impl Drop for GLVertexArray {
    fn drop(&mut self) {
        self.delete().expect("Could not drop GLVertexArray")
    }
}
