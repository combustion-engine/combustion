#![feature(box_syntax)]

extern crate specs;
extern crate petgraph;
extern crate num_cpus;
extern crate fnv;

#[macro_use]
extern crate combustion_common as common;

pub mod error;
pub mod system;
pub mod systems;

pub type Delta = f64;
pub type Planner = specs::Planner<Delta>;