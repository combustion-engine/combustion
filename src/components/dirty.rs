use specs;

#[derive(Clone, Debug)]
pub struct Component(pub bool);

impl specs::Component for Component {
    type Storage = specs::VecStorage<Component>;
}