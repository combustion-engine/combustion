use ::error::WindowResult;

/// Defines methods for thread-safe render contexts
pub trait RenderContext {
    /// Make this context the current active one
    fn make_current(&mut self) -> WindowResult<()>;
    /// For double-buffered render contexts, swap the buffers front to back
    fn swap_buffers(&mut self) -> WindowResult<()>;
    /// Check if this context is the currently active context
    fn is_current(&self) -> bool;
}

pub trait WindowProvider {
    /// Show the window
    fn show(&mut self);

    /// Hide the window
    fn hide(&mut self);

    /// Set the window title
    fn set_title(&mut self, &str);

    /// Get the window position
    fn get_position(&self) -> (i32, i32);

    /// Set the window position
    fn set_position(&mut self, x: i32, y: i32);

    /// Get the window size
    fn get_size(&self) -> (u32, u32);

    /// Set the window size
    fn set_size(&mut self, width: u32, height: u32);

    /// Get the size of the window frame
    fn get_frame_size(&self) -> (u32, u32, u32, u32);

    /// Set min/max size limits for the window
    fn set_size_limits(&mut self, minwidth: u32, minheight: u32, maxwidth: u32, maxheight: u32);

    /// Consume the window and close it
    fn close(self);
}

/// Extra functionality that may not be on all providers
pub trait WindowProviderExt {
    /// Iconify or Minimize the window
    fn iconify(&mut self);
    /// Restore or un-Minimize the window
    fn restore(&mut self);
    /// Maximize the window
    fn maximize(&mut self);
    /// Focus the window
    fn focus(&mut self);

    /// Check if the window is iconified
    fn is_iconified(&self) -> bool;
    /// Check if the window is maximized
    fn is_maximized(&self) -> bool;
    /// Check if the window is focused
    fn is_focused(&self) -> bool;

    /// Check if the window is resizable
    fn is_resizable(&self) -> bool;
    /// Check if the window is visible
    fn is_visible(&self) -> bool;
    /// Check if the window is decorated
    fn is_decorated(&self) -> bool;
}

/// Standard methods for building windows
pub trait WindowBuilder {
    /// The resulting window provider type
    type Provider: WindowProvider;
    /// If the `WindowBuilder` is on top of a library window builder,
    /// then this is the underlying builder type
    type Raw;

    /// Create a new window builder
    fn new() -> Self;

    /// Create a new window builder from its raw form
    fn from_raw(builder: Self::Raw) -> Self;

    /// Consume self and return the underlying library window builder
    fn into_raw(self) -> Self::Raw;

    /// Allows easy access to the underlying builder for a single callback
    fn with_raw<F>(self, F) -> Self where F: FnOnce(Self::Raw) -> Self::Raw;

    /// Set the intended size of the window
    fn size(self, width: u32, height: u32) -> Self;

    /// Set the intended title of the window
    fn title(self, title: String) -> Self;

    /// Enable/Disable window decorations
    fn decorated(self, enabled: bool) -> Self;

    /// Enable/disable window resizing
    fn resizable(self, enabled: bool) -> Self;

    /// Enable/disable window visibility
    fn visible(self, enabled: bool) -> Self;

    /// Build the window
    fn build(self) -> WindowResult<Self::Provider>;
}