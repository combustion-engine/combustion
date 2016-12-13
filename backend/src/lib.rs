//! Combustion backend
//!
//! This crate contains most graphics API specific code.
//!
//! Though most of it just focuses on OpenGL, support for Vulkan and DX11 are planned... eventually.

#![allow(unused_imports, unknown_lints)]
#![feature(proc_macro)]

#[macro_use]
extern crate enum_primitive;
#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate nalgebra;
extern crate num_traits;
extern crate image;
extern crate palette;
extern crate regex;
extern crate vec_map;
extern crate serde;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate combustion_common;

pub mod gl;
pub mod vulkan;
pub mod dx11;
pub mod generic;

pub mod command;