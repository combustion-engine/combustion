use ecs;

pub mod sound;
pub mod sample;
pub mod music;

pub fn register_all(world: &mut ecs::World) {
    ecs_register_mod!(world, sound);
    ecs_register_mod!(world, sample);
    ecs_register_mod!(world, music);
}