//! Rotation component

use specs;
use nalgebra::{Rotation3, Vector3};
use num_traits::One;

#[derive(Clone, Debug)]
pub struct Component(pub Rotation3<f32>);

impl specs::Component for Component {
    type Storage = specs::VecStorage<Component>;
}

impl Component {
    /// Create identity rotation which does nothing
    #[inline(always)]
    pub fn none() -> Component {
        Component(Rotation3::one())
    }

    /// Create rotation from axisangle rotation vector
    #[inline(always)]
    pub fn new(axisangle: Vector3<f32>) -> Component {
        Component(Rotation3::new(axisangle))
    }
}

impl From<Rotation3<f32>> for Component {
    #[inline(always)]
    fn from(rotation: Rotation3<f32>) -> Component {
        Component(rotation)
    }
}