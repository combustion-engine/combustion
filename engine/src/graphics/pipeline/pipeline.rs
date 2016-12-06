use nalgebra::Vector2;

use ::backend::gl::*;
use ::backend::gl::types::*;
use ::backend::gl::bindings as glb;

use super::gbuffer::Gbuffer;
use super::stage::Stage;
use super::screen::ScreenQuad;

pub const GEOMETRY_STAGE_COMPONENTS: [(GLenum, GLenum); 3] = [
    (glb::RGBA, glb::RGBA16F),
    (glb::RGBA, glb::RGBA32F),
    (glb::RGB, glb::RGB32F),
];

pub const LIGHTING_STAGE_NAMES: [&'static str; 3] = [
    "ColorSs",
    "NormalMs",
    "Positions"
];

pub const LIGHTING_STAGE_COMPONENTS: [(GLenum, GLenum); 1] = [
    (glb::RGBA, glb::RGBA16F)
];

pub const SCREEN_SHADER_NAMES: [&'static str; 1] = [
    "screen"
];

pub struct Pipeline {
    geometry_stage: Stage,
    lighting_stage: Stage,
    final_stage: Stage,

    geometry_shader: GLShaderProgram,
    screen_shader: GLShaderProgram,

    screen: ScreenQuad,
    resolution: Vector2<f32>
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

        let screen_vertex_shader = try!(GLShader::from_file("shaders/screen.vert", GLShaderVariant::VertexShader));
        let screen_fragment_shader = try!(GLShader::from_file("shaders/screen.frag", GLShaderVariant::FragmentShader));

        let screen_shader = GLShaderProgramBuilder::new()?
            .attach_shader(screen_vertex_shader)?
            .attach_shader(screen_fragment_shader)?
            .link()?
            .finish();

        let geometry_stage = try!(Stage::new(width, height, Some(&GEOMETRY_STAGE_COMPONENTS)));
        let mut lighting_stage = try!(Stage::new(width, height, Some(&LIGHTING_STAGE_COMPONENTS)));
        //TODO: Add transparency stage
        let final_stage = try!(Stage::new(width, height, None));

        //TODO: Move this to whatever stage is right before the screen stage
        try!(lighting_stage.set_filter(GLTextureFilter::Linear));

        Ok(Pipeline {
            geometry_stage: geometry_stage,
            lighting_stage: lighting_stage,
            final_stage: final_stage,
            geometry_shader: geometry_shader,
            screen_shader: screen_shader,
            screen: try!(ScreenQuad::new()),
            resolution: Vector2::new(width as f32, height as f32)
        })
    }

    /// The Geometry pass is where all world objects are rendered to the G-Buffer.
    ///
    /// This pass gives some amount of control to the renderer, allowing it to bind shader uniforms and so forth.
    pub fn geometry_pass<F>(&mut self, mut f: F) -> GLResult<()> where F: FnMut(&GLShaderProgram) -> GLResult<()> {
        // When the geometry pass is called it invalidates any lighting stage results, so bind it really quick and clear it
        try!(self.lighting_stage.bind());

        unsafe {
            glb::ClearColor(0.0, 0.0, 0.0, 0.0);
            glb::Clear(glb::COLOR_BUFFER_BIT);
        }

        check_errors!();

        try!(self.geometry_stage.bind());

        unsafe {
            glb::ClearColor(0.0, 0.0, 0.0, 0.0);
            glb::Clear(glb::COLOR_BUFFER_BIT | glb::DEPTH_BUFFER_BIT | glb::STENCIL_BUFFER_BIT);

            //glb::Enable(glb::STENCIL_TEST);

            glb::Enable(glb::DEPTH_TEST);
            glb::DepthFunc(glb::LESS);

            glb::Enable(glb::CULL_FACE);
            glb::CullFace(glb::BACK);

            //Geometry pass cannot use blending at all
            glb::Disable(glb::BLEND);
        }

        check_errors!();

        try!(self.geometry_shader.use_program());

        try!(f(&self.geometry_shader));

        check_errors!();

        Ok(())
    }

    /// The Lighting pass applies custom shaders to the G-Buffer data to light the scene as desired.
    ///
    /// This pass gives almost no control to the renderer except a few uniforms and which shader to use in the first place.
    ///
    /// However, this pass can be repeated multiple times for varying lighting shaders
    pub fn lighting_pass<F>(&mut self, shader: &GLShaderProgram, mut f: F) -> GLResult<()> where F: FnMut(&GLShaderProgram) -> GLResult<()> {
        try!(self.lighting_stage.bind());

        //The lighting pass can't use anything other than the Gbuffer, basically
        unsafe {
            glb::Disable(glb::DEPTH_TEST);
            glb::Disable(glb::CULL_FACE);
            glb::Disable(glb::BLEND);
        }

        check_errors!();

        try!(shader.use_program());

        try!(self.geometry_stage.gbuffer().unwrap().bind_textures(&shader, &LIGHTING_STAGE_NAMES));

        try!(f(&shader));

        try!(self.screen.draw());

        Ok(())
    }

    /// The Forward pass is traditional forward rendering, which is required for transparent objects or more complex shaders
    /// that simple can't rely on the Gbuffer.
    ///
    /// This stage accumulates it's results into the same framebuffer as the lighting stage, so blending of transparent objects
    /// is done automatically.
    pub fn forward_pass<F>(&mut self, mut f: F) -> GLResult<()> where F: FnMut() -> GLResult<()> {
        try!(self.lighting_stage.bind());

        unsafe {
            glb::Enable(glb::DEPTH_TEST);
            glb::DepthFunc(glb::LESS);

            glb::Clear(glb::DEPTH_BUFFER_BIT);

            glb::Enable(glb::CULL_FACE);
            glb::CullFace(glb::BACK);

            glb::Enable(glb::BLEND);
            glb::BlendFunc(glb::SRC_ALPHA, glb::ONE_MINUS_SRC_ALPHA);
        }

        check_errors!();

        try!(f());

        Ok(())
    }

    /// The Screen pass renders the final result to a quad on the default framebuffer,
    /// effectively drawing it on the the screen.
    ///
    /// This stage also applies FXAA, smoothing out aliasing artifacts
    pub fn final_pass(&mut self) -> GLResult<()> {
        try!(self.final_stage.bind());

        unsafe {
            //No depth or culling for a single quad
            glb::Disable(glb::DEPTH_TEST);
            glb::Disable(glb::CULL_FACE);

            //FXAA may take advantage of blending a bit
            glb::Enable(glb::BLEND);
            glb::BlendFunc(glb::ONE, glb::ONE);
        }

        check_errors!();

        try!(self.screen_shader.use_program());

        let mut res_uniform = try!(self.screen_shader.get_uniform("resolution"));

        try!(res_uniform.vec2f(&self.resolution));

        try!(self.lighting_stage.gbuffer().unwrap().bind_textures(&self.screen_shader, &SCREEN_SHADER_NAMES));

        try!(self.screen.draw());

        Ok(())
    }

    pub fn resize(&mut self, width: usize, height: usize) -> GLResult<()> {
        try!(self.geometry_stage.resize(width, height));
        try!(self.lighting_stage.resize(width, height));
        try!(self.final_stage.resize(width, height));

        self.resolution = Vector2::new(width as f32, height as f32);

        Ok(())
    }
}


