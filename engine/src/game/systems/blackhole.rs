//! Blackhole event handling system

use specs;

use ::resources::event_queue::{Event, Resource as EventQueue};

pub struct System;

impl specs::System<super::Delta> for System {
    fn run(&mut self, arg: specs::RunArg, _: super::Delta) {
        let mut event_queue = arg.fetch(|world| world.write_resource::<EventQueue>());

        event_queue.clear();
    }
}