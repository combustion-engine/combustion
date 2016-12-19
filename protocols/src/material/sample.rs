use super::*;

pub fn sample() -> MaterialMap {
    let mut materials = MaterialMap::default();

    materials.insert("Material 1".into(), Material {
        texture: Some("Sometexture.ctex".into()),
        roughness: 0.33,
        metallic: Some(0.4),
        color: (0.12, 0.15, 0.34, 1.0).into(),
        shader: MaterialShader::Uber,
        render: RenderMethod::Deferred,
        ..Material::default()
    });

    materials.insert("Material 2".into(), Material {
        texture: Some("glass1.ctex".into()),
        roughness_map: Some("glass1_roughness.ctex".into()),
        roughness: 0.05,
        color: (1.0, 1.0, 1.0, 1.0).into(),
        shader: MaterialShader::Glass,
        render: RenderMethod::Forward,
        ..Material::default()
    });

    materials
}