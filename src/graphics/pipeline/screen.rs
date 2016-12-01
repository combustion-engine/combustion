use std::mem;
use std::ptr;

use ::backend::gl::*;
use ::backend::gl::types::*;
use ::backend::gl::bindings as glb;

use super::shaders::{SCREEN_VERTEX_SHADER_SRC, SCREEN_FRAGMENT_SHADER_SRC};

static QUAD_DATA: [f32; 20] = [
    -1.0, 1.0, 0.0, 0.0, 1.0,
    -1.0, -1.0, 0.0, 0.0, 0.0,
    1.0, 1.0, 0.0, 1.0, 1.0,
    1.0, -1.0, 0.0, 1.0, 0.0,
];

pub struct ScreenQuad {
    vao: GLVertexArray,
    buffer: GLBuffer,
    shader: GLShaderProgram
}

impl ScreenQuad {
    pub fn new() -> GLResult<ScreenQuad> {
        let screen_vertex_shader = try!(GLShader::from_source(SCREEN_VERTEX_SHADER_SRC.to_string(),
                                                              GLShaderVariant::VertexShader));

        let screen_fragment_shader = try!(GLShader::from_source(SCREEN_FRAGMENT_SHADER_SRC.to_string(),
                                                                GLShaderVariant::FragmentShader));

        let screen_shader = GLShaderProgramBuilder::new()?
            .attach_shader(screen_vertex_shader)?
            .attach_shader(screen_fragment_shader)?
            .link()?
            .finish();

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
            buffer: buffer,
            shader: screen_shader
        })
    }

    pub fn draw(&mut self, texture: &GLTexture) -> GLResult<()> {
        try!(DEFAULT_FRAMEBUFFER.bind());

        unsafe {
            glb::Clear(glb::COLOR_BUFFER_BIT);

            glb::Disable(glb::DEPTH_TEST);
            glb::Disable(glb::STENCIL_TEST);
        }

        check_errors!();

        try!(self.vao.bind());

        try!(self.shader.use_program());

        unsafe {
            glb::ActiveTexture(glb::TEXTURE0);
            try!(texture.bind());
        }

        check_errors!();

        unsafe {
            glb::DrawArrays(glb::TRIANGLE_STRIP, 0, 4);
        }

        check_errors!();

        Ok(())
    }
}