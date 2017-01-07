#![allow(unused_imports, dead_code)]
#![allow(unknown_lints, inline_always, toplevel_ref_arg)]
#![feature(specialization)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate enum_primitive;
extern crate libc;
extern crate nalgebra;
extern crate num_traits;
extern crate time;
extern crate num_cpus;
extern crate vec_map;
extern crate lazy;

#[macro_use]
extern crate combustion_macros;

pub extern crate combustion_core as core;

//#[macro_use]
//pub mod components;
//pub mod resources;
//pub mod entities;
//pub mod systems;