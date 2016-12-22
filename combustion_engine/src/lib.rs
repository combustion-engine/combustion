#![allow(unused_imports, dead_code)]
#![allow(unknown_lints, inline_always, toplevel_ref_arg)]
#![feature(proc_macro, receiver_try_iter, specialization)]
#![crate_type = "bin"]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate enum_primitive;
extern crate libc;
extern crate nalgebra;
extern crate rusttype;
extern crate image;
extern crate palette;
extern crate num_traits;
extern crate regex;
extern crate glfw;
extern crate time;
extern crate specs;
extern crate num_cpus;
extern crate vec_map;
extern crate petgraph;
extern crate lazy;
extern crate capnp;
extern crate capnpc;

#[macro_use]
extern crate combustion_common as common;

#[macro_use]
extern crate combustion_backend as backend;

#[macro_use]
extern crate combustion_protocols;

pub use common;
pub use backend;
pub use combustion_protocols as protocols;

pub mod error;

#[macro_use]
pub mod components;
pub mod resources;
pub mod entities;
pub mod systems;

pub mod storage;
pub mod scene;
pub mod graphics;
pub mod game;
pub mod scripting;