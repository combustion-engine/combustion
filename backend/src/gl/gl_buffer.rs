use super::bindings::types::*;
use super::bindings::*;
use super::GLObject;

use std::mem;
use std::ptr;
use std::sync::Arc;
use std::os::raw::c_void;

use super::gl_error::*;

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum GLBufferTarget {
    ArrayBuffer = ARRAY_BUFFER,
    AtomicCounterBuffer = ATOMIC_COUNTER_BUFFER,
    CopyReadBuffer = COPY_READ_BUFFER,
    CopyWriteBuffer = COPY_WRITE_BUFFER,
    DispatchIndirectBuffer = DISPATCH_INDIRECT_BUFFER,
    DrawIndirectBuffer = DRAW_INDIRECT_BUFFER,
    ElementArrayBuffer = ELEMENT_ARRAY_BUFFER,
    PixelPackBuffer = PIXEL_PACK_BUFFER,
    QueryBuffer = QUERY_BUFFER,
    ShaderStorageBuffer = SHADER_STORAGE_BUFFER,
    TextureBuffer = TEXTURE_BUFFER,
    TransformFeedbackBuffer = TRANSFORM_FEEDBACK_BUFFER,
    UniformBuffer = UNIFORM_BUFFER
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum GLBufferUsage {
    StreamDraw = STREAM_DRAW,
    StreamRead = STREAM_READ,
    StreamCopy = STREAM_COPY,
    StaticDraw = STATIC_DRAW,
    StaticRead = STATIC_READ,
    StaticCopy = STATIC_COPY,
    DynamicDraw = DYNAMIC_DRAW,
    DynamicRead = DYNAMIC_READ,
    DynamicCopy = DYNAMIC_COPY,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct GLBuffer(GLuint, GLBufferTarget);

impl_simple_globject!(GLBuffer, IsBuffer, "GLBuffer");

impl GLBuffer {
    pub fn new(target: GLBufferTarget) -> GLResult<GLBuffer> {
        let mut buffer = 0;

        unsafe { GenBuffers(1, &mut buffer); }

        check_errors!();

        unsafe { BindBuffer(target as GLenum, buffer); }

        check_errors!();

        Ok(GLBuffer(buffer, target))
    }

    #[inline]
    pub fn target(&self) -> GLBufferTarget { self.1 }

    pub fn bind(&self) -> GLResult<()> {
        try!(self.check());

        unsafe { BindBuffer(self.1 as GLenum, self.0); }

        check_errors!();

        Ok(())
    }

    pub fn buffer_elements<T>(&mut self, data: &Vec<T>, usage: GLBufferUsage) -> GLResult<()> {
        try!(self.bind());

        unsafe {
            BufferData(self.1 as GLenum,
                       (data.len() * mem::size_of::<T>()) as GLsizeiptr,
                       data.as_ptr() as *const c_void,
                       usage as GLenum
            );
        }

        check_errors!();

        Ok(())
    }

    pub fn buffer_slice<T>(&mut self, data: &[T], usage: GLBufferUsage) -> GLResult<()> {
        try!(self.bind());

        unsafe {
            BufferData(self.1 as GLenum,
                       (data.len() * mem::size_of::<T>()) as GLsizeiptr,
                       data.as_ptr() as *const c_void,
                       usage as GLenum
            );
        }

        check_errors!();

        Ok(())
    }

    pub unsafe fn buffer_raw(&mut self, size: usize, data: *const c_void, usage: GLBufferUsage) -> GLResult<()> {
        if data.is_null() || size == 0 {
            Err(GLError::InvalidValue)
        } else {
            try!(self.bind());

            BufferData(self.1 as GLenum, size as GLsizeiptr, data, usage as GLenum);

            check_errors!();

            Ok(())
        }
    }

    pub fn delete(&mut self) -> GLResult<()> {
        if self.is_valid() {
            unsafe { DeleteBuffers(1, &self.0 as *const GLuint); }

            check_errors!();
        }

        Ok(())
    }
}

impl Drop for GLBuffer {
    fn drop(&mut self) {
        self.delete().expect("Could not drop GLBuffer")
    }
}
