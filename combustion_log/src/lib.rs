//! Logging tools for Combustion

#![deny(missing_docs)]

#[macro_use]
extern crate slog;
extern crate slog_term;

#[macro_use]
extern crate slog_scope;
extern crate slog_atomic;

extern crate chrono;
extern crate tinyfiledialogs;

#[macro_use]
extern crate trace_error;

pub mod error;
pub mod log;
pub mod ext;
