use specs;

pub struct Component {
    name: String,
}

impl specs::Component for Component {
    type Storage = specs::VecStorage<Component>;
}