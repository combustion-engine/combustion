use specs;
use specs::Join;

use nalgebra::{Rotation3, Isometry3, Vector3, Norm};

use ::components;

pub fn solve(arg: &specs::RunArg) {
    use components::position::Component as Position;
    use components::rotation::Component as Rotation;

    use components::constraints::lookat::Component as LookAtConstraint;
    use components::constraints::lookat::LookAtTarget;

    // Gets the components from the world
    let (ref positions, ref constraints, ref mut rotations) = arg.fetch(|world| {
        (
            world.read::<Position>(),
            world.read::<LookAtConstraint>(),
            world.write::<Rotation>(),
        )
    });

    for (constraint, position, mut rotation) in (constraints, positions, rotations).iter() {
        let ref dir = match constraint.target {
            LookAtTarget::Entity(other_entity) => {
                if let Some(entity_position) = positions.get(other_entity) {
                    entity_position.0 - position.0
                } else {
                    Vector3::new(0.0, 0.0, 1.0)
                }
            }
            LookAtTarget::Position(target) => {
                target - position.0
            }
        };

        rotation.0 = if constraint.lh {
            Rotation3::look_at_lh(dir, &constraint.up)
        } else {
            Rotation3::look_at_rh(dir, &constraint.up)
        };
    }
}