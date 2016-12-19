use super::*;

use backend::generic::color::palette::*;

pub fn sample() -> MaterialMap {
    let mut materials = MaterialMap::default();

    materials.insert("MyMaterial 1".into(), Material {
        texture: Some("Sometexture.ctex".into()),
        roughness: Some(0.33),
        metallic: Some(0.4),
        color: (0.12, 0.15, 0.34, 1.0).into(),
        shader: Some(MaterialShader::Uber),
        render: Some(RenderMethod::Deferred),
        ..Material::default()
    });

    materials.insert("sapphire".into(), Material {
        preset: Some("glass".into()),
        roughness_map: Some("tiny_scratches.ctex".into()),
        roughness: Some(0.05),
        color: Srgb::<_, f32>::new_u8(15, 82, 186).into(),
        ior: Some(1.763),
        ..Material::default()
    });

    materials
}