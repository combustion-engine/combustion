#![feature(plugin, conservative_impl_trait)]
#![plugin(phf_macros)]
#![allow(dead_code)]

#[macro_use]
extern crate combustion_common as common;

#[macro_use]
extern crate combustion_protocols as protocols;

extern crate capnp;
extern crate capnpc;

#[macro_use]
extern crate phf;
extern crate rayon;
extern crate nalgebra;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate serde_derive;

extern crate assimp;
extern crate lz4;

pub mod model;
pub mod material;