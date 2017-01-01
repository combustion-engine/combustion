use ecs::Entity;
use fnv::FnvBuildHasher;
use typemap::{Key, SimpleTypeMap};

pub mod entity_node;
pub mod multi_entity_node;

pub use self::entity_node::EntityNode;
pub use self::multi_entity_node::MultiEntityNode;

/// Kinds of scene nodes that can be in the graph.
pub enum SceneNodeKind {
    EntityNode(EntityNode),
    MultiEntityNode(MultiEntityNode),
    MetaNode,
}

/// A single scene node which can contain any `SceneNodeKind` and arbitrary typed properties.
pub struct SceneNode {
    kind: SceneNodeKind,
    properties: Option<SimpleTypeMap<FnvBuildHasher>>,
}

/// Extra convenience constructors and check functions for scene nodes
pub trait SceneNodeExt {
    fn new_entity_node(entity: Entity) -> SceneNode;
    fn new_multi_entity_node(entities: Option<Vec<Entity>>) -> SceneNode;
    fn new_meta_node() -> SceneNode;
    fn is_entity(&self) -> bool;
    fn is_multi_entity(&self) -> bool;
    fn is_meta(&self) -> bool;
}

impl SceneNode {
    #[inline]
    pub fn new(kind: SceneNodeKind) -> SceneNode {
        SceneNode { kind: kind, properties: None }
    }

    #[inline]
    pub fn kind(&self) -> &SceneNodeKind { &self.kind }

    #[inline]
    pub fn kind_mut(&mut self) -> &mut SceneNodeKind { &mut self.kind }

    /// Set a type property on this node.
    ///
    /// Returns the previous value if one existed.
    pub fn set_property<T: Key>(&mut self, value: T::Value) -> Option<T::Value> {
        if let Some(ref mut props) = self.properties {
            props.insert::<T>(value)
        } else {
            let mut map = SimpleTypeMap::custom();

            let previous = map.insert::<T>(value);

            self.properties = Some(map);

            previous
        }
    }

    pub fn get_property<T: Key>(&self) -> Option<&T::Value> {
        if let Some(ref props) = self.properties {
            props.get::<T>()
        } else {
            None
        }
    }

    pub fn get_property_mut<T: Key>(&mut self) -> Option<&mut T::Value> {
        if let Some(ref mut props) = self.properties {
            props.get_mut::<T>()
        } else {
            None
        }
    }

    pub fn has_property<T: Key>(&self) -> bool {
        if let Some(ref props) = self.properties {
            props.contains::<T>()
        } else {
            false
        }
    }
}

impl SceneNodeExt for SceneNode {
    #[inline]
    fn new_entity_node(entity: Entity) -> SceneNode {
        SceneNode::new(SceneNodeKind::EntityNode(EntityNode::new(entity)))
    }

    #[inline]
    fn new_multi_entity_node(entities: Option<Vec<Entity>>) -> SceneNode {
        SceneNode::new(SceneNodeKind::MultiEntityNode(MultiEntityNode::new(entities)))
    }

    #[inline]
    fn new_meta_node() -> SceneNode {
        SceneNode::new(SceneNodeKind::MetaNode)
    }

    fn is_entity(&self) -> bool {
        match self.kind() {
            &SceneNodeKind::EntityNode(_) => true,
            _ => false,
        }
    }

    fn is_multi_entity(&self) -> bool {
        match self.kind() {
            &SceneNodeKind::MultiEntityNode(_) => true,
            _ => false,
        }
    }

    fn is_meta(&self) -> bool {
        match self.kind() {
            &SceneNodeKind::MetaNode => true,
            _ => false
        }
    }
}