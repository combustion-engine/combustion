#![feature(plugin, proc_macro, conservative_impl_trait)]
#![plugin(phf_macros)]

extern crate capnp;
extern crate capnpc;
#[macro_use]
extern crate phf;

#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate nalgebra;

#[macro_use]
extern crate combustion_common as common;

#[macro_use]
extern crate combustion_backend as backend;

pub mod protocols;