#![feature(plugin, conservative_impl_trait)]
#![plugin(phf_macros)]
#![allow(dead_code, unknown_lints, inline_always)]

extern crate capnp;
extern crate capnpc;
#[macro_use]
extern crate phf;
extern crate nalgebra;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate trace_error;

#[macro_use]
extern crate combustion_common as common;

pub mod error;

#[macro_use]
pub mod named;

pub mod utils;
pub mod math;
pub mod mesh;
pub mod model;
pub mod scene;
pub mod texture;
pub mod material;