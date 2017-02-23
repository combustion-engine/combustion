//! GLFW Window Provider and Builder implementations

use std::sync::{mpsc, Arc};

use libc::c_void;

pub use glfw;

use glfw::{Context, Glfw, WindowHint, WindowMode, OpenGlProfileHint};

use ::error::{WindowError, WindowResult};
use ::provider::{WindowBuilder, WindowProvider, WindowProviderExt, RenderContext, GetProcAddressProvider};

/// `WindowProvider` for GLFW windows. It includes the underlying GLFW handle, the window itself,
/// and the event receiver.
///
/// This type should probably be wrapped in a `Arc<RwLock<GlfwWindowProvider>>` if you intend to use
/// it on multiple threads.
///
/// The `event_receiver` member is thread-safe and in an `Arc`,
/// so it can be passed around however you want
pub struct GlfwWindowProvider {
    /// Glfw instance
    pub glfw: glfw::Glfw,
    /// Window instance
    pub window: glfw::Window,
    /// Window event receiver
    pub event_receiver: Arc<mpsc::Receiver<(f64, glfw::WindowEvent)>>,
}

impl GetProcAddressProvider for GlfwWindowProvider {
    unsafe fn get_proc_address(&mut self, name: &str) -> WindowResult<*const c_void> {
        let proc_address = self.window.get_proc_address(name) as *const c_void;

        if proc_address.is_null() {
            throw!(WindowError::ProcAddressNotFound);
        } else {
            Ok(proc_address)
        }
    }
}

impl GlfwWindowProvider {
    /// Create a `RenderContext` compatible render context for the window
    pub fn render_context(&mut self) -> impl RenderContext {
        self.window.render_context()
    }

    /// Flush messages to an iterator
    ///
    /// Note that modifying the provider while the iterator lifetime is active isn't possible,
    /// so using `glfw::flush_messages(&*provider.event_receiver)` directly might be required.
    pub fn flush_messages(&self) -> glfw::FlushedMessages<(f64, glfw::WindowEvent)> {
        glfw::flush_messages(&*self.event_receiver)
    }
}

impl<T> RenderContext for T where T: Context {
    fn make_current(&mut self) -> WindowResult<()> {
        <Self as Context>::make_current(self);

        Ok(())
    }

    fn swap_buffers(&mut self) -> WindowResult<()> {
        <Self as Context>::swap_buffers(self);

        Ok(())
    }

    fn is_current(&self) -> bool {
        <Self as Context>::is_current(self)
    }
}

impl WindowProvider for GlfwWindowProvider {
    #[inline(always)]
    fn show(&mut self) {
        self.window.show()
    }

    #[inline(always)]
    fn hide(&mut self) {
        self.window.hide()
    }

    #[inline(always)]
    fn set_title(&mut self, title: &str) {
        self.window.set_title(title)
    }

    #[inline(always)]
    fn get_position(&self) -> (i32, i32) {
        self.window.get_pos()
    }

    #[inline(always)]
    fn set_position(&mut self, x: i32, y: i32) {
        self.window.set_pos(x, y)
    }

    #[inline(always)]
    fn get_size(&self) -> (u32, u32) {
        let (w, h) = self.window.get_size();

        (w as u32, h as u32)
    }

    #[inline(always)]
    fn set_size(&mut self, width: u32, height: u32) {
        self.window.set_size(width as i32, height as i32)
    }

    #[inline(always)]
    fn get_frame_size(&self) -> (u32, u32, u32, u32) {
        let (x1, y1, x2, y2) = self.window.get_frame_size();

        (x1 as u32, y1 as u32, x2 as u32, y2 as u32)
    }

    #[inline(always)]
    fn set_size_limits(&mut self, minwidth: u32, minheight: u32, maxwidth: u32, maxheight: u32) {
        self.window.set_size_limits(minwidth, minheight, maxwidth, maxheight)
    }

    #[inline(always)]
    fn close(self) {
        self.window.close()
    }
}

impl WindowProviderExt for GlfwWindowProvider {
    #[inline(always)]
    fn iconify(&mut self) { self.window.iconify() }
    #[inline(always)]
    fn restore(&mut self) { self.window.restore() }
    #[inline(always)]
    fn maximize(&mut self) { self.window.maximize() }
    #[inline(always)]
    fn focus(&mut self) { self.window.focus() }

    #[inline(always)]
    fn is_iconified(&self) -> bool { self.window.is_iconified() }
    #[inline(always)]
    fn is_maximized(&self) -> bool { self.window.is_maximized() }
    #[inline(always)]
    fn is_focused(&self) -> bool { self.window.is_focused() }

    #[inline(always)]
    fn is_resizable(&self) -> bool { self.window.is_resizable() }
    #[inline(always)]
    fn is_visible(&self) -> bool { self.window.is_visible() }
    #[inline(always)]
    fn is_decorated(&self) -> bool { self.window.is_decorated() }
}

/// Builder for GLFW windows
pub struct GlfwWindowBuilder<'monitor> {
    glfw: Glfw,
    size: Option<(u32, u32)>,
    aspect_ratio: Option<(u32, u32)>,
    title: Option<String>,
    mode: Option<WindowMode<'monitor>>,
    common_hints: Vec<WindowHint>,
    try_hints: Vec<Vec<WindowHint>>,
    set_all_polling: bool,
}

impl<'monitor> GlfwWindowBuilder<'monitor> {
    /// Create a GlfwWindowBuilder from an existing GLFW instance
    pub fn with_glfw(glfw: Glfw) -> GlfwWindowBuilder<'monitor> {
        GlfwWindowBuilder {
            glfw: glfw,
            size: None,
            aspect_ratio: None,
            title: None,
            mode: None,
            try_hints: vec![],
            common_hints: vec![],
            set_all_polling: false,
        }
    }
}

impl<'monitor> GlfwWindowBuilder<'monitor> {
    /// Set the desired refresh rate of the GLFW window. If set to `None`,
    /// it will try for the highest refresh rate possible
    pub fn refresh_rate(self, rate: Option<u32>) -> GlfwWindowBuilder<'monitor> {
        self.common_hints(&[WindowHint::RefreshRate(rate)])
    }

    /// Adds a list of `WindowHint`s to try creating a window with.
    ///
    /// If multiple `try_hints()` calls are present, then only one of them
    /// will be applied (the first that lead to a successful window creation).
    ///
    /// This method works in combination with `common_hints()` to create
    /// a list of fallback window configurations to try initializing with.
    /// For details, see `create()`.
    pub fn try_hints(mut self, hints: &[WindowHint]) -> GlfwWindowBuilder<'monitor> {
        self.try_hints.push(hints.iter().cloned().collect());
        self
    }

    /// Sets the aspect ratio of the window.
    /// It will try to constrain the dimensions to this ratio even when resizing the window
    pub fn aspect_ratio(mut self, numer: u32, denom: u32) -> GlfwWindowBuilder<'monitor> {
        self.aspect_ratio = Some((numer, denom));
        self
    }

    /// Adds a list of `WindowHint`s for the window to be created.
    ///
    /// If multiple `common_hints()` calls are present, they will all be
    /// applied for the created window in the order they where given.
    ///
    /// This method works in combination with `try_hints()` to create
    /// a list of fallback window configurations to try initializing with.
    /// For details, see `create()`.
    pub fn common_hints(mut self, hints: &[WindowHint]) -> GlfwWindowBuilder<'monitor> {
        self.common_hints.extend(hints.iter().cloned());
        self
    }

    /// Attempt to use the most recent OpenGL context versions (and Core profile),
    /// falling back to older versions if necessary.
    pub fn try_modern_context_hints(self) -> GlfwWindowBuilder<'monitor> {
        self.try_hints(&[WindowHint::ContextVersion(4, 5), WindowHint::OpenGlProfile(OpenGlProfileHint::Core)])
            .try_hints(&[WindowHint::ContextVersion(4, 4), WindowHint::OpenGlProfile(OpenGlProfileHint::Core)])
            .try_hints(&[WindowHint::ContextVersion(4, 3), WindowHint::OpenGlProfile(OpenGlProfileHint::Core)])
            .try_hints(&[WindowHint::ContextVersion(4, 2), WindowHint::OpenGlProfile(OpenGlProfileHint::Core)])
            .try_hints(&[WindowHint::ContextVersion(4, 1), WindowHint::OpenGlProfile(OpenGlProfileHint::Core)])
            .try_hints(&[WindowHint::ContextVersion(4, 0), WindowHint::OpenGlProfile(OpenGlProfileHint::Core)])
            .try_hints(&[WindowHint::ContextVersion(3, 2), WindowHint::OpenGlProfile(OpenGlProfileHint::Core)])
            .try_hints(&[WindowHint::ContextVersion(3, 1)])
            .try_hints(&[WindowHint::ContextVersion(3, 1)])
            .try_hints(&[WindowHint::ContextVersion(3, 0)])
            .try_hints(&[WindowHint::ContextVersion(3, 0)])
            .try_hints(&[WindowHint::ContextVersion(2, 1)])
            .try_hints(&[WindowHint::ContextVersion(2, 0)])
    }

    /// Automatically begin polling for all window events
    pub fn set_all_polling(mut self, enable: bool) -> GlfwWindowBuilder<'monitor> {
        self.set_all_polling = enable;
        self
    }

    /// Tell the OpenGL context that it can expect no errors from your program
    pub fn no_error(self) -> GlfwWindowBuilder<'monitor> {
        self.common_hints(&[WindowHint::ContextNoError(true)])
    }
}

impl<'monitor> WindowBuilder for GlfwWindowBuilder<'monitor> {
    type Provider = GlfwWindowProvider;
    type Raw = ();

    fn new() -> Self {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        GlfwWindowBuilder::with_glfw(glfw)
    }

    fn from_raw(_: ()) -> Self { Self::new() }

    fn into_raw(self) -> () { () }

    fn with_raw<F>(self, _: F) -> Self where F: FnOnce(Self::Raw) -> Self::Raw { self }

    fn size(self, width: u32, height: u32) -> Self {
        GlfwWindowBuilder { size: Some((width, height)), ..self }
    }

    fn title(self, title: String) -> Self {
        GlfwWindowBuilder { title: Some(title), ..self }
    }

    fn decorated(mut self, enabled: bool) -> Self {
        self.common_hints.push(WindowHint::Decorated(enabled));
        self
    }

    fn resizable(mut self, enabled: bool) -> Self {
        self.common_hints.push(WindowHint::Resizable(enabled));
        self
    }

    fn visible(mut self, enabled: bool) -> Self {
        self.common_hints.push(WindowHint::Visible(enabled));
        self
    }

    fn build(self) -> WindowResult<Self::Provider> {
        let GlfwWindowBuilder { mut glfw, common_hints, try_hints, size, aspect_ratio, title, mode, set_all_polling } = self;

        let (width, height) = size.unwrap_or((640, 480));
        let title = title.unwrap_or_else(|| String::from("Untitled Window"));
        let mode = mode.unwrap_or(WindowMode::Windowed);

        #[cfg(target = "macos")]
        glfw.window_hint(WindowHint::OpenGlForwardCompat(true));

        for setup in &try_hints {
            glfw.default_window_hints();

            for hint in &common_hints {
                glfw.window_hint(*hint);
            }

            for hint in setup {
                glfw.window_hint(*hint);
            }

            if let Some((mut window, event_receiver)) = glfw.create_window(width, height, title.as_str(), mode) {
                info!("Created GLFW window with GL context hints {:?} and {:?}", common_hints, setup);

                if let Some((numerator, denominator)) = aspect_ratio {
                    window.set_aspect_ratio(numerator, denominator);
                }

                if set_all_polling {
                    window.set_all_polling(true);
                }

                return Ok(GlfwWindowProvider {
                    glfw: glfw,
                    window: window,
                    event_receiver: Arc::new(event_receiver),
                })
            } else {
                debug!("Couldn't create a context for hints {:?} and {:?}", common_hints, setup);
            }
        }

        throw!(glfw::Error::NoWindowContext);
    }
}