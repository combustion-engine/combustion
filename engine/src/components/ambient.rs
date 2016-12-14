//! Ambient lighting component

use specs;
use ::backend::generic::color::Color;

#[derive(Clone, Debug)]
pub struct Component {
    pub color: Color
}

impl specs::Component for Component {
    type Storage = specs::VecStorage<Component>;
}
