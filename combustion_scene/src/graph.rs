use std::ops::{Deref, DerefMut};

use petgraph::prelude::*;
use petgraph::visit::*;
use petgraph::algo::*;

use ecs::{Entity, World};
use fnv::FnvHashMap;

use error::*;
use node::*;

/// Numeric index type
pub type Ix = usize;

//TODO
pub struct SceneEdge {}


pub struct SceneGraph {
    graph: StableDiGraph<SceneNode, SceneEdge, Ix>,
    cycle_state: DfsSpace<NodeIndex<Ix>, < StableDiGraph<SceneNode, SceneEdge, Ix> as Visitable >::Map>,
    root: NodeIndex<Ix>,
    entity_table: FnvHashMap<Entity, NodeIndex<Ix>>,
}

impl SceneGraph {
    pub fn new(world: &World) -> SceneGraph {
        let mut graph = StableDiGraph::default();

        let root = graph.add_node(SceneNode::new_entity_node(world.create_later()));

        SceneGraph { graph: graph, cycle_state: DfsSpace::default(), root: root, entity_table: FnvHashMap::default() }
    }

    #[inline(always)]
    pub fn root(&self) -> NodeIndex<Ix> { self.root }

    /// Adds a new scene node to the graph with the given parent, and returns the new node's index
    pub fn add_child(&mut self, parent: NodeIndex<Ix>, entity: Entity) -> NodeIndex<Ix> {
        let child_node = self.graph.add_node(SceneNode::new_entity_node(entity));

        self.entity_table.insert(entity, child_node);

        self.graph.add_edge(parent, child_node, SceneEdge {});

        child_node
    }

    /// Adds a new scene node to the graph with root as its parent, and returns the new node's index.
    #[inline]
    pub fn add_node(&mut self, entity: Entity) -> NodeIndex<Ix> {
        let root = self.root;

        self.add_child(root, entity)
    }

    /// Find the node index for a specific entity using a lookup table.
    ///
    /// This operation is `O(1)`
    #[inline]
    pub fn lookup_index(&self, entity: Entity) -> Option<&NodeIndex<Ix>> {
        self.entity_table.get(&entity)
    }

    /// Find the node value from the given index.
    ///
    /// This operation is `O(1)`
    #[inline]
    pub fn lookup_entity(&self, index: NodeIndex<Ix>) -> Option<&SceneNode> {
        self.graph.node_weight(index)
    }

    /// This operation is `O(e' + e')` for the two edge lookups.
    pub fn reparent(&mut self, child: NodeIndex<Ix>, old_parent: NodeIndex<Ix>, new_parent: NodeIndex<Ix>) -> SceneResult<()> {
        if has_path_connecting(&self.graph, new_parent, child, Some(&mut self.cycle_state)) {
            return Err(SceneError::WouldCycle);
        }

        if let Some(edge) = self.graph.find_edge(old_parent, child) {
            self.graph.remove_edge(edge);

            self.graph.add_edge(new_parent, child, SceneEdge {});

            Ok(())
        } else {
            Err(SceneError::InvalidEdge)
        }
    }

    pub fn recursive_remove(&mut self, _node: NodeIndex<Ix>) -> SceneResult<()> {
        //TODO
        Ok(())
    }
}

impl Deref for SceneGraph {
    type Target = StableDiGraph<SceneNode, SceneEdge, Ix>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target { &self.graph }
}

impl DerefMut for SceneGraph {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.graph }
}