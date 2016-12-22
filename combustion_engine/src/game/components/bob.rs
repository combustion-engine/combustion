use specs;

#[derive(Default)]
pub struct Component {
    pub up: bool,
    pub value: f32
}

impl specs::Component for Component {
    type Storage = specs::VecStorage<Component>;
}

impl Component {
    pub fn new() -> Component {
        Component { up: true, value: 0.0 }
    }
}