//! Combustion Plugin crate
//!
//! This crate defines the API and utilities for interacting with engine plugins.
//!
//! The idea so far is to use an Actor-Model pattern for plugins, where the engine provides the "model" and the plugins are the actors,
//! modifying or creating data. Though that may be subject to change in the future, it does allow some plugins to be run concurrently.

#![feature(proc_macro)]

extern crate libloading;
#[macro_use]
extern crate lazy_static;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate semver;

pub const COMBUSTION_PLUGIN_CARGO_TOML: &'static str = include_str!("../Cargo.toml");

pub mod version;