//! Data structures for manipulating Models

use std::fmt::{Debug, Formatter, Result as FmtResult};

use common::traits::DefaultName;

use ::mesh::data::Mesh;
use ::math::data::Transform;

/// Node within a `Model`
#[derive(Named, Clone, Default, Serialize, Deserialize)]
pub struct Node {
    /// Name of the node
    #[serde(default = "Node::default_name")]
    pub name: String,
    /// List of meshes in this node
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub meshes: Vec<u32>,
    /// List of child nodes
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub children: Vec<Node>,
    /// List of transforms to apply to all meshes in this node and in child nodes
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub transforms: Vec<Transform>,
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, r#"Node {{name: "{}", meshes: {:?}, children: {:?}, transforms: {}}}"#,
               self.name, self.meshes, self.children, self.transforms.len())
    }
}

/// Whole model with nested node structure
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Model {
    /// Root node
    pub root: Node,
    /// List of meshes in the model.
    ///
    /// These should be accessed via the indices provided in the `Node`s
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub meshes: Vec<Mesh>,
    /// List of material names used in the model.
    ///
    /// These should be accessed via the indices provided in the `Node`s
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub materials: Vec<String>,
}

impl Debug for Model {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Model {{root: {:?}, meshes: {:?}}}", self.root, self.meshes)
    }
}