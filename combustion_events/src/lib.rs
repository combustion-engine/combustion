//! Event handling

#![feature(conservative_impl_trait)]
#![deny(missing_docs)]

extern crate fnv;

#[cfg(feature = "parallel")]
extern crate futures;
#[cfg(feature = "parallel")]
extern crate futures_cpupool;

#[macro_use]
extern crate combustion_common as common;

pub mod event_emitter;