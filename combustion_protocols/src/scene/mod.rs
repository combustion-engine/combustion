//! Scene description structures

use std::collections::HashMap;

use nalgebra::*;

use common::color::Color;
use common::color::de as color_de;
use common::traits::named::DefaultName;

use ::math::data::Transform;

pub mod defaults;

#[cfg(feature = "sample")]
pub mod sample;

pub use self::defaults::*;

/// Entire scene description
#[derive(Debug, Named, Serialize, Deserialize)]
pub struct Scene {
    /// Name of the scene. Will default to `"Untitled Scene"` if one is not specified.
    #[serde(default = "Scene::default_name")]
    pub name: String,
    /// List of lights the scene contains
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub lights: Vec<Light>,
    /// List of materials used in the scene
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub materials: Vec<Material>,
    /// Root node
    pub root: Node,
}

/// A single scene node
#[derive(Debug, Named, Serialize, Deserialize)]
pub struct Node {
    /// Name of the node. Will default to `"Untitled None"` if one is not specified.
    #[serde(default = "Node::default_name")]
    name: String,
    /// Children of this node.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub children: Vec<Node>,
    /// Transforms to apply to node children, in applied order
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub transform: Vec<Transform>
}

/// Varieties of lights
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LightKind {
    /// Directional light
    #[serde(rename = "directional")]
    Directional,
    /// Point light
    #[serde(rename = "point")]
    Point,
    /// Spotlight
    #[serde(rename = "spotlight")]
    Spotlight,
}

/// Light structure
#[derive(Debug, Named, Clone, Serialize, Deserialize)]
pub struct Light {
    /// Name of the light. Will default to `"Untitled Light"` if one is not specified.
    #[serde(default = "Light::default_name")]
    pub name: String,
    /// Minimum and maximum distances the light can affect
    #[serde(default = "Light::default_zdistance")]
    pub zdistance: (f32, f32),
    /// Position in the light in the world
    #[serde(default = "Light::default_position")]
    pub position: Point3<f32>,
    /// Orientation of the light
    #[serde(default = "Light::default_direction")]
    pub direction: Vector3<f32>,
    /// Color of the light
    #[serde(skip_serializing_if = "Color::is_none")]
    #[serde(deserialize_with = "color_de::from_name_or_value")]
    #[serde(default = "Color::white")]
    pub color: Color,
    /// Ambient color "emitted" from the light to add into the scene
    #[serde(skip_serializing_if = "Color::is_none")]
    #[serde(deserialize_with = "color_de::from_name_or_value")]
    #[serde(default = "Color::none")]
    pub ambient: Color,
    /// Light kind variant
    #[serde(default = "Light::default_kind")]
    pub kind: LightKind,
    /// Effect radius of light, outside of which the light does not illuminate
    #[serde(default = "Light::default_effect_radius")]
    pub effect_radius: f32,
    /// Inner cone angle (in radians) for spotlights
    #[serde(default = "Light::default_inner_cone")]
    pub inner_cone: f32,
    /// Outer cone angle (in radians) for spotlights
    #[serde(default = "Light::default_outer_cone")]
    pub outer_cone: f32,
    /// Intensity (brightness) of the light
    #[serde(default = "Light::default_intensity")]
    pub intensity: f32,
    /// Any arbitrary properties the engine might check for
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[serde(default)]
    pub properties: HashMap<String, String>,
}

/// Material structure
#[derive(Debug, Named, Clone, Serialize, Deserialize)]
pub struct Material {
    /// Name of the material used
    #[serde(default = "Material::default_name")]
    pub name: String,
}