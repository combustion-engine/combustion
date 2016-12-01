//! Look-At constraint
//!
//! Forces an entity to face towards a point or other entity.

use specs;
use nalgebra::{Point3, Vector3};

/// The `LookAtTarget` specifies what the entity should look at
#[derive(Copy, Clone, Debug)]
pub enum LookAtTarget {
    Position(Point3<f32>),
    Entity(specs::Entity)
}

#[derive(Clone, Debug)]
pub struct Component {
    pub target: LookAtTarget,
    pub up: Vector3<f32>,
    pub lh: bool
}

impl specs::Component for Component {
    type Storage = specs::VecStorage<Component>;
}

impl Component {
    #[inline(always)]
    pub fn new(target: LookAtTarget) -> Component {
        Component {
            target: target,
            up: Vector3::new(0.0, 1.0, 0.0),
            lh: false
        }
    }
}