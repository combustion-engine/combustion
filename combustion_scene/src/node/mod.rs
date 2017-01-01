use ecs::Entity;
use fnv::FnvBuildHasher;
use typemap::{Key, SimpleTypeMap};

pub mod entity_node;
pub mod multi_entity_node;

pub use self::entity_node::EntityNode;
pub use self::multi_entity_node::MultiEntityNode;

pub enum SceneNodeKind {
    EntityNode(EntityNode),
    MultiEntityNode(MultiEntityNode),
    MetaNode,
}

pub struct SceneNode {
    pub kind: SceneNodeKind,
    properties: Option<SimpleTypeMap<FnvBuildHasher>>,
}

impl SceneNode {
    #[inline]
    pub fn new(kind: SceneNodeKind) -> SceneNode {
        SceneNode { kind: kind, properties: None }
    }

    #[inline]
    pub fn new_entity_node(entity: Entity) -> SceneNode {
        SceneNode::new(SceneNodeKind::EntityNode(EntityNode::new(entity)))
    }

    #[inline]
    pub fn new_multi_entity_node(entities: Option<Vec<Entity>>) -> SceneNode {
        SceneNode::new(SceneNodeKind::MultiEntityNode(MultiEntityNode::new(entities)))
    }

    #[inline]
    pub fn new_meta_node() -> SceneNode {
        SceneNode::new(SceneNodeKind::MetaNode)
    }

    #[inline]
    pub fn kind(&self) -> &SceneNodeKind { &self.kind }

    #[inline]
    pub fn kind_mut(&mut self) -> &mut SceneNodeKind { &mut self.kind }

    pub fn is_entity(&self) -> bool {
        match self.kind() {
            &SceneNodeKind::EntityNode(_) => true,
            _ => false,
        }
    }

    pub fn is_multi_entity(&self) -> bool {
        match self.kind() {
            &SceneNodeKind::MultiEntityNode(_) => true,
            _ => false,
        }
    }

    pub fn is_meta(&self) -> bool {
        match self.kind() {
            &SceneNodeKind::MetaNode => true,
            _ => false
        }
    }

    /// Set a type property on this node.
    ///
    /// If a value for this property already existed, the previous value is returned.
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