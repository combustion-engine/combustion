//! Entity components specific to the game

use specs;

pub mod turntable;

pub fn register_all(world: &mut specs::World) {
    register_mod!(world, turntable);
}