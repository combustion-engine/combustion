//! Defines traits and types for use with streams

use std::io::prelude::*;
use std::fmt::Debug;

/// Some type with `Debug + Read + Seek + Write + 'static`
///
/// Automatically implemented for types that satisfy those above requirements.
pub trait Stream: Debug + Read + Seek + Write + 'static {}

impl<T> Stream for T where T: Debug + Read + Seek + Write + 'static {}

/// Simple alias for a `Box<String>` to ease in use
pub type BoxedStream = Box<Stream>;