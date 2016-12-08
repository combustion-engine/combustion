extern crate glfw;
extern crate nice_glfw;
extern crate image;
extern crate clap;

#[macro_use]
extern crate combustion_common as common;

#[macro_use]
extern crate combustion_backend as backend;

use common::error::*;

use backend::gl::*;
use backend::gl::bindings as glb;

use std::path::Path;

use clap::{App, Arg};
use glfw::WindowHint;

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

fn compress_texture<P: AsRef<Path>>(path: P, dir: &Path) -> GLResult<()> {
    let stem = path.as_ref().file_stem().unwrap();



    Ok(())
}

fn main() {
    let matches: clap::ArgMatches = App::new("texture_compressor")
        .version("0.1.0")
        .author("Aaron Trent <novacrazy@gmail.com>")
        .about("Converts image files into compressed textures")
        .arg(Arg::with_name("files").multiple(true).required(true).help("Images to compress").validator(|ref path| -> Result<(), String> {
            if Path::new(path).extension().is_some() {
                Ok(())
            } else {
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
