use std::iter::{Iterator, repeat};

use common::traits::Named;

use ::math::data::Transform;

use super::*;

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
            name: "Test node".into(),
            children: vec![],
            transform: vec![
                Transform::Matrix(Matrix4::new_identity(4)),
                Transform::Translation(Vector3::new(1.2, -0.251, 0.1456))
            ]
        }
    }
}