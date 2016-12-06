//! 3D position component

use specs;
use nalgebra::{Point3};

#[derive(Clone, Debug)]
pub struct Component(pub Point3<f32>);

impl specs::Component for Component {
    type Storage = specs::VecStorage<Component>;
}

impl Component {
    #[inline(always)]
    pub fn new(x: f32, y: f32, z: f32) -> Component {
        Component(Point3::new(x, y, z))
    }
}

impl From<Point3<f32>> for Component {
    #[inline(always)]
    fn from(point: Point3<f32>) -> Component {
        Component(point)
    }
}
