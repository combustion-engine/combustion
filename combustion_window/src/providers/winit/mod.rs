use std::ops::{Deref, DerefMut};

pub use winit;

use ::error::WindowError;
use ::provider::{WindowBuilder, WindowProvider};

pub struct WinitWindowProvider {
    window: winit::Window,
}

impl Deref for WinitWindowProvider {
    type Target = winit::Window;

    fn deref(&self) -> &winit::Window {
        &self.window
    }
}

impl DerefMut for WinitWindowProvider {
    fn deref_mut(&mut self) -> &mut winit::Window {
        &mut self.window
    }
}

impl WindowProvider for WinitWindowProvider {}

pub struct WinitWindowBuilder {
    builder: winit::WindowBuilder,
}

impl WindowBuilder for WinitWindowBuilder {
    type Provider = WinitWindowProvider;
    type Raw = winit::WindowBuilder;

    fn new() -> WinitWindowBuilder {
        WinitWindowBuilder { builder: winit::WindowBuilder::new() }
    }

    fn from_raw(builder: winit::WindowBuilder) -> WinitWindowBuilder {
        WinitWindowBuilder { builder: builder }
    }

    fn into_raw(self) -> winit::WindowBuilder {
        self.builder
    }

    fn with_raw<F>(self, f: F) -> Self where F: FnOnce(Self::Raw) -> Self::Raw {
        WinitWindowBuilder { builder: f(self.builder) }
    }

    fn size(self, width: u32, height: u32) -> Self {
        WinitWindowBuilder { builder: self.builder.with_dimensions(width, height) }
    }

    fn title(self, title: &str) -> Self {
        WinitWindowBuilder { builder: self.builder.with_title(title) }
    }

    fn build(self) -> Result<Self::Provider, WindowError> {
        let window = self.builder.build()?;

        Ok(WinitWindowProvider { window: window })
    }
}