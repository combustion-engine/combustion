//! Entity components for the whole engine

use specs;

pub mod node;
pub mod renderable;
pub mod effector;
pub mod model;
pub mod mesh;
pub mod material;
pub mod instanced;
pub mod position;
pub mod isometry;
pub mod rotation;
pub mod quaternion_rotation;
pub mod scale;
pub mod transform;
pub mod camera;
pub mod light;
pub mod physics;

#[macro_export]
macro_rules! register_mod {
    ($world:expr, $component_mod:ident) => { $world.register::<$component_mod::Component>() }
}

pub mod constraints;

pub fn register_all(world: &mut specs::World) {
    register_mod!(world, node);
    register_mod!(world, renderable);
    register_mod!(world, effector);
    register_mod!(world, mesh);
    register_mod!(world, model);
    register_mod!(world, material);
    register_mod!(world, instanced);
    register_mod!(world, position);
    register_mod!(world, isometry);
    register_mod!(world, rotation);
    register_mod!(world, quaternion_rotation);
    register_mod!(world, scale);
    register_mod!(world, transform);
    register_mod!(world, camera);
    register_mod!(world, light);
    register_mod!(world, physics);

    constraints::register_all(world);
}