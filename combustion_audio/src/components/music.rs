use ecs;

pub struct Component;

impl ecs::Component for Component {
    type Storage = ecs::VecStorage<Component>;
}