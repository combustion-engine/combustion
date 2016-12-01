use std::mem;
use std::ops::{Deref, DerefMut};

use nalgebra::Matrix4;

use ::components;

pub static RENDER_QUEUE_SIZE: usize = 256;

pub struct RenderItem {
    pub buffer: components::gpu_buffer::LazyBufferSync,
    pub transform: Matrix4<f32>,
    pub inverse: Option<Matrix4<f32>>
}

unsafe impl Send for RenderItem {}

unsafe impl Sync for RenderItem {}

pub struct Resource {
    pub queue: Vec<RenderItem>
}

impl Default for Resource {
    #[inline(always)]
    fn default() -> Resource { Resource::new() }
}

impl Resource {
    pub fn new() -> Resource {
        Resource { queue: Vec::with_capacity(RENDER_QUEUE_SIZE) }
    }

    pub fn swap(&mut self, mut other: &mut Vec<RenderItem>) {
        mem::swap(&mut self.queue, other);
    }
}

impl Deref for Resource {
    type Target = Vec<RenderItem>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target { &self.queue }
}

impl DerefMut for Resource {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.queue }
}