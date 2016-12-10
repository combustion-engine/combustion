extern crate glfw;
extern crate nice_glfw;
extern crate image;
extern crate clap;
extern crate capnp;
extern crate capnpc;

#[macro_use]
extern crate combustion_common as common;
#[macro_use]
extern crate combustion_backend as backend;
extern crate combustion_protocols;

use combustion_protocols::protocols;

use common::error::*;

use backend::gl::*;
use backend::gl::bindings as glb;

use std::path::{Path, PathBuf};
use std::fs::File;

use clap::{App, Arg};
use glfw::WindowHint;
use image::GenericImage;

fn init() {
    common::log::init_global_logger("logs").expect("Could not initialize logging system!");

    let mut glfw: glfw::Glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect_logged("Could not initialize GLFW!");

    let (mut window, _) = nice_glfw::WindowBuilder::new(&mut glfw)
        .try_modern_context_hints()
        .size(1, 1)
        .common_hints(&[
            WindowHint::Visible(false),
            WindowHint::OpenGlDebugContext(true)
        ])
        .title("texture_compressor")
        .create()
        .expect_logged("Couldn't create window");

    info!("Window created");

    //Load up all the OpenGL functions from the process
    backend::gl::bindings::load_all_with(|symbol| window.get_proc_address(symbol) as *const _);

    //Enable debugging of OpenGL messages
    backend::gl::enable_debug(backend::gl::default_debug_callback, true).unwrap();
}

fn compress_texture<P: AsRef<Path> + Clone>(path: P, dir: &Path) -> GLResult<()> {
    use image::DynamicImage;
    use protocols::texture;
    use protocols::texture::protocol::{Format as TextureFormat, Compression};
    use protocols::texture::protocol::texture as texture_protocol;

    let image: DynamicImage = try!(image::open(path.clone()));

    let dimensions = image.dimensions();

    //Get the format and raw pixel data
    let (format, bytes) = match image {
        DynamicImage::ImageRgb8(i) => (TextureFormat::Rgb, i.into_raw()),
        DynamicImage::ImageRgba8(i) => (TextureFormat::Rgba, i.into_raw()),
        DynamicImage::ImageLuma8(i) => (TextureFormat::Luma, i.into_raw()),
        DynamicImage::ImageLumaA8(i) => (TextureFormat::LumaAlpha, i.into_raw())
    };

    let mut texture_message = capnp::message::Builder::new_default();

    {
        let mut texture_builder = texture_message.init_root::<texture_protocol::Builder>();

        texture_builder.set_width(dimensions.0);
        texture_builder.set_height(dimensions.1);
        texture_builder.set_format(format);
        texture_builder.set_compression(Compression::None);
        texture_builder.set_data(&bytes);
    }

    let stem = path.as_ref().file_stem().unwrap();

    let mut out_path = PathBuf::from(dir).join(stem);

    out_path.set_extension(texture::EXTENSION);

    let mut out = try!(File::create(out_path.as_path()));

    try!(capnp::serialize_packed::write_message(&mut out, &texture_message));

    Ok(())
}

fn main() {
    let matches: clap::ArgMatches = App::new("texture_compressor")
        .version("0.1.0")
        .author("Aaron Trent <novacrazy@gmail.com>")
        .about("Converts image files into compressed textures")
        .arg(Arg::with_name("files").multiple(true).required(true).help("Images to compress").validator(|ref path| -> Result<(), String> {
            if Path::new(path).extension().is_some() { Ok(()) } else {
                Err("the images must have file extensions".to_string())
            }
        }))
        .arg(Arg::with_name("out_dir").short("o").help("Output directory").takes_value(true))
        .get_matches();

    if let Some(files) = matches.values_of("files") {
        init();

        let out_dir = matches.value_of("out_dir").map(|d| Path::new(d));

        for file in files {
            let dir = if let Some(out_dir) = out_dir { out_dir } else {
                Path::new(file).parent().unwrap_or(Path::new("."))
            };

            compress_texture(file, dir).expect_logged("Could not process file");
        }
    }
}
