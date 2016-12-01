use specs;
use specs::Join;

pub struct System;

impl specs::System<super::Delta> for System {
    fn run(&mut self, arg: specs::RunArg, _: super::Delta) {
        use ::components::effector::Component as EffectorNode;

        let (ref mut effectors, ) = arg.fetch(|world| {
            (
                world.write::<EffectorNode>(),
            )
        });

        for mut effector in effectors.iter() {
            effector.ran = false;
        }
    }
}