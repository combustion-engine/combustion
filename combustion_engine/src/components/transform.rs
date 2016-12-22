//! Transform matrix component
//!
//! This component usually stores the results of position, rotation and scale transformations, but if none of those exist
//! it can be set to a specific matrix.

use nalgebra::{Matrix4, Eye};

use specs;

pub struct Component {
    /// Resulting transformation matrix
    pub matrix: Matrix4<f32>,
    /// Inverse (if it exists) of the transformation matrix
    pub inverse: Option<Matrix4<f32>>,
}

impl specs::Component for Component {
    type Storage = specs::VecStorage<Component>;
}

impl Default for Component {
    #[inline(always)]
    fn default() -> Component { Component::new() }
}

impl Component {
    /// Create a new identity transform
    pub fn new() -> Component {
        Component {
            matrix: Matrix4::new_identity(4),
            inverse: None,
        }
    }
}

impl super::effector::Effector<Component> for Component {
    fn effect(&self, mut child: &mut Component) {
        child.matrix = self.matrix * child.matrix;
    }
}