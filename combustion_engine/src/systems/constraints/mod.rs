use specs;

pub mod lookat;

pub struct System;

impl specs::System<super::Delta> for System {
    fn run(&mut self, arg: specs::RunArg, _: super::Delta) {
        lookat::solve(&arg);
    }
}