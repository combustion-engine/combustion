use specs;

pub struct System;

impl specs::System<super::Delta> for System {
    fn run(&mut self, _: specs::RunArg, _: super::Delta) {}
}