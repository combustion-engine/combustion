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
extern crate combustion_common as common;

#[macro_use]
extern crate combustion_backend as backend;

#[macro_use]
pub mod named;

/// Utility structures used within the protocols
pub mod utils {
    pub mod protocol {
        include!(concat!(env!("OUT_DIR"), "/protocols/utils_capnp.rs"));
    }
}

pub mod math;
pub mod mesh;
pub mod model;
pub mod scene;
pub mod texture;
pub mod material;