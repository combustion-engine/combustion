//! Renderable component
//!
//! This component is requiered for any entity which should appear on screen

use specs;

#[derive(Clone, Copy)]
pub struct Component;

impl specs::Component for Component {
    type Storage = specs::VecStorage<Component>;
}

impl Default for Component {
    #[inline(always)]
    fn default() -> Component { Component }
}

impl Component {
    #[inline(always)]
    pub fn new() -> Component { Component }
}