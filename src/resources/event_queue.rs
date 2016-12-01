use std::ops::{Deref, DerefMut};

use glfw::WindowEvent;

pub enum Event {
    WindowEvent(WindowEvent)
}

pub struct Resource {
    pub queue: Vec<Event>
}

impl Resource {
    pub fn new() -> Resource {
        Resource { queue: Vec::with_capacity(256) }
    }
}

impl Deref for Resource {
    type Target = Vec<Event>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target { &self.queue }
}

impl DerefMut for Resource {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.queue }
}