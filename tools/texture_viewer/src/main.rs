#![feature(receiver_try_iter)]

extern crate glfw;
extern crate nice_glfw;
extern crate clap;
extern crate image;
extern crate capnp;
extern crate capnpc;

#[macro_use]
extern crate combustion_common as common;
#[macro_use]
extern crate combustion_backend as backend;
extern crate combustion_protocols;

use common::error::*;

use backend::gl::bindings as glb;

use std::sync::mpsc;
use std::path::{Path, PathBuf};
use std::thread::Builder;

use clap::{App, Arg};
use glfw::{Action, Key, WindowHint, WindowEvent};

pub mod render;
pub mod screen;

use render::RenderSignal;

fn main() {
    let matches: clap::ArgMatches = App::new("texture_viewer")
        .version("0.1.0")
        .author("Aaron Trent <novacrazy@gmail.com>")
        .about("Allows Combustion textures to be viewed easily")
        .arg(Arg::with_name("file").takes_value(true).help("Texture to open on start").validator(|ref path| {
            if Path::new(path).exists() { Ok(()) } else {
                Err("File must exist".to_string())
            }
        }))
        .get_matches();

    run(matches.value_of("file"));
}

fn run<P: AsRef<Path>>(path: Option<P>) {
    common::log::init_global_logger("logs").expect("Could not initialize logging system!");

    let mut glfw: glfw::Glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect_logged("Could not initialize GLFW!");

    let (mut window, events): (glfw::Window, _) = nice_glfw::WindowBuilder::new(&mut glfw)
        .try_modern_context_hints()
        .size(800, 600)
        .common_hints(&[
            WindowHint::Visible(true),
            WindowHint::OpenGlDebugContext(true),
            WindowHint::DoubleBuffer(true),
        ])
        .title("Combustion Texture Viewer")
        .create()
        .expect_logged("Couldn't create window");

    window.set_all_polling(true);

    info!("Window created");

    //Load up all the OpenGL functions from the process
    backend::gl::bindings::load_all_with(|symbol| window.get_proc_address(symbol) as *const _);

    //Enable debugging of OpenGL messages
    backend::gl::enable_debug(backend::gl::default_debug_callback, true).unwrap();

    //Create channel for forwarding events to the render thread
    let (tx, rx) = mpsc::channel();

    // Disconnect current context
    glfw::make_context_current(None);

    //Create Send-able context to send to render thread
    let context = window.render_context();

    let render_thread = Builder::new().name("Render thread".to_string()).spawn(move || {
        info!("Render thread started...");

        //Make the OpenGL context active on the render thread
        glfw::make_context_current(Some(&context));

        render::start(context, rx).expect_logged("Render thread crashed");

        //Once rendering has ended, free the OpenGL context
        glfw::make_context_current(None);
    }).expect_logged("Could not start render thread");

    //If there was a path given at the command line, load it up first
    if let Some(path) = path {
        tx.send(RenderSignal::ChangeTexture(path.as_ref().to_path_buf())).unwrap();
    }

    macro_rules! send_and_unpark {
        ($event:expr) => ({
            let ret = tx.send($event);
            render_thread.thread().unpark();
            ret
        })
    }

    info!("Listening for events...");

    while !window.should_close() {
        glfw.wait_events();

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                }
                WindowEvent::FileDrop(paths) => {
                    if let Some(last) = paths.last() {
                        if last.extension().is_some() {
                            send_and_unpark!(RenderSignal::ChangeTexture(last.clone())).unwrap();
                        } else {
                            error!("Invalid path");
                        }
                    }
                }
                WindowEvent::Refresh => {
                    send_and_unpark!(RenderSignal::Refresh).unwrap();
                }
                WindowEvent::FramebufferSize(width, height) |
                WindowEvent::Size(width, height) if width > 0 && height > 0 => {
                    send_and_unpark!(RenderSignal::Resize(width, height)).unwrap();
                }
                _ => {}
            }
        }
    }

    info!("Shutting down...");

    //Signal the render thread to close
    send_and_unpark!(RenderSignal::Stop).expect_logged("Failed to signal render task.");

    render_thread.join().expect_logged("Failed to join render thread");

    info!("Goodbye");
}

