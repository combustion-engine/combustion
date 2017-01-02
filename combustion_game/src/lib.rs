#![allow(unused_imports, dead_code)]
#![allow(unknown_lints, inline_always, toplevel_ref_arg)]
#![feature(proc_macro, specialization)]
#![crate_type = "bin"]

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

extern crate combustion_core as core;

#[macro_use]
extern crate combustion_common as common;
#[macro_use]
extern crate combustion_backend as backend;
#[macro_use]
extern crate combustion_protocols as protocols;

extern crate combustion_asset as asset;
extern crate combustion_ecs as ecs;
extern crate combustion_audio as audio;

#[macro_use]
pub mod components;
pub mod resources;
pub mod entities;
pub mod systems;