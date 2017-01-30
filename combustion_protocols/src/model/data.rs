use ::mesh::data::Mesh;
use ::math::data::Transform;

#[derive(Clone, Serialize, Deserialize)]
pub struct Model {
    pub root: Node,
    pub meshes: Vec<Mesh>,
    pub materials: Vec<String>,
}

#[derive(Clone, Named, Serialize, Deserialize)]
pub struct Node {
    pub name: String,
    pub meshes: Vec<u32>,
    pub children: Vec<Node>,
    pub transforms: Vec<Transform>,
}