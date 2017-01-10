//! Fork of `rust-stopwatch` using the `time` crate's `PreciseTime`
//!
//! https://github.com/ellisonch/rust-stopwatch

use time::{Duration, PreciseTime};
use std::fmt;

/// Stopwatch structure containing the start time and accumulated elapsed time.
///
/// This is a passive stopwatch implementation. Time instances and elapsed durations are
/// modified on method calls only.
#[derive(Clone, Copy)]
pub struct Stopwatch {
    start_time: Option<PreciseTime>,
    elapsed: Duration
}

impl Default for Stopwatch {
    fn default() -> Stopwatch {
        Stopwatch::new()
    }
}

impl fmt::Display for Stopwatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}ms", self.elapsed_ms());
    }
}

impl Stopwatch {
    /// Create a new paused `Stopwatch`
    pub fn new() -> Stopwatch {
        Stopwatch { start_time: None, elapsed: Duration::seconds(0) }
    }

    /// Start the `Stopwatch`
    #[inline(always)]
    pub fn start(&mut self) {
        self.start_time = Some(PreciseTime::now());
    }

    /// Stop the `Stopwatch` and increment the elapsed time.
    #[inline(always)]
    pub fn stop(&mut self) {
        self.elapsed = self.elapsed();
        self.start_time = None;
    }

    /// Reset the `Stopwatch` to a paused state with no elapsed time.
    #[inline(always)]
    pub fn reset(&mut self) {
        self.start_time = None;
        self.elapsed = Duration::seconds(0)
    }

    /// Reset the `Stopwatch` and immediately start it again.
    #[inline(always)]
    pub fn restart(&mut self) {
        self.reset();
        self.start();
    }

    /// Returns `true` if the `Stopwatch` is active.
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        self.start_time.is_some()
    }

    /// Returns the elapsed duration.
    #[inline(always)]
    pub fn elapsed(&self) -> Duration {
        match self.start_time {
            Some(time) => time.to(PreciseTime::now()) + self.elapsed,
            None => self.elapsed
        }
    }

    /// Return the elapsed duration as milliseconds.
    #[inline(always)]
    pub fn elapsed_ms(&self) -> i64 {
        self.elapsed.num_milliseconds()
    }
}