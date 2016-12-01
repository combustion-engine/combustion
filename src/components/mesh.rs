//! Mesh component
//!
//! Stores a reference to a mesh to render

use std::sync::{Arc, RwLock};

use specs;
use assimp;

pub struct Component {
    pub source: usize,
    pub index: usize
}

impl specs::Component for Component {
    type Storage = specs::VecStorage<Component>;
}

impl Component {
    /// Stores a references to a mesh
    pub fn new(source: usize, index: usize) -> Component {
        Component { source: source, index: index }
    }
}