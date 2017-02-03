//! Data structures for manipulating Models

use ::mesh::data::Mesh;
use ::math::data::Transform;

/// Whole model with nested node structure
#[derive(Clone, Serialize, Deserialize)]
pub struct Model {
    /// Root node
    pub root: Node,
    /// List of meshes in the model.
    ///
    /// These should be accessed via the indices provided in the `Node`s
    pub meshes: Vec<Mesh>,
    /// List of material names used in the model.
    ///
    /// These should be accessed via the indices provided in the `Node`s
    pub materials: Vec<String>,
}

/// Node within a `Model`
#[derive(Clone, Named, Serialize, Deserialize)]
pub struct Node {
    /// Name of the node
    pub name: String,
    /// List of meshes in this node
    pub meshes: Vec<u32>,
    /// List of child nodes
    pub children: Vec<Node>,
    /// List of transforms to apply to all meshes in this node and in child nodes
    pub transforms: Vec<Transform>,
}