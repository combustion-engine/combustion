use std::ops::{Deref, DerefMut};

use specs;

use ::scene::graph::SceneGraph;

pub struct Resource {
    graph: SceneGraph
}

impl Resource {
    pub fn new(world: &specs::World) -> Resource {
        Resource { graph: SceneGraph::new(world) }
    }
}

impl From<SceneGraph> for Resource {
    fn from(graph: SceneGraph) -> Resource {
        Resource { graph: graph }
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