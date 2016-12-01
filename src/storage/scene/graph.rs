use specs;
use petgraph::*;
use petgraph::stable_graph::*;

pub type Ix = u32;
pub type SceneGraphInner = StableDiGraph<specs::Entity, (), Ix>;

pub struct SceneGraph {
    graph: SceneGraphInner,
    root: NodeIndex<Ix>
}

impl SceneGraph {
    pub fn new(world: &specs::World) -> SceneGraph {
        let mut graph = StableDiGraph::new();

        let root = graph.add_node(world.create_later());

        SceneGraph {
            graph: graph,
            root: root
        }
    }

    #[inline(always)]
    pub fn root(&self) -> NodeIndex<Ix> { self.root }

    #[inline(always)]
    pub fn graph(&self) -> &SceneGraphInner { &self.graph }

    pub fn add_child(&mut self, parent: NodeIndex<Ix>, entity: specs::Entity) -> NodeIndex<Ix> {
        let child = self.graph.add_node(entity);

        self.graph.add_edge(parent, child, ());

        child
    }
}