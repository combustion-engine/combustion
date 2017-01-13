//! Utilities for parallel programming

use futures::{Future, IntoFuture};

use crossbeam::sync::MsQueue;
