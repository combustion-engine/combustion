#![feature(conservative_impl_trait)]

#[cfg(feature = "glfw")]
pub extern crate glfw;

#[cfg(feature = "glutin")]
pub extern crate glutin;

#[cfg(feature = "winit")]
pub extern crate winit;

#[macro_use]
extern crate combustion_common as common;

#[macro_use]
extern crate trace_error;

pub mod error;
pub mod provider;
pub mod providers;