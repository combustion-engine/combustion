use ::backend::gl::*;
use ::backend::gl::types::*;
use ::backend::gl::bindings as glb;

use super::gbuffer::Gbuffer;
use super::stage::Stage;
use super::screen::ScreenQuad;

use super::shaders::{SCREEN_VERTEX_SHADER_SRC, SCREEN_FRAGMENT_SHADER_SRC, SCREEN_SHADER_NAMES};

pub const GEOMETRY_STAGE_COMPONENTS: [(GLenum, GLenum); 3] = [
    (glb::RGB, glb::RGB16F),
    (glb::RGB, glb::RGB16F),
    (glb::RGB, glb::RGB16F),
];

pub const LIGHTING_STAGE_NAMES: [&'static str; 3] = [
    "colors",
    "normals",
    "RMDs"
];

pub const LIGHTING_STAGE_COMPONENTS: [(GLenum, GLenum); 1] = [
    (glb::RGB, glb::RGB16F)
];

pub struct Pipeline {
    geometry_stage: Stage,
    lighting_stage: Stage,
    final_stage: Stage,

    geometry_shader: GLShaderProgram,
    screen_shader: GLShaderProgram,

    screen: ScreenQuad
}

impl Pipeline {
    pub fn new(width: usize, height: usize) -> GLResult<Pipeline> {
        let geometry_vertex_shader = try!(GLShader::from_file("shaders/deferred_geometry.vert", GLShaderVariant::VertexShader));
        let geometry_fragment_shader = try!(GLShader::from_file("shaders/deferred_geometry.frag", GLShaderVariant::FragmentShader));

        let geometry_shader = GLShaderProgramBuilder::new()?
            .attach_shader(geometry_vertex_shader)?
            .attach_shader(geometry_fragment_shader)?
            .link()?
            .finish();

        let screen_vertex_shader = try!(GLShader::from_source(SCREEN_VERTEX_SHADER_SRC.to_string(),
                                                              GLShaderVariant::VertexShader));

        let screen_fragment_shader = try!(GLShader::from_source(SCREEN_FRAGMENT_SHADER_SRC.to_string(),
                                                                GLShaderVariant::FragmentShader));

        let screen_shader = GLShaderProgramBuilder::new()?
            .attach_shader(screen_vertex_shader)?
            .attach_shader(screen_fragment_shader)?
            .link()?
            .finish();

        let geometry_stage = try!(Stage::new(width, height, Some(&GEOMETRY_STAGE_COMPONENTS)));
        let lighting_stage = try!(Stage::new(width, height, Some(&LIGHTING_STAGE_COMPONENTS)));
        let final_stage = try!(Stage::new(width, height, None));

        Ok(Pipeline {
            geometry_stage: geometry_stage,
            lighting_stage: lighting_stage,
            final_stage: final_stage,
            geometry_shader: geometry_shader,
            screen_shader: screen_shader,
            screen: try!(ScreenQuad::new())
        })
    }

    /// The Geometry pass is where all world objects are rendered to the G-Buffer.
    pub fn geometry_pass<F>(&mut self, mut f: F) -> GLResult<()> where F: FnMut(&GLShaderProgram) -> GLResult<()> {
        try!(self.geometry_stage.bind());

        unsafe {
            glb::ClearColor(0.0, 0.0, 0.0, 0.0);
            glb::Clear(glb::COLOR_BUFFER_BIT | glb::DEPTH_BUFFER_BIT | glb::STENCIL_BUFFER_BIT);

            //glb::Enable(glb::STENCIL_TEST);

            glb::Enable(glb::DEPTH_TEST);
            glb::DepthFunc(glb::LESS);

            glb::Enable(glb::CULL_FACE);
            glb::CullFace(glb::BACK);

            glb::Disable(glb::BLEND);
        }

        check_errors!();

        try!(self.geometry_shader.use_program());

        try!(f(&self.geometry_shader));

        check_errors!();

        Ok(())
    }

    /// The Lighting pass applies custom shaders to the G-Buffer data to light the scene as desired.
    pub fn lighting_pass<F>(&mut self, shader: &GLShaderProgram, mut f: F) -> GLResult<()> where F: FnMut(&GLShaderProgram) -> GLResult<()> {
        try!(self.lighting_stage.bind());

        try!(shader.use_program());

        try!(self.geometry_stage.gbuffer().unwrap().bind_textures(&shader, &LIGHTING_STAGE_NAMES));

        try!(f(&shader));

        try!(self.screen.draw());

        Ok(())
    }

    /// The Screen pass renders the final result to a quad on the default framebuffer,
    /// effectively drawing it on the the screen.
    pub fn final_pass(&mut self) -> GLResult<()> {
        try!(self.final_stage.bind());

        try!(self.screen_shader.use_program());

        try!(self.lighting_stage.gbuffer().unwrap().bind_textures(&self.screen_shader, &SCREEN_SHADER_NAMES));

        try!(self.screen.draw());

        Ok(())
    }

    pub fn resize(&mut self, width: usize, height: usize) -> GLResult<()> {
        try!(self.geometry_stage.resize(width, height));
        try!(self.lighting_stage.resize(width, height));
        try!(self.final_stage.resize(width, height));

        Ok(())
    }
}


