extern crate glfw;
extern crate nice_glfw;
extern crate clap;

#[macro_use]
extern crate combustion_common as common;
#[macro_use]
extern crate combustion_backend as backend;
extern crate combustion_protocols as protocols;

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
        .size(600, 800)
        .common_hints(&[
            WindowHint::Visible(true),
            WindowHint::OpenGlDebugContext(true)
        ])
        .title("Combustion Texture Viewer")
        .create()
        .expect_logged("Couldn't create window");

    info!("Window created");

    //Load up all the OpenGL functions from the process
    backend::gl::bindings::load_all_with(|symbol| window.get_proc_address(symbol) as *const _);

    //Enable debugging of OpenGL messages
    backend::gl::enable_debug(backend::gl::default_debug_callback, true).unwrap();
}

fn main() {
    let matches: clap::ArgMatches = App::new("texture_viewer")
        .version("0.1.0")
        .author("Aaron Trent <novacrazy@gmail.com>")
        .about("Allows Combustion textures to be viewed easily")
        .get_matches();

    init();
}
