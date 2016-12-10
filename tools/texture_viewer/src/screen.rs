use std::mem;
use std::ptr;

use backend::gl::*;
use backend::gl::types::*;
use backend::gl::bindings as glb;

static QUAD_DATA: [f32; 20] = [
    -1.0, 1.0, 0.0, 0.0, 1.0,
    -1.0, -1.0, 0.0, 0.0, 0.0,
    1.0, 1.0, 0.0, 1.0, 1.0,
    1.0, -1.0, 0.0, 1.0, 0.0,
];

pub struct ScreenQuad {
    vao: GLVertexArray,
    #[allow(dead_code)]
    buffer: GLBuffer,
}

impl ScreenQuad {
    pub fn new() -> GLResult<ScreenQuad> {
        let vao = try!(GLVertexArray::new());

        try!(vao.bind());

        let mut buffer = try!(GLBuffer::new(GLBufferTarget::ArrayBuffer));

        try!(buffer.bind());

        try!(buffer.buffer_slice(&QUAD_DATA, GLBufferUsage::StaticDraw));

        let stride = 5 * mem::size_of::<f32>() as GLsizei;

        unsafe {
            glb::EnableVertexAttribArray(0);
            glb::VertexAttribPointer(0, 3, glb::FLOAT, glb::FALSE, stride, ptr::null());
        }

        check_errors!();

        unsafe {
            let offset = ptr::null::<f32>().offset(3);

            glb::EnableVertexAttribArray(1);
            glb::VertexAttribPointer(1, 2, glb::FLOAT, glb::FALSE, stride, offset as *const _);
        }

        check_errors!();

        try!(DEFAULT_VERTEXARRAY.bind());

        Ok(ScreenQuad {
            vao: vao,
            buffer: buffer
        })
    }

    pub fn draw(&mut self) -> GLResult<()> {
        unsafe {
            glb::Disable(glb::DEPTH_TEST);
            glb::Disable(glb::STENCIL_TEST);
            glb::Disable(glb::CULL_FACE);
        }

        check_errors!();

        try!(self.vao.bind());

        check_errors!();

        unsafe {
            glb::DrawArrays(glb::TRIANGLE_STRIP, 0, 4);
        }

        check_errors!();

        Ok(())
    }
}