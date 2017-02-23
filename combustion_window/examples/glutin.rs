#![cfg(feature = "glutin")]

extern crate combustion_window;

use combustion_window::provider::{WindowBuilder, WindowProvider};
use combustion_window::providers::glutin::{glutin, GlutinWindowBuilder};

fn main() {
    let window = GlutinWindowBuilder::new()
        .size(800, 600)
        .with_raw(|raw| {
            raw.with_decorations(true)
               .with_visibility(true)
        }).build().unwrap();


    unsafe { window.make_current(); }

    'events: loop {
        for event in window.poll_events() {
            match event {
                glutin::Event::Closed => {
                    break 'events;
                }
                // process events here
                _ => ()
            }
        }

        // draw everything here

        window.swap_buffers();

        ::std::thread::sleep_ms(100);
    }
}