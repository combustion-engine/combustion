#![feature(box_syntax)]

extern crate specs;
extern crate petgraph;
//extern crate num_cpus;
extern crate fnv;

#[macro_use]
extern crate combustion_common as common;

pub mod error;
pub mod builder;
pub mod macros;

pub type Delta = f64;
pub type Planner = specs::Planner<Delta>;

pub use specs::{
    Entity,
    Component,
    System,
    World,
    VecStorage,
    NullStorage,
    HashMapStorage,
    UnprotectedStorage,
    RunArg
};
