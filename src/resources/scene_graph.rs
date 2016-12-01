use std::ops::{Deref, DerefMut};

use specs;

use ::storage::scene::graph::SceneGraph;

pub struct Resource {
    graph: SceneGraph
}

impl Resource {
    pub fn new(world: &specs::World) -> Resource {
        Resource { graph: SceneGraph::new(world) }
    }
}

impl Deref for Resource {
    type Target = SceneGraph;

    #[inline(always)]
    fn deref(&self) -> &Self::Target { &self.graph }
}

impl DerefMut for Resource {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.graph }
}