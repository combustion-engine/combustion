//! Mesh component
//!
//! Stores a reference to a mesh to render

use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::ptr;
use std::mem;

use specs;
use lazy;

use ::backend::gl::*;
use ::backend::gl::types::*;
use ::backend::gl::bindings as glb;

/// Fields to be used in specifying shader layouts
pub enum BufferField {
    Vertex,
    Normal,
    Uv,
    Tangent,
    Bitangent,
}

pub struct Buffer {
    vao: GLVertexArray,
    num_indices: usize,
}

pub struct Component {
    pub mesh: Arc<RwLock<()>>,
    pub buffer: (),
}

impl specs::Component for Component {
    type Storage = specs::VecStorage<Component>;
}

impl Component {
    /// Stores a references to a mesh
    pub fn new(source: usize, index: usize) -> Component {
        Component { source: source, index: index, buffer: () }
    }
}