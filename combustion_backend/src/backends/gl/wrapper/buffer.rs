use super::bindings::types::*;
use super::bindings::*;
use super::GLObject;

use std::mem;
use std::ptr;
use std::os::raw::c_void;

use super::error::*;

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
pub struct GLBuffer(GLuint, GLBufferTarget, usize);

impl_simple_globject!(GLBuffer, IsBuffer);

impl GLBuffer {
    /// Create a new empty OpenGL Buffer and bind it
    pub fn new(target: GLBufferTarget) -> GLResult<GLBuffer> {
        let mut buffer = 0;

        unsafe { GenBuffers(1, &mut buffer); }

        check_gl_errors!();

        unsafe { BindBuffer(target as GLenum, buffer); }

        check_gl_errors!();

        Ok(GLBuffer(buffer, target, 0))
    }

    /// Returns the buffer target.
    #[inline]
    pub fn target(&self) -> GLBufferTarget { self.1 }

    pub fn bind(&self) -> GLResult<()> {
        try_rethrow!(self.check());

        unsafe { BindBuffer(self.1 as GLenum, self.0); }

        check_gl_errors!();

        Ok(())
    }

    /// Returns the last number of bytes buffered
    #[inline(always)]
    pub fn num_bytes(&self) -> usize { self.2 }

    /// Returns the last number of elements `T` buffered
    ///
    /// It's up to you to keep track of type `T`, as `GLBuffer` really only stores the number of bytes, not elements, buffered.
    #[inline(always)]
    pub fn num_elements<T>(&self) -> usize { self.2 / mem::size_of::<T>() }

    /// Buffer a slice of `T` to the `GLBuffer`
    #[inline]
    pub fn buffer_slice<T>(&mut self, data: &[T], usage: GLBufferUsage) -> GLResult<()> {
        unsafe { self.buffer_raw(data.as_ptr() as *const c_void, data.len() * mem::size_of::<T>(), usage) }
    }

    /// Buffer raw data to the `GLBuffer`
    pub unsafe fn buffer_raw(&mut self, data: *const c_void, size: usize, usage: GLBufferUsage) -> GLResult<()> {
        if data.is_null() || size == 0 {
            throw!(GLError::InvalidValue)
        } else {
            try_rethrow!(self.bind());

            BufferData(self.1 as GLenum, size as GLsizeiptr, data, usage as GLenum);

            check_gl_errors!();

            self.2 = size;

            Ok(())
        }
    }

    pub fn delete(&mut self) -> GLResult<()> {
        if self.is_valid() {
            unsafe { DeleteBuffers(1, &self.0 as *const GLuint); }

            check_gl_errors!();
        }

        Ok(())
    }
}

impl Drop for GLBuffer {
    fn drop(&mut self) {
        self.delete().expect("Could not drop GLBuffer")
    }
}
