use super::*;

use std::iter::{Iterator, repeat};

pub fn generate_named<T: Named + Default + Clone>(name: &'static str, len: usize) -> impl Iterator<Item = T> {
    repeat(T::default()).take(len).enumerate().map(move |(i, mut item)| {
        item.set_name(format!("{} {}", name, i + 1).to_string());

        item
    })
}

pub fn sample() -> Scene {
    Scene {
        name: Scene::default_name(),
        lights: generate_named("Untitled Light", 2).collect(),
        materials: generate_named("Untitled Material", 2).collect(),
        root: Node {
            children: vec![],
            transform: vec![
            NodeTransform::Transform(Matrix4::new_identity(4)),
            NodeTransform::Translate(Vector3::new(1.2, -0.251, 0.1456))
            ]
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    pub fn json_test() {
        use serde_json::to_string_pretty;

        let sample_scene = super::sample();

        println!("Scene {}", to_string_pretty(&sample_scene).unwrap());
    }
}