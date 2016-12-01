//! Turntable transform system

use specs;
use specs::Join;

use nalgebra::{Rotation3, Vector3, Rotation};

pub struct System;

impl specs::System<super::Delta> for System {
    fn run(&mut self, arg: specs::RunArg, delta: super::Delta) {
        use ::components::rotation::Component as Rotation;
        use ::game::components::turntable::Component as Turntable;

        let (ref turntables, ref mut rotations) = arg.fetch(|world| {
            (
                world.read::<Turntable>(),
                world.write::<Rotation>(),
            )
        });

        for (turntable, mut rotation) in (turntables, rotations).iter() {
            rotation.0.append_rotation_mut(&Vector3::new(0.0, turntable.rate * delta, 0.0));
        }
    }
}