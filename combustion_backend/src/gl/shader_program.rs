use super::bindings::types::*;
use super::bindings::*;
use super::GLObject;

use std::mem;
use std::ptr;
use std::ffi::CString;

use super::error::*;
use super::shader::*;
use super::uniform::GLUniform;

/// `GLShaderProgram` represents a whole shader program, linked with many shaders
#[derive(Eq, PartialEq)]
pub struct GLShaderProgram(GLuint);

impl_simple_globject!(GLShaderProgram, IsProgram);

#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum GLProgramInfo {
    DeleteStatus = DELETE_STATUS,
    LinkStatus = LINK_STATUS,
    ValidateStatus = VALIDATE_STATUS,
    InfoLogLength = INFO_LOG_LENGTH,
    AttachedShaders = ATTACHED_SHADERS,
    ActiveAttributes = ACTIVE_ATTRIBUTES,
    ActiveAttributeMaxLength = ACTIVE_ATTRIBUTE_MAX_LENGTH,
    ActiveUniforms = ACTIVE_UNIFORMS,
    ActiveUniformMaxLength = ACTIVE_UNIFORM_MAX_LENGTH
}

pub enum GLProgramString {
    InfoLog
}

#[derive(Eq, PartialEq)]
pub struct GLShaderProgramBuilder(GLShaderProgram);

impl GLShaderProgramBuilder {
    #[inline(always)]
    pub fn new() -> GLResult<GLShaderProgramBuilder> {
        Ok(GLShaderProgramBuilder(try_rethrow!(GLShaderProgram::new())))
    }

    #[inline(always)]
    pub fn attach_shader(mut self, shader: GLShader) -> GLResult<GLShaderProgramBuilder> {
        try_rethrow!(self.0.attach_shader(shader));

        Ok(self)
    }

    #[inline(always)]
    pub fn link(mut self) -> GLResult<GLShaderProgramBuilder> {
        try_rethrow!(self.0.link());

        Ok(self)
    }

    #[inline(always)]
    pub fn finish(self) -> GLShaderProgram { self.0 }
}


impl GLShaderProgram {
    pub fn new() -> GLResult<GLShaderProgram> {
        let program: GLShaderProgram = GLShaderProgram(unsafe { CreateProgram() });

        check_gl_errors!();

        Ok(program)
    }

    pub fn use_program(&self) -> GLResult<()> {
        try_rethrow!(self.check());

        unsafe { UseProgram(self.0); }

        check_gl_errors!();

        Ok(())
    }

    pub fn attach_shader(&mut self, shader: GLShader) -> GLResult<()> {
        try_rethrow!(self.check());
        assert!(shader.is_valid());

        unsafe { AttachShader(self.0, shader.into_raw()); }

        check_gl_errors!();

        Ok(())
    }

    pub fn link(&mut self) -> GLResult<()> {
        try_rethrow!(self.check());

        unsafe { LinkProgram(self.0); }

        check_gl_errors!();

        let status = try_rethrow!(self.get_info(GLProgramInfo::LinkStatus));

        if status != TRUE as GLint {
            panic!("{}", self.get_string(GLProgramString::InfoLog).unwrap());
        }

        Ok(())
    }

    pub fn get_info(&self, field: GLProgramInfo) -> GLResult<GLint> {
        try_rethrow!(self.check());

        let mut status = FALSE as GLint;

        unsafe { GetProgramiv(self.0, field as GLenum, &mut status); }

        check_gl_errors!();

        Ok(status)
    }

    pub fn get_string(&self, field: GLProgramString) -> GLResult<String> {
        let len = try_rethrow!(self.get_info(match field {
            GLProgramString::InfoLog => GLProgramInfo::InfoLogLength
        }));

        let mut buffer: Vec<u8> = Vec::with_capacity(len as usize);

        unsafe {
            buffer.set_len(len as usize - 1); //Skip NULL-terminator

            match field {
                GLProgramString::InfoLog => {
                    GetProgramInfoLog(self.0, len, ptr::null_mut(), buffer.as_mut_ptr() as *mut GLchar);
                }
            }
        }

        check_gl_errors!();

        Ok(try_throw!(String::from_utf8(buffer)))
    }

    /// Effectively calls glGetAttachedShaders
    pub fn get_raw_shaders(&self) -> GLResult<Vec<GLuint>> {
        let len = try_rethrow!(self.get_info(GLProgramInfo::AttachedShaders));

        let mut buffer: Vec<GLuint> = Vec::with_capacity(len as usize);
        let mut count: GLsizei = 0;

        unsafe {
            buffer.set_len(len as usize);

            GetAttachedShaders(self.0, len, &mut count, buffer.as_mut_ptr() as *mut GLuint);
        }

        check_gl_errors!();

        assert_eq!(len, count);

        Ok(buffer)
    }

    pub fn get_uniform(&self, name: &str) -> GLResult<GLUniform> {
        let name = try_throw!(CString::new(name));

        let id = unsafe { GetUniformLocation(self.0, name.as_ptr() as *const GLchar) };

        check_gl_errors!();

        Ok(GLUniform(id))
    }

    /// Deletes the shader program
    ///
    /// This function is called on Drop
    pub fn delete(&mut self) -> GLResult<()> {
        if self.is_valid() {
            unsafe { DeleteProgram(self.0); }

            check_gl_errors!();

            //If the current program still exists, at least check if it is queued for deletion...
            if self.is_valid() {
                let status = try_rethrow!(self.get_info(GLProgramInfo::DeleteStatus));

                if status != TRUE as GLint {
                    panic!("{}", self.get_string(GLProgramString::InfoLog).unwrap());
                }

                check_gl_errors!();
            }
        }

        Ok(())
    }
}

impl Drop for GLShaderProgram {
    fn drop(&mut self) {
        self.delete().expect("Could not drop GLShaderProgram")
    }
}
