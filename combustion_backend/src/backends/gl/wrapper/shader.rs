use super::bindings::types::*;
use super::bindings::*;
use super::GLObject;

use std::mem;
use std::ptr;
use std::ffi::CString;
use std::io::prelude::*;
use std::ops::Deref;
use std::path::{Path, PathBuf};

use super::error::*;
use super::shader_program::*;

/// `GLShader` represents a single shader. It is not a shader program.
#[derive(Eq, PartialEq)]
pub struct GLShader(GLuint);

impl_simple_globject!(GLShader, IsShader);

impl From<GLuint> for GLShader {
    fn from(handle: GLuint) -> GLShader {
        GLShader(handle)
    }
}

/// `GLShaderVariant` represents the supported shader types
#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum GLShaderVariant {
    VertexShader = VERTEX_SHADER,
    FragmentShader = FRAGMENT_SHADER,
    ComputeShader = COMPUTE_SHADER,
    GeometryShader = GEOMETRY_SHADER,
    TessEvaluationShader = TESS_EVALUATION_SHADER,
    TessControlShader = TESS_CONTROL_SHADER,
}

/// `GLShaderInfo` represents the types of info that the shader can be queried for
#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum GLShaderInfo {
    ShaderType = SHADER_TYPE,
    DeleteStatus = DELETE_STATUS,
    CompileStatus = COMPILE_STATUS,
    InfoLogLength = INFO_LOG_LENGTH,
    ShaderSourceLength = SHADER_SOURCE_LENGTH,
}

/// `GLShaderString` represents the strings that can be obtained from a shader
pub enum GLShaderString {
    InfoLog,
    ShaderSource,
}

/// `GLShaderBuilder` can be used to chain together
pub struct GLShaderBuilder(GLShader);

impl GLShaderBuilder {
    #[inline(always)]
    pub fn new(variant: GLShaderVariant) -> GLResult<GLShaderBuilder> {
        Ok(GLShaderBuilder(try_rethrow!(GLShader::new(variant))))
    }

    #[inline(always)]
    pub fn source(mut self, source: String) -> GLResult<Self> {
        try_rethrow!(self.0.set_source(source));

        Ok(self)
    }

    #[inline(always)]
    pub fn compile(mut self) -> GLResult<Self> {
        try_rethrow!(self.0.compile());

        Ok(self)
    }

    #[inline(always)]
    pub fn finish(self) -> GLShader { self.0 }
}

impl GLShader {
    pub fn from_source(source: String, variant: GLShaderVariant) -> GLResult<GLShader> {
        let mut shader: GLShader = try_rethrow!(GLShader::new(variant));

        try_rethrow!(shader.set_source(source));

        try_rethrow!(shader.compile());

        Ok(shader)
    }

    pub fn new(variant: GLShaderVariant) -> GLResult<GLShader> {
        let shader: GLShader = GLShader(unsafe { CreateShader(variant as GLenum) });

        check_gl_errors!();

        Ok(shader)
    }

    pub fn set_source(&mut self, source: String) -> GLResult<()> {
        try_rethrow!(self.check());

        let source_c = CString::new(source).unwrap();

        unsafe { ShaderSource(self.0, 1, &source_c.as_ptr(), ptr::null()); }

        check_gl_errors!();

        Ok(())
    }

    pub fn compile(&mut self) -> GLResult<()> {
        try_rethrow!(self.check());

        unsafe { CompileShader(self.0); }

        check_gl_errors!();

        let status = try_rethrow!(self.get_info(GLShaderInfo::CompileStatus));

        if status != TRUE as GLint {
            panic!("{}", self.get_string(GLShaderString::InfoLog).unwrap());
        }

        Ok(())
    }

    /// Gets a single field from the shader info
    pub fn get_info(&self, field: GLShaderInfo) -> GLResult<GLint> {
        try_rethrow!(self.check());

        let mut status = FALSE as GLint;

        unsafe { GetShaderiv(self.0, field as GLenum, &mut status); }

        check_gl_errors!();

        Ok(status)
    }

    /// Gets a single string from the shader
    ///
    /// Panics if the shader returned a Non-UTF8 string
    pub fn get_string(&self, field: GLShaderString) -> GLResult<String> {
        let len = try_rethrow!(self.get_info(match field {
            GLShaderString::ShaderSource => GLShaderInfo::ShaderSourceLength,
            GLShaderString::InfoLog => GLShaderInfo::InfoLogLength
        }));

        let mut buffer: Vec<u8> = Vec::with_capacity(len as usize);

        unsafe {
            buffer.set_len(len as usize - 1); //Skip NULL-terminator

            match field {
                GLShaderString::ShaderSource => {
                    GetShaderSource(self.0, len, ptr::null_mut(), buffer.as_mut_ptr() as *mut GLchar);
                },
                GLShaderString::InfoLog => {
                    GetShaderInfoLog(self.0, len, ptr::null_mut(), buffer.as_mut_ptr() as *mut GLchar);
                }
            }
        }

        check_gl_errors!();

        Ok(try_throw!(String::from_utf8(buffer)))
    }

    pub fn detach(&mut self, program: &mut GLShaderProgram) -> GLResult<()> {
        try_rethrow!(self.check());

        unsafe { DetachShader(program.raw(), self.0); }

        check_gl_errors!();

        Ok(())
    }

    /// Deletes the shader
    ///
    /// This function is called on Drop
    pub fn delete(&mut self) -> GLResult<()> {
        if self.is_valid() {
            unsafe { DeleteShader(self.0); }

            check_gl_errors!();

            let status = try_rethrow!(self.get_info(GLShaderInfo::DeleteStatus));

            if status != TRUE as GLint {
                panic!("{}", self.get_string(GLShaderString::InfoLog).unwrap());
            }

            check_gl_errors!();
        }

        Ok(())
    }
}

impl Drop for GLShader {
    fn drop(&mut self) {
        self.delete().expect("Could not drop GLShader")
    }
}
