use specs;
use petgraph::graph::NodeIndex;
use ::scene::graph::Ix;

#[derive(Copy, Clone, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Component {
    index: NodeIndex<Ix>
}

impl specs::Component for Component {
    type Storage = specs::VecStorage<Component>;
}