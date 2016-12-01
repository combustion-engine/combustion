//! Rotation component using Quaternions

use specs;

use nalgebra::{Quaternion, Unit, UnitQuaternion};
use num_traits::{One};

pub struct Component(pub Quaternion<f32>);

impl specs::Component for Component {
    type Storage = specs::VecStorage<Component>;
}

impl Component {
    #[inline(always)]
    pub fn new() -> Component {
        Component(Quaternion::one())
    }

    pub fn unit(&self) -> UnitQuaternion<f32> {
        UnitQuaternion::new(&self.0)
    }
}

impl From<Quaternion<f32>> for Component {
    #[inline(always)]
    fn from(quat: Quaternion<f32>) -> Component {
        Component(quat)
    }
}