use specs;
use nalgebra::Isometry3;

#[derive(Clone, Debug)]
pub struct Component {
    pub thrust: Isometry3<f32>
}

impl specs::Component for Component {
    type Storage = specs::VecStorage<Component>;
}
