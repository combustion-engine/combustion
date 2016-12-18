use std::collections::HashMap;
use std::ops::Deref;
use std::hash::{Hash, Hasher};

use backend::generic::color::Color;

pub mod defaults;
pub mod sample;

pub use super::named::*;
pub use self::defaults::*;

/// Map of materials used for a certain model or scene
#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialMap {
    pub map: HashMap<String, Material>
}

impl Deref for MaterialMap {
    type Target = HashMap<String, Material>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target { &self.map }
}

/// Represents a certain material for an object in a scene
#[derive(Debug, Serialize, Deserialize)]
pub struct Material {
    /// Texture to apply to the material
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "Material::default_texture")]
    pub texture: Option<String>,
    //TODO: Maybe texture opacity?
    /// Roughness of material for BRDF calculations
    #[serde(default = "Material::default_roughness")]
    pub roughness: f32,
    /// Color of material
    #[serde(default = "Material::default_color")]
    pub color: Color,
    /// What specific shader should be used for the material
    #[serde(default = "Material::default_shader")]
    pub shader: MaterialShader,
    /// How the object should be rendered
    #[serde(default = "Material::default_render")]
    pub render: RenderMethod,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RenderMethod {
    /// Use traditional forward rendering for this material.
    ///
    /// Suitable for non-opaque objects or with complex reflections.
    #[serde(rename = "forward")]
    Forward,
    /// Use a more efficient but less flexible deferred rendering pipeline for this material.
    ///
    /// Suitable for most opaque objects.
    #[serde(rename = "deferred")]
    Deferred,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MaterialShader {
    /// All-in-one lighting shader used in deferred or forward rendering contexts
    #[serde(rename = "uber")]
    Uber,
    #[serde(rename = "mirror")]
    Mirror,
    #[serde(rename = "metal")]
    Metal,
    #[serde(rename = "matte")]
    Matte,
    #[serde(rename = "substrate")]
    Substrate,
    #[serde(rename = "glass")]
    Glass,
}