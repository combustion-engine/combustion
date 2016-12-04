use nalgebra::{Point3, Vector3, Isometry3};

use specs;

use error::*;

use components;
use resources;
use systems;

pub struct Entity(specs::Entity);

impl super::Entity<Entity> for Entity {
    fn new(mut world: &mut specs::World) -> AppResult<Entity> {
        use components::transform::Component as Transform;
        use components::position::Component as Position;
        use components::rotation::Component as Rotation;
        use components::isometry::Component as Isometry;
        use components::camera::Component as Camera;
        use components::constraints::lookat::{Component as LookAtConstraint, LookAtTarget};
        use ::game::components::bob::Component as Bob;

        let camera = world.create_now()
                          .with(Transform::new())
                          .with(Position(Point3::new(0.0, 0.0, 0.0)))
                          .with(Isometry::empty())
                          .with(Bob::new())
                          .with(LookAtConstraint::new(LookAtTarget::Position(Point3::new(0.0, 0.0, 0.0))))
                          .with(Camera::new_perspective(16.0 / 9.0, 70.0f32.to_radians(), 0.1, 1000.0))
                          .build();

        Ok(Entity(camera))
    }

    #[inline(always)]
    fn raw(&self) -> specs::Entity { self.0 }

    #[inline(always)]
    fn from_raw(entity: specs::Entity) -> Entity { Entity(entity) }
}