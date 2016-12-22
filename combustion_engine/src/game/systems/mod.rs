//! Systems specific to the game

pub mod turntable;
pub mod blackhole;
pub mod bob;

pub use systems::Delta;

pub enum Priorities {
    Blackhole,
    Turntable,
    Bob,
}