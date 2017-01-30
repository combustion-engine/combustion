use std::collections::HashMap;

use nalgebra::*;

use common::color::Color;
use common::color::de as color_de;

pub use ::traits::*;

pub mod defaults;
pub mod sample;

pub use self::defaults::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Scene {
    #[serde(default = "Scene::default_name")]
    pub name: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub lights: Vec<Light>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub materials: Vec<Material>,
    pub root: Node,
}

impl_named!(Scene);

/// Transforms that are applicable to a scene node. Most are pretty obvious.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum NodeTransform {
    #[serde(rename = "translate")]
    Translate(Vector3<f32>),
    /// Rotation as Euler angles
    #[serde(rename = "rotate")]
    Rotate(Vector3<f32>),
    #[serde(rename = "scale")]
    Scale(Vector3<f32>),
    #[serde(rename = "transform")]
    Transform(Matrix4<f32>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub children: Vec<Node>,
    /// Transforms to apply to node children, in applied order
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub transform: Vec<NodeTransform>
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LightKind {
    #[serde(rename = "directional")]
    Directional,
    #[serde(rename = "point")]
    Point,
    #[serde(rename = "spotlight")]
    Spotlight,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Light {
    #[serde(default = "Light::default_name")]
    pub name: String,
    #[serde(default = "Light::default_zdistance")]
    pub zdistance: (f32, f32),
    #[serde(default = "Light::default_position")]
    pub position: Point3<f32>,
    #[serde(default = "Light::default_direction")]
    pub direction: Vector3<f32>,
    #[serde(skip_serializing_if = "Color::is_none")]
    #[serde(deserialize_with = "color_de::from_name_or_value")]
    #[serde(default = "Color::white")]
    pub color: Color,
    #[serde(skip_serializing_if = "Color::is_none")]
    #[serde(deserialize_with = "color_de::from_name_or_value")]
    #[serde(default = "Color::none")]
    pub ambient: Color,
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

impl_named!(Light);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material {
    #[serde(default = "Material::default_name")]
    pub name: String,
}

impl_named!(Material);