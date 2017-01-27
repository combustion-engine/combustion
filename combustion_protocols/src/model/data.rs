use ::named::Named;

use ::mesh::data::Mesh;

use ::math::Transform;

pub struct Model {
    pub root: Node,
    pub meshes: Vec<Mesh>,
    materials: Vec<String>,
}

pub struct Node {
    pub name: String,
    pub meshes: Vec<u32>,
    pub children: Vec<Node>,
    pub transforms: Vec<Transform>,
}

impl_named!(Node);