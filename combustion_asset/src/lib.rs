//! combustion_asset crate
//!
//! This crate defines routines for loading and saving assets from/to both the protocols defined in combustion_protocols,
//! and in some cases external formats like textures as images and so forth.
//!
//! Most of the focus of this is not on saving/exporting, however, but on being able to load in and normalize many formats
//! for textures, models and anything else.

#![feature(plugin, conservative_impl_trait)]
#![plugin(phf_macros)]
#![allow(dead_code)]
#![warn(missing_docs)]

extern crate capnp;
extern crate capnpc;
extern crate phf;
extern crate rayon;
extern crate nalgebra;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate image;
extern crate assimp;
extern crate lz4;

#[macro_use]
extern crate trace_error;

extern crate combustion_common as common;
extern crate combustion_protocols as protocols;

pub mod error;
pub mod asset;
pub mod assets;