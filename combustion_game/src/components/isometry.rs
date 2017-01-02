//! Isometry transform component

use specs;
use nalgebra::Isometry3;
use num_traits::One;

use super::effector::Effector;

#[derive(Clone, Debug)]
pub struct Component(pub Isometry3<f32>);

impl specs::Component for Component {
    type Storage = specs::VecStorage<Component>;
}

impl Component {
    #[inline(always)]
    pub fn new(isometry: Isometry3<f32>) -> Component {
        Component(isometry)
    }

    /// Creates an identity isometry that does nothing
    #[inline(always)]
    pub fn empty() -> Component {
        Component(Isometry3::one())
    }
}