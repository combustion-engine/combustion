//! Entity components specific to the game

use specs;

pub mod turntable;
pub mod bob;

pub fn register_all(world: &mut specs::World) {
    register_mod!(world, turntable);
    register_mod!(world, bob);
}