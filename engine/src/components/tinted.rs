//! Object tint component

use specs;
use ::backend::gl::GLColor;

#[derive(Clone, Debug)]
pub struct Component {
    /// Color to be layered over the object to tint it
    pub color: GLColor
}

impl specs::Component for Component {
    type Storage = specs::VecStorage<Component>;
}
