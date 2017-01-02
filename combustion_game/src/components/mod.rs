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

pub mod constraints;

pub fn register_all(world: &mut specs::World) {
    ecs_register_mod!(world, node);
    ecs_register_mod!(world, renderable);
    ecs_register_mod!(world, effector);
    ecs_register_mod!(world, mesh);
    ecs_register_mod!(world, model);
    ecs_register_mod!(world, material);
    ecs_register_mod!(world, instanced);
    ecs_register_mod!(world, position);
    ecs_register_mod!(world, isometry);
    ecs_register_mod!(world, rotation);
    ecs_register_mod!(world, quaternion_rotation);
    ecs_register_mod!(world, scale);
    ecs_register_mod!(world, transform);
    ecs_register_mod!(world, camera);
    ecs_register_mod!(world, light);
    ecs_register_mod!(world, physics);

    constraints::register_all(world);
}