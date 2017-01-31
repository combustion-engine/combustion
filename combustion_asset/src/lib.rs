#![feature(plugin, conservative_impl_trait)]
#![plugin(phf_macros)]
#![allow(dead_code)]

extern crate capnp;
extern crate capnpc;
extern crate phf;
extern crate rayon;
extern crate nalgebra;
extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate assimp;
extern crate lz4;

#[macro_use]
extern crate combustion_common as common;

#[macro_use]
extern crate combustion_protocols as protocols;