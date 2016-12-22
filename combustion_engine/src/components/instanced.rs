//! Instanced rendering component

use specs;

#[derive(Default)]
pub struct Component;

impl specs::Component for Component {
    type Storage = specs::NullStorage<Component>;
}