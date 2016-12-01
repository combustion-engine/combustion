//! Renderable component
//!
//! This component is requiered for any entity which should appear on screen

use specs;

use ::backend::gl::gl_error::*;

#[derive(Clone, Debug)]
pub struct Component {
    /// Specifies if the renderable should have its GPU data updated
    pub dirty: bool,
}

impl specs::Component for Component {
    type Storage = specs::VecStorage<Component>;
}

impl Component {
    #[inline(always)]
    pub fn new() -> Component {
        Component { dirty: true }
    }
}