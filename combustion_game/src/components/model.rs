//! The model component is a mostly symbolic component for entities
//! who are parents of mesh entity-components.

use specs;

pub struct Component {
    name: String,
}

impl specs::Component for Component {
    type Storage = specs::VecStorage<Component>;
}