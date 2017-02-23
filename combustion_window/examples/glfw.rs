#![cfg(feature = "glfw")]

extern crate combustion_window;

use combustion_window::provider::WindowBuilder;
use combustion_window::providers::glfw::{glfw, GlfwWindowBuilder, GlfwWindowProvider};

fn main() {
    let mut provider: GlfwWindowProvider = GlfwWindowBuilder::new()
        .size(800, 600)
        .visible(true)
        .try_modern_context_hints()
        .build().unwrap();

    while !provider.window.should_close() {
        provider.glfw.wait_events();

        for (_, event) in glfw::flush_messages(&*provider.event_receiver) {
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    provider.window.set_should_close(true);
                }
                _ => ()
            }
        }
    }
}