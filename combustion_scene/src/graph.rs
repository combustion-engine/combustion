use std::ops::{Deref, DerefMut};

use petgraph::prelude::*;
use specs::{Entity, World};
use fnv::FnvHashMap;

use error::*;

pub type Ix = usize;

pub struct SceneGraph {
    graph: StableDiGraph<Entity, (), Ix>,
    root: NodeIndex<Ix>,
    entity_table: FnvHashMap<Entity, NodeIndex<Ix>>,
}

impl SceneGraph {
    pub fn new(world: &World) -> SceneGraph {
        let mut graph = StableDiGraph::default();

        let root = graph.add_node(world.create_later());

        SceneGraph { graph: graph, root: root, entity_table: FnvHashMap::default() }
    }

    #[inline(always)]
    pub fn root(&self) -> NodeIndex<Ix> { self.root }

    pub fn add_child(&mut self, parent: NodeIndex<Ix>, entity: Entity) -> NodeIndex<Ix> {
        let child_node = self.graph.add_node(entity);

        self.entity_table.insert(entity, child_node);

        self.graph.add_edge(parent, child_node, ());

        child_node
    }

    #[inline]
    pub fn add_root(&mut self, entity: Entity) -> NodeIndex<Ix> {
        let root = self.root;

        self.add_child(root, entity)
    }

    #[inline]
    pub fn lookup_index(&self, entity: Entity) -> Option<&NodeIndex<Ix>> {
        self.entity_table.get(&entity)
    }

    #[inline]
    pub fn lookup_entity(&self, index: NodeIndex<Ix>) -> Option<&Entity> {
        self.graph.node_weight(index)
    }

    pub fn reparent(&mut self, child: NodeIndex<Ix>, old_parent: NodeIndex<Ix>, new_parent: NodeIndex<Ix>) -> SceneResult<()> {
        if let Some(edge) = self.graph.find_edge(old_parent, child) {
            self.graph.remove_edge(edge);

            self.graph.add_edge(new_parent, child, ());

            Ok(())
        } else {
            Err(SceneError::InvalidEdge)
        }
    }

    pub fn recursive_remove(&mut self, node: NodeIndex<Ix>) -> SceneResult<()> {
        //TODO
        Ok(())
    }
}

impl Deref for SceneGraph {
    type Target = StableDiGraph<Entity, (), Ix>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target { &self.graph }
}

impl DerefMut for SceneGraph {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.graph }
}