//! `combustion_protocols` crate
//!
//! This crate defines all data structures, protocols and storage routines for interacting with engine-specific data,
//! such as textures, meshes, models, scene descriptions, etc, etc.
//!
//! For the actual loading and management of Combustion assets and external file formats, see the `combustion_asset` crate

#![feature(plugin, conservative_impl_trait, box_syntax)]
#![plugin(phf_macros)]
#![allow(dead_code, unknown_lints, inline_always)]
#![deny(missing_docs)]

extern crate capnp;
extern crate capnpc;
extern crate phf;
extern crate nalgebra;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate trace_error;
extern crate base64;
extern crate blob;
#[cfg(test)]
extern crate serde_json;

#[macro_use]
extern crate combustion_macros;
extern crate combustion_common as common;

pub mod error;

pub mod traits;

pub mod math;
pub mod mesh;
pub mod model;
pub mod scene;
pub mod texture;
pub mod material;

/// Protocol utilities
pub mod utils {
    pub mod protocol {
        #![allow(missing_docs)]

        include!(concat!(env!("OUT_DIR"), "/protocols/utils_capnp.rs"));
    }
}