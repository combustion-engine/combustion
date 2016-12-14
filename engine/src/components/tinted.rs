//! Object tint component

use specs;
use ::backend::generic::color::Color;

#[derive(Clone, Debug)]
pub struct Component {
    /// Color to be layered over the object to tint it
    pub color: Color
}

impl specs::Component for Component {
    type Storage = specs::VecStorage<Component>;
}
