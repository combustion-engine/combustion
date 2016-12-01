//! Ambient lighting component

use specs;
use ::backend::gl::GLColor;

#[derive(Clone, Debug)]
pub struct Component {
    pub color: GLColor
}

impl specs::Component for Component {
    type Storage = specs::VecStorage<Component>;
}
