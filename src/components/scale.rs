//! Size scaling component

use nalgebra::Vector3;

use specs;

#[derive(Clone, Debug)]
pub struct Component(pub Vector3<f32>);

impl specs::Component for Component {
    type Storage = specs::VecStorage<Component>;
}

impl Component {
    #[inline(always)]
    pub fn new(x: f32, y: f32, z: f32) -> Component {
        Component(Vector3::new(x, y, z))
    }

    /// Create a scale transform which scales all axes equally
    #[inline(always)]
    pub fn uniform(scale: f32) -> Component {
        Component(Vector3::new(scale, scale, scale))
    }

    #[inline(always)]
    pub fn one() -> Component {
        Component::new(1.0, 1.0, 1.0)
    }
}