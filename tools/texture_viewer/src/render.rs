use std::sync::mpsc;
use std::path::PathBuf;
use std::fs::File;
use std::io::BufReader;

use glfw::{self, Context};
use image::{self, DynamicImage, GenericImage};
use capnp;

use common::error::*;
use common::utils::*;

use backend::gl::*;
use backend::gl::types::*;
use backend::gl::bindings as glb;

use combustion_protocols as protocols;

use self::protocols::texture;
use self::protocols::texture::protocol::{Kind};
use self::protocols::texture::protocol::texture as texture_protocol;
use self::protocols::texture::gl::*;

use screen::ScreenQuad;

pub enum RenderSignal {
    Stop,
    Refresh,
    Resize(i32, i32),
    ChangeTexture(PathBuf),
    Zoom(f64),
    Move(f64, f64)
}

#[cfg(debug_assertions)]
fn load_screen_shader() -> GLResult<GLShaderProgram> {
    let screen_vertex_shader = try!(GLShader::from_file("shaders/screen.vert", GLShaderVariant::VertexShader));
    let screen_fragment_shader = try!(GLShader::from_file("shaders/screen.frag", GLShaderVariant::FragmentShader));

    let screen_shader = GLShaderProgramBuilder::new()?
        .attach_shader(screen_vertex_shader)?
        .attach_shader(screen_fragment_shader)?
        .link()?
        .finish();

    Ok(screen_shader)
}

#[cfg(not(debug_assertions))]
fn load_screen_shader() -> GLResult<GLShaderProgram> {
    const SCREEN_VERTEX_SHADER_SRC: &'static str = include_str!("../shaders/screen.vert");
    const SCREEN_FRAGMENT_SHADER_SRC: &'static str = include_str!("../shaders/screen.frag");

    let screen_vertex_shader = try!(GLShader::from_source(SCREEN_VERTEX_SHADER_SRC.to_string(), GLShaderVariant::VertexShader));
    let screen_fragment_shader = try!(GLShader::from_source(SCREEN_FRAGMENT_SHADER_SRC.to_string(), GLShaderVariant::FragmentShader));

    let screen_shader = GLShaderProgramBuilder::new()?
        .attach_shader(screen_vertex_shader)?
        .attach_shader(screen_fragment_shader)?
        .link()?
        .finish();

    Ok(screen_shader)
}

pub fn start(mut context: glfw::RenderContext, rx: mpsc::Receiver<RenderSignal>) -> GLResult<()> {
    let mut active_texture = try!(GLTexture::new(GLTextureKind::Texture2D));

    try!(active_texture.set_filter(GLTextureFilter::Nearest, Some(GLTextureFilter::Nearest)));
    try!(active_texture.set_wrap(GLTextureWrap::ClampToBorder));

    let max_anisotropy = active_texture.get_max_anisotropy().expect_logged("Couldn't get max anisotropy value");
    active_texture.set_anisotropy(max_anisotropy).expect_logged("Couldn't set max anisotropy");

    let screen_shader = try!(load_screen_shader());

    let mut screen = try!(ScreenQuad::new());

    let mut resolution: (u32, u32) = (800, 600);
    let mut texture_resolution: (u32, u32) = (0, 0);
    let mut zoom: f64 = 1.0;
    let mut pos: (f64, f64) = (0.0, 0.0);

    'render: loop {
        let mut viewport_size = None;

        for event in rx.try_iter() {
            match event {
                RenderSignal::Stop => {
                    break 'render;
                }
                RenderSignal::Refresh => {}
                RenderSignal::Resize(width, height) => {
                    viewport_size = Some((width, height));
                }
                RenderSignal::Zoom(value) => {
                    zoom = clamp(zoom - value, 0.1, 100.0);
                }
                RenderSignal::Move(x, y) => {
                    pos.0 += x;
                    pos.1 += y;
                }
                RenderSignal::ChangeTexture(path) => {
                    try!(active_texture.bind());

                    if path.extension().unwrap() == protocols::texture::EXTENSION {
                        info!("Loading Combustion texture...");

                        let mut source = BufReader::new(File::open(path)?);

                        let texture_message = capnp::serialize_packed::read_message(&mut source, capnp::message::ReaderOptions {
                            traversal_limit_in_words: u64::max_value(), nesting_limit: 64
                        }).expect_logged("Could not open Texture protocol");

                        let texture = texture_message.get_root::<texture_protocol::Reader>()
                                                     .expect_logged("No texture protocol root found");

                        let width = texture.get_width();
                        let height = texture.get_height();

                        //TODO: Support more kinds
                        //let depth = texture.get_depth();

                        let kind = texture.get_kind()
                                          .expect_logged("Couldn't find Kind value. This could be caused by using an older texture format.");

                        //TODO: Support more kinds
                        assert!(kind == Kind::Texture2D);

                        let specific_format = texture::SpecificFormat::read_texture(&texture)
                            .expect_logged("Error retrieving texture information");

                        let data = texture.get_data()
                                          .expect_logged("No texture data found");

                        info!("Combustion texture loaded.");

                        let generic_format = specific_format.to_generic();

                        info!("Buffering Combustion texture...");

                        if specific_format.is_compressed() {
                            unsafe {
                                glb::CompressedTexImage2D(glb::TEXTURE_2D, 0, specific_format.specific(),
                                                          width as GLsizei, height as GLsizei,
                                                          0, data.len() as GLsizei, data.as_ptr() as *const _);
                            }

                        } else {
                            unsafe {
                                glb::TexImage2D(glb::TEXTURE_2D, 0, specific_format.specific() as GLint,
                                                width as GLsizei, height as GLsizei, 0,
                                                generic_format.generic(), glb::UNSIGNED_BYTE, data.as_ptr() as *const _);
                            }
                        }

                        check_errors!();

                        texture_resolution = (width, height);
                    } else {
                        info!("Loading normal image...");

                        let image = try!(image::open(path));

                        let (width, height) = image.dimensions();

                        let (format, iformat, data) = match image {
                            DynamicImage::ImageLuma8(i) => (glb::RED, glb::R8, i.into_vec()),
                            DynamicImage::ImageLumaA8(i) => (glb::RG, glb::RG8, i.into_vec()),
                            DynamicImage::ImageRgb8(i) => (glb::RGB, glb::RGB8, i.into_vec()),
                            DynamicImage::ImageRgba8(i) => (glb::RGBA, glb::RGBA8, i.into_vec())
                        };

                        info!("Buffering normal image...");

                        unsafe {
                            glb::TexImage2D(glb::TEXTURE_2D, 0, iformat as GLint,
                                            width as GLsizei, height as GLsizei, 0,
                                            format, glb::UNSIGNED_BYTE, data.as_ptr() as *const _);
                        }

                        check_errors!();

                        texture_resolution = (width, height);
                    }

                    info!("Generating mipmaps...");
                    try!(active_texture.generate_mipmap());

                    zoom = 1.0;
                    pos = (0.0, 0.0);

                    info!("Done!");
                }
            }
        }

        if let Some((width, height)) = viewport_size {
            unsafe { glb::Viewport(0, 0, width as GLsizei, height as GLsizei); }

            resolution = (width as u32, height as u32);

            check_errors!();

            info!("Viewport resized to {}x{}", width, height);
        }

        try!(screen_shader.use_program());

        let mut res_uniform = try!(screen_shader.get_uniform("resolution"));
        let mut tex_res_uniform = try!(screen_shader.get_uniform("texture_resolution"));
        let mut zoom_uniform = try!(screen_shader.get_uniform("zoom"));
        let mut pos_uniform = try!(screen_shader.get_uniform("pos"));

        try!(res_uniform.float2(resolution.0 as f32, resolution.1 as f32));
        try!(tex_res_uniform.float2(texture_resolution.0 as f32, texture_resolution.1 as f32));
        try!(zoom_uniform.float1(zoom as f32));
        try!(pos_uniform.float2(pos.0 as f32, pos.1 as f32));

        try!(screen.draw());

        context.swap_buffers();

        ::std::thread::park();
    }

    Ok(())
}