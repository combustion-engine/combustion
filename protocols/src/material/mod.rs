use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;

use backend::generic::color::Color;
use backend::generic::color::de as color_de;

pub mod defaults;
pub mod sample;
pub mod anisotropy;

pub use self::defaults::*;
pub use self::anisotropy::MaterialAnisotropy;

use self::anisotropy::de as anisotropy_de;

/// Map of materials used for a certain model or scene
#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialMap {
    pub materials: HashMap<String, Material>
}

impl Deref for MaterialMap {
    type Target = HashMap<String, Material>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target { &self.materials }
}

impl DerefMut for MaterialMap {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.materials }
}

/// Represents a certain material for an object in a scene.
#[derive(Debug, Serialize, Deserialize)]
pub struct Material {
    /// Presets allow for materials to inherit properties from another material
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "Option::default")]
    pub preset: Option<String>,
    /// Path to texture to apply to the material
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "Option::default")]
    pub texture: Option<PathBuf>,
    //TODO: Maybe texture opacity?
    /// Path to normal map for material
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "Option::default")]
    pub normal_map: Option<PathBuf>,
    /// Path to tangent map for material
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "Option::default")]
    pub tangent_map: Option<PathBuf>,
    /// Path to height map for material.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "Option::default")]
    pub height_map: Option<PathBuf>,
    /// Path to texture to be used as roughness values.
    ///
    /// See `roughness` field for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "Option::default")]
    pub roughness_map: Option<PathBuf>,
    /// Path to texture to be used as metallic values.
    ///
    /// See `metallic` field for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "Option::default")]
    pub metallic_map: Option<PathBuf>,
    /// Roughness of material for BRDF calculations
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "Option::default")]
    pub roughness: Option<f32>,
    /// Alternative to Roughness.
    ///
    /// It is converted to roughness via `roughness = pow(1.0 - smoothness, 2.0)`
    ///
    /// If both smoothness and roughness are specified, they are averaged together.
    ///
    /// Use whichever makes the most artistic sense to you.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "Option::default")]
    pub smoothness: Option<f32>,
    /// Metallic-ness of the material.
    ///
    /// A value of `None` means the material is purely dialectic.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "Option::default")]
    pub metallic: Option<f32>,
    /// Color of material
    #[serde(skip_serializing_if = "Color::is_none")]
    #[serde(deserialize_with = "color_de::from_name_or_value")]
    #[serde(default = "Color::none")]
    pub color: Color,
    /// Emissive materials emit light from their surface.
    ///
    /// Uses the `color` field for the emitted light color.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "Option::default")]
    pub emission: Option<f32>,
    /// Overall translucency for the material. 0.0 is totally transparent and 1.0 is fully opaque.
    ///
    /// This entry can be omitted or set to `None` to assume fully opaque materials.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "Option::default")]
    pub translucency: Option<f32>,
    /// Index-of-Refraction for material
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "Option::default")]
    pub ior: Option<f32>,
    /// What specific shader should be used for the material
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "Option::default")]
    pub shader: Option<MaterialShader>,
    /// How the object should be rendered
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "Option::default")]
    pub render: Option<RenderMethod>,
    /// Anisotropy of the material
    ///
    /// Uses model tangents and tangent maps
    #[serde(skip_serializing_if = "MaterialAnisotropy::is_none")]
    #[serde(deserialize_with = "anisotropy_de::num_or_anisotropy")]
    #[serde(default = "MaterialAnisotropy::default")]
    pub anisotropy: MaterialAnisotropy,
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
    /// Suitable for most opaque objects. Alpha for this material is always interpreted as `1.0`.
    #[serde(rename = "deferred")]
    Deferred,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MaterialShader {
    /// All-in-one lighting shader used in deferred or forward rendering contexts
    #[serde(rename = "uber")]
    Uber,
    /// Shader optimized for mirror-like surfaces
    #[serde(rename = "mirror")]
    Mirror,
    /// Shader optimized for metallic surfaces
    #[serde(rename = "metal")]
    Metal,
    /// Shader optimized for simple, flat surfaces
    #[serde(rename = "matte")]
    Matte,
    #[serde(rename = "substrate")]
    Substrate,
    /// Shader optimized for transparent objects
    #[serde(rename = "glass")]
    Glass,
}