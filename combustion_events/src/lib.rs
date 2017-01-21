//! Event handling

#![deny(missing_docs)]

#[macro_use]
extern crate combustion_common as common;

pub extern crate parallel_event_emitter;

/// Use my Parallel Event Emitter crate
pub use parallel_event_emitter::*;