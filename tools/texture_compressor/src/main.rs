extern crate glfw;
extern crate nice_glfw;
extern crate image;

#[macro_use]
extern crate combustion_common as common;

#[macro_use]
extern crate combustion_backend as backend;

use common::error::*;

use glfw::{WindowHint};

fn main() {
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

    //TODO: Load up textures, buffer them to GPU, then retrieve the compressed versions.
}
