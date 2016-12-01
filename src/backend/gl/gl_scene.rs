use super::bindings::types::*;
use super::bindings::*;

use std::mem;
use std::ptr;

use super::gl_error::*;

pub struct GLScene {
    pub gamma: f32
}