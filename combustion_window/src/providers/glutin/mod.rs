use std::ops::{Deref, DerefMut};

pub use glutin;

use ::error::WindowError;
use ::provider::{WindowBuilder, WindowProvider};

pub struct GlutinWindowProvider {
    window: glutin::Window,
}

impl Deref for GlutinWindowProvider {
    type Target = glutin::Window;

    fn deref(&self) -> &glutin::Window {
        &self.window
    }
}

impl DerefMut for GlutinWindowProvider {
    fn deref_mut(&mut self) -> &mut glutin::Window {
        &mut self.window
    }
}

impl WindowProvider for GlutinWindowProvider {
    #[inline(always)]
    fn show(&self) { self.window.show() }

    #[inline(always)]
    fn hide(&self) { self.window.hide() }
}

pub struct GlutinWindowBuilder<'a> {
    builder: glutin::WindowBuilder<'a>,
}

impl<'a> WindowBuilder for GlutinWindowBuilder<'a> {
    type Provider = GlutinWindowProvider;
    type Raw = glutin::WindowBuilder<'a>;

    fn new() -> GlutinWindowBuilder<'a> {
        GlutinWindowBuilder { builder: glutin::WindowBuilder::new() }
    }

    fn from_raw(builder: glutin::WindowBuilder<'a>) -> Self {
        GlutinWindowBuilder { builder: builder }
    }

    fn into_raw(self) -> glutin::WindowBuilder<'a> {
        self.builder
    }

    fn with_raw<F>(self, f: F) -> Self where F: FnOnce(Self::Raw) -> Self::Raw {
        GlutinWindowBuilder { builder: f(self.builder) }
    }

    fn size(self, width: u32, height: u32) -> Self {
        GlutinWindowBuilder { builder: self.builder.with_dimensions(width, height) }
    }

    fn title(self, title: &str) -> Self {
        GlutinWindowBuilder { builder: self.builder.with_title(title) }
    }

    fn build(self) -> Result<Self::Provider, WindowError> {
        let window = self.builder.build()?;

        Ok(GlutinWindowProvider { window: window })
    }
}