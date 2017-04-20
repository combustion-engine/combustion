//! Common stuff for the Combustion engine.
//!
//! "Stuff", in this case, being assorted tools, data structures,
//! and macros that don't belong in any single crate yet are used often in the engine.

#![deny(missing_docs)]
#![allow(unknown_lints, inline_always)]

extern crate vec_map;
extern crate num_traits;

#[cfg(feature = "mmap")]
extern crate memmap;
extern crate nalgebra;
extern crate palette;
extern crate time;
extern crate void;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate lz4;
extern crate rand;

pub mod traits;
pub mod macros;
pub mod compression;
pub mod num_utils;
pub mod stopwatch;
pub mod humanize;
pub mod structures;
pub mod color;
pub mod streams;
pub mod vfs;