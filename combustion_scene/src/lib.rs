extern crate petgraph;
extern crate fnv;
extern crate typemap;

extern crate combustion_ecs as ecs;

/// Numeric index type
pub type Ix = usize;

pub mod error;
pub mod node;
pub mod edge;
pub mod graph;