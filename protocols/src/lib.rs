#![feature(plugin)]
#![plugin(phf_macros)]

extern crate capnp;
extern crate capnpc;
#[macro_use]
extern crate phf;

#[macro_use]
extern crate combustion_common as common;

#[macro_use]
extern crate combustion_backend as backend;

pub mod protocols;