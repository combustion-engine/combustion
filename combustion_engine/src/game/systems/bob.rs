use std::f32::consts::PI;

use specs;
use specs::Join;

use nalgebra::{Rotation3, Vector3, Rotation};

pub struct System;

impl specs::System<super::Delta> for System {
    fn run(&mut self, arg: specs::RunArg, delta: super::Delta) {
        use ::components::position::Component as Position;
        use ::game::components::bob::Component as Bob;

        let (ref mut bobs, ref mut positions) = arg.fetch(|world| {
            (
                world.write::<Bob>(),
                world.write::<Position>(),
            )
        });

        for (mut bob, mut position) in (bobs, positions).iter() {
            bob.value += delta;

            position.0.y = (bob.value / 3.0).sin() / 3.0;

            position.0.x = bob.value.cos();
            position.0.z = bob.value.sin();
        }
    }
}