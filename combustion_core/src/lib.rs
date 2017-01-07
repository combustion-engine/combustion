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
pub extern crate combustion_common as common;

#[macro_use]
pub extern crate combustion_backend as backend;

#[macro_use]
pub extern crate combustion_protocols as protocols;
pub extern crate combustion_asset as asset;
pub extern crate combustion_events as events;
pub extern crate combustion_ecs as ecs;

pub mod error;

#[macro_use]
pub mod scheduler;

//pub mod storage;
//pub mod scene;
//pub mod graphics;
//pub mod game;
//pub mod scripting;