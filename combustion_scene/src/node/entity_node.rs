use ecs::Entity;

pub struct EntityNode {
    entity: Entity,
}

impl EntityNode {
    /// Create a new `EntityNode` with no properties allocated
    pub fn new(entity: Entity) -> EntityNode {
        EntityNode { entity: entity }
    }

    /// Get the entity associated with this node
    #[inline]
    pub fn entity(&self) -> Entity { self.entity }
}
