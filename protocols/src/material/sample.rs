use std::collections::HashMap;

use super::*;
use ::named::*;

pub fn sample() -> MaterialMap {
    let mut map = HashMap::new();

    map.insert("Material 1".to_string(), Material {
        texture: Some("Sometexture.ctex".to_string()),
        roughness: 0.33,
        color: (0.12, 0.15, 0.34, 1.0).into(),
        shader: MaterialShader::Uber,
        render: RenderMethod::Deferred
    });

    map.insert("Material 2".to_string(), Material {
        texture: Some("glass1.ctex".to_string()),
        roughness: 0.05,
        color: (1.0, 1.0, 1.0, 1.0).into(),
        shader: MaterialShader::Glass,
        render: RenderMethod::Forward
    });

    MaterialMap { map: map }
}

#[cfg(test)]
mod test {
    #[test]
    pub fn json_test() {
        use serde_json::to_string_pretty;

        let sample_material = super::sample();

        println!("MaterialMap {}", to_string_pretty(&sample_material).unwrap());
    }
}