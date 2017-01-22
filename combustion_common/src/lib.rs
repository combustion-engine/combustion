//! Common stuff for the Combustion engine.
//!
//! "Stuff", in this case, being assorted tools, data structures,
//! and macros that don't belong in any single crate yet are used often in the engine.

#![feature(macro_reexport, associated_type_defaults)]
#![deny(missing_docs)]
#![allow(unknown_lints, inline_always)]

extern crate regex;
extern crate vec_map;
extern crate num_traits;
#[macro_use]
extern crate lazy_static;

#[macro_use(o, slog_log, slog_trace, slog_debug, slog_info, slog_warn, slog_error)]
#[macro_reexport(o, slog_log, slog_trace, slog_debug, slog_info, slog_warn, slog_error)]
extern crate slog;
extern crate slog_term;
extern crate slog_stream;
#[macro_use]
#[macro_reexport(crit, error, warn, info, debug, trace)]
extern crate slog_scope;
extern crate slog_atomic;

extern crate nalgebra;
extern crate tinyfiledialogs;
extern crate palette;
extern crate time;
extern crate chrono;
extern crate statrs;
extern crate void;
extern crate backtrace as bt;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate lz4;
extern crate rand;

#[macro_use]
#[macro_reexport(backtrace, backtrace_noheader, throw, try_throw, try_rethrow)]
pub extern crate trace_error;

pub use trace_error as error;

pub mod macros;
pub mod compression;
pub mod preprocessor;
pub mod num_utils;
pub mod stopwatch;
pub mod humanize;
pub mod structures;
pub mod log;
pub mod color;
pub mod ext;