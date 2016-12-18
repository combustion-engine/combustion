use std::collections::HashMap;

use nalgebra::*;

use backend::generic::color::Color;

pub mod defaults;
pub mod sample;

pub use super::named::*;
pub use self::defaults::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Scene {
    #[serde(default = "Scene::default_name")]
    pub name: String,
    pub lights: Vec<Light>,
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
    #[serde(default)]
    pub children: Vec<Node>,
    /// Transforms to apply to node children, in applied order
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
    #[serde(default = "Color::white")]
    pub color: Color,
    #[serde(default = "Color::none")]
    pub ambient: Color,
    #[serde(default = "Light::default_kind")]
    pub kind: LightKind,
    /// Effect radius of light, outside of which the light does not illuminate
    #[serde(default = "Light::default_radius")]
    pub radius: f32,
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