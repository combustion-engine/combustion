//! I/O Stream structures and utilities

pub mod definitions;
pub mod utils;
pub mod lazy_buffer;
pub mod read_only;

pub use self::definitions::{Stream, BoxedStream};
pub use self::lazy_buffer::LazyBuffer;
pub use self::read_only::ReadOnlyStream;