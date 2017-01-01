use std::ops::Deref;

use ecs::Entity;

pub struct MultiEntityNode {
    entities: Vec<Entity>,
}

impl MultiEntityNode {
    /// Create a new `MultiEntityNode`
    pub fn new(entities: Option<Vec<Entity>>) -> MultiEntityNode {
        MultiEntityNode { entities: entities.unwrap_or_default() }
    }
}

impl Deref for MultiEntityNode {
    type Target = Vec<Entity>;

    fn deref(&self) -> &Vec<Entity> { &self.entities }
}