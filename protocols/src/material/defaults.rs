use backend::generic::color::Color;

use super::*;
use ::named::*;

pub trait DefaultMaterial {
    #[inline(always)]
    fn default_texture() -> Option<String> { None }

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
            texture: Material::default_texture(),
            roughness: Material::default_roughness(),
            color: Material::default_color(),
            shader: Material::default_shader(),
            render: Material::default_render(),
        }
    }
}