use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::sync::mpsc::Receiver;

use glfw::{Glfw, WindowMode, WindowHint, Window, WindowEvent, OpenGlProfileHint};

pub struct WindowBuilder<'title, 'monitor> {
    glfw: Glfw,
    size: Option<(u32, u32)>,
    aspect_ratio: Option<(u32, u32)>,
    title: Option<&'title str>,
    mode: Option<WindowMode<'monitor>>,
    common_hints: Vec<WindowHint>,
    try_hints: Vec<Vec<WindowHint>>,
    set_all_polling: bool,
}

impl<'title, 'monitor> WindowBuilder<'title, 'monitor> {
    /// Creates a new `WindowBuilder` for a existing `Glfw` value
    pub fn new(glfw: Glfw) -> WindowBuilder<'title, 'monitor> {
        WindowBuilder {
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

impl<'title, 'monitor> WindowBuilder<'title, 'monitor> {
    /// Sets the size of the GLFW window to `width x height`.
    /// Defaults to `640 x 480` if not given.
    pub fn size(mut self, width: u32, height: u32) -> WindowBuilder<'title, 'monitor> {
        self.size = Some((width, height));
        self
    }

    /// Sets the title of the GLFW window to `title`.
    /// Defaults to `"GLFW Window"` if not given.
    pub fn title(mut self, title: &'title str) -> WindowBuilder<'title, 'monitor> {
        self.title = Some(title);
        self
    }

    /// Sets the mode of the GLFW window to `mode`.
    /// Defaults to `Windowed` if not given.
    pub fn mode(mut self, mode: WindowMode<'monitor>) -> WindowBuilder<'title, 'monitor> {
        self.mode = Some(mode);
        self
    }

    /// Tell the OpenGL context that it can expect no errors from your program
    pub fn no_error(self) -> WindowBuilder<'title, 'monitor> {
        self.common_hints(&[WindowHint::ContextNoError(true)])
    }

    /// Set the desired refresh rate of the GLFW window. If set to `None`,
    /// it will try for the highest refresh rate possible
    pub fn refresh_rate(self, rate: Option<u32>) -> WindowBuilder<'title, 'monitor> {
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
    pub fn try_hints(mut self, hints: &[WindowHint]) -> WindowBuilder<'title, 'monitor> {
        self.try_hints.push(hints.iter().map(|&x| x).collect());
        self
    }

    /// Sets the aspect ratio of the window.
    /// It will try to constrain the dimensions to this ratio even when resizing the window
    pub fn aspect_ratio(mut self, numer: u32, denom: u32) -> WindowBuilder<'title, 'monitor> {
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
    pub fn common_hints(mut self, hints: &[WindowHint]) -> WindowBuilder<'title, 'monitor> {
        self.common_hints.extend(hints.iter().map(|&x| x));
        self
    }

    pub fn try_modern_context_hints(self) -> WindowBuilder<'title, 'monitor> {
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

    pub fn set_all_polling(mut self, enable: bool) -> WindowBuilder<'title, 'monitor> {
        self.set_all_polling = enable;
        self
    }

    /// Try to create the window.
    ///
    /// This method tries each of the possible window hints given
    /// with `try_hints()` in order, returning the first one that succeeds.
    ///
    /// In order for that to work, it has to disable the `Glfw` error callback,
    /// so you'll have to rebind it afterwards.
    ///
    /// For every set of window hints given with a `try_hints()`, this method
    ///
    /// - Applies the window hints of all `common_hints()` given.
    /// - Applies the window hints of the current `try_hints()`.
    /// - Tries to call `glfw.create_window()` with the given arguments
    ///   (or default values).
    /// - Returns on successful window creation.
    pub fn create(self) -> Option<(Arc<RwLock<Window>>, Receiver<(f64, WindowEvent)>)> {
        let WindowBuilder { mut glfw, common_hints, try_hints, size, aspect_ratio, title, mode, set_all_polling } = self;

        let (width, height) = size.unwrap_or((640, 480));
        let title = title.unwrap_or("Untitled Window");
        let mode = mode.unwrap_or(WindowMode::Windowed);

        #[cfg(target = "macos")]
        glfw.window_hint(WindowHint::OpenGlForwardCompat(true));

        for setup in try_hints.iter() {
            glfw.default_window_hints();

            for hint in common_hints.iter() {
                glfw.window_hint(*hint);
            }

            for hint in setup.iter() {
                glfw.window_hint(*hint);
            }

            if let Some((mut window, events)) = glfw.create_window(width, height, title, mode) {
                info!("Created GLFW window with GL context hints {:?} and {:?}", common_hints, setup);

                if let Some((numer, denom)) = aspect_ratio {
                    window.set_aspect_ratio(numer, denom);
                }

                if set_all_polling {
                    window.set_all_polling(true);
                }

                return Some((Arc::new(RwLock::new(window)), events));
            } else {
                debug!("Couldn't create a context for hints {:?} and {:?}", common_hints, setup);
            }
        }
        None
    }
}
