use std::collections::HashMap;

use backend::generic::color::Color;

use super::*;

pub trait DefaultMaterial {
    #[inline(always)]
    fn default_roughness() -> f32 { 0.25 }

    #[inline(always)]
    fn default_color() -> Color { Color::white() }

    #[inline(always)]
    fn default_shader() -> MaterialShader { MaterialShader::Uber }

    #[inline(always)]
    fn default_render() -> RenderMethod { RenderMethod::Deferred }
}

impl DefaultMaterial for Material {}

impl Default for Material {
    fn default() -> Material {
        Material {
            texture: None,
            normal_map: None,
            roughness_map: None,
            metallic_map: None,
            height_map: None,
            roughness: Material::default_roughness(),
            metallic: None,
            color: Material::default_color(),
            emission: None,
            translucency: None,
            shader: Material::default_shader(),
            render: Material::default_render(),
        }
    }
}

impl Default for MaterialMap {
    fn default() -> MaterialMap {
        MaterialMap { materials: HashMap::default() }
    }
}