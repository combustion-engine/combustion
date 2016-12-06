//! 2D Sprite component

use specs;

pub struct Component;

impl specs::Component for Component {
    type Storage = specs::VecStorage<Component>;
}