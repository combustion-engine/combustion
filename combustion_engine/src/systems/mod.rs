pub mod render;
pub mod clean;
pub mod physics;
pub mod transform;
pub mod constraints;

pub type Delta = f32;

pub enum Priorities {
    LAST = 0,
    Render,
    Constraints,
    Transforms,
    Physics,
    Clean,
    FIRST
}