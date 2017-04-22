//! Provides algorithms and utilities for drawing things on images, usually in the form of graphs.
//!
//! Currently, three line algorithms are supported:
//!
//! * Bresenham's Algorithm
//! * Xiaolin Wu's algorithm
//! * Xiaolin Wu's algorithm where at every pixel a dot is plotted via a Gaussian probability distribution function,
//!     thereby forming a smooth, antialiased thick line.
//!
//! Supported shapes:
//!
//! * Circles
//! * Ellipses
//! * Rectangles
//!
//! Where the circles and ellipsis are drawn using the midpoint circle algorithm, so they are very nice.
//!
//! Supported equations:
//!
//! * Linear equations (e.g., `y = 2x + 3`)
//! * Parametric equations
//! * Polar equations
//! * Planar equations (WIP)
//!
//! Other supported shapes/curves:
//!
//! * Bezier curves (up to degree 3 for now)
//! * Automatic axis drawing based on function domains
//!
//! Future work:
//!
//! * Text on graphs
//! * Make it easier to plot directly to pixels for some things,
//!     rather than using function domains

#![feature(range_contains)]

extern crate combustion_common as common;

pub mod stat;
pub mod graph;
pub mod bezier;
pub mod geometry;