use specs::Entity;
use fnv::FnvBuildHasher;
use typemap::{Key, SimpleTypeMap};

/// Scene node that contains an entity and a type-safe property map,
/// where values are unique to types.
pub struct SceneNode {
    entity: Entity,
    properties: Option<SimpleTypeMap<FnvBuildHasher>>,
}

impl SceneNode {
    /// Create a new `SceneNode` with no properties allocated
    pub fn new(entity: Entity) -> SceneNode {
        SceneNode { entity: entity, properties: None }
    }

    /// Get the entity associated with this node
    #[inline]
    pub fn entity(&self) -> Entity { self.entity }

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

#[cfg(test)]
mod test {
    use super::*;
    use specs::*;

    struct TestProperty {
        value: i32
    }

    impl Key for TestProperty {
        type Value = TestProperty;
    }

    struct TestKey;

    impl Key for TestKey {
        type Value = TestProperty;
    }

    #[test]
    fn test_null_property() {
        let world = World::new();

        let e = world.create_later();

        let mut node = SceneNode::new(e);

        node.set_property::<TestProperty>(TestProperty { value: 42 });
        node.set_property::<TestKey>(TestProperty { value: 16 });

        assert_eq!(node.get_property::<TestProperty>().unwrap().value, 42);
        assert_eq!(node.get_property::<TestKey>().unwrap().value, 16);
    }
}