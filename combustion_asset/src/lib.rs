//! `combustion_asset` crate
//!
//! This crate defines routines for loading and saving assets from/to both the protocols defined in `combustion_protocols`,
//! and in some cases external formats like textures as images and so forth.
//!
//! Most of the focus of this is not on saving/exporting, however, but on being able to load in and normalize many formats
//! for textures, models and anything else.

#![feature(plugin, conservative_impl_trait, box_syntax)]
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
extern crate fnv;
#[macro_use]
extern crate matches;

#[cfg(feature = "tar")]
extern crate tar;
#[cfg(feature = "flate2")]
extern crate flate2;
#[cfg(feature = "zip")]
extern crate zip;

#[cfg(feature = "json")]
extern crate serde_json as json;
#[cfg(feature = "yaml")]
extern crate serde_yaml as yaml;
#[cfg(feature = "bincode")]
extern crate bincode;
#[cfg(feature = "cbor")]
extern crate serde_cbor as cbor;
#[cfg(feature = "toml")]
extern crate toml;

#[macro_use]
extern crate trace_error;

extern crate combustion_common as common;
extern crate combustion_protocols as protocols;

pub mod error;
pub mod vfs;
pub mod asset;
pub mod external;
pub mod cache;
pub mod assets;