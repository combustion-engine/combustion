#![feature(macro_reexport, proc_macro)]

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
extern crate backtrace;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod macros;

pub mod preprocessor;
pub mod utils;
pub mod structures;
pub mod log;
pub mod bt;
pub mod error;
pub mod color;