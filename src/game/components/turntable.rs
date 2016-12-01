//! Turntable component
//!
//! This component rotates an entity on the Y-axis at the given rate

use specs;

#[derive(Default)]
pub struct Component {
    /// Rate of rotation in radians per second
    pub rate: f32
}

impl specs::Component for Component {
    type Storage = specs::VecStorage<Component>;
}