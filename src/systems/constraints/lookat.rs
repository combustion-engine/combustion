use specs;
use specs::Join;

use nalgebra::*;

use ::components;

pub fn solve(arg: &specs::RunArg) {
    use components::position::Component as Position;
    use components::isometry::Component as Isometry;

    use components::constraints::lookat::Component as LookAtConstraint;
    use components::constraints::lookat::LookAtTarget;

    // Gets the components from the world
    let (ref positions, ref constraints, ref mut isometries) = arg.fetch(|world| {
        (
            world.read::<Position>(),
            world.read::<LookAtConstraint>(),
            world.write::<Isometry>(),
        )
    });

    for (constraint, position, mut isometry) in (constraints, positions, isometries).iter() {
        let target = match constraint.target {
            LookAtTarget::Entity(other_entity) => {
                if let Some(entity_position) = positions.get(other_entity) {
                    entity_position.0
                } else {
                    Point3::new(0.0, 0.0, 0.0)
                }
            }
            LookAtTarget::Position(target) => {
                target
            }
        };

        isometry.0 = Isometry3::look_at_rh(&position.0, &target, &constraint.up);
    }
}