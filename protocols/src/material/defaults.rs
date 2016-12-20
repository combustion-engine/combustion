use std::collections::HashMap;

use backend::generic::color::Color;

use super::*;

impl Default for Material {
    fn default() -> Material {
        Material {
            preset: None,
            texture: None,
            normal_map: None,
            tangent_map: None,
            roughness_map: None,
            metallic_map: None,
            height_map: None,
            roughness: None,
            smoothness: None,
            metallic: None,
            color: Color::none(),
            emission: None,
            translucency: None,
            ior: None,
            shader: None,
            render: None,
            anisotropy: MaterialAnisotropy::default(),
        }
    }
}

impl Default for MaterialMap {
    fn default() -> MaterialMap {
        MaterialMap { materials: HashMap::default() }
    }
}