//! Constraint components
//!
//! These are used to apply artificial constraints to entities after all physics and input related systems have finished.

use specs;

pub mod lookat;

pub fn register_all(world: &mut specs::World) {
    ecs_register_mod!(world, lookat);
}