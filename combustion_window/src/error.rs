//! Error handling

use std::io;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

use trace_error::TraceResult;

/// Traceable result for for `WindowError`s
pub type WindowResult<T> = TraceResult<T, WindowError>;

#[cfg(feature = "glfw")]
use glfw;

#[cfg(feature = "glutin")]
use glutin;

#[cfg(feature = "winit")]
use winit;

/// Glfw errors wrapper
#[cfg(feature = "glfw")]
pub mod glfw_error {
    use std::error::Error;
    use std::fmt::{Display, Formatter, Result as FmtResult};

    use glfw;

    /// Glfw errors wrapper
    #[derive(Debug)]
    pub enum GlfwError {
        /// Generic GLFW Error
        GenericError(glfw::Error),
        /// GLFW Initialization Error
        InitializationError(glfw::InitError),
    }

    impl Display for GlfwError {
        fn fmt(&self, f: &mut Formatter) -> FmtResult {
            match *self {
                GlfwError::GenericError(ref err) => Display::fmt(err, f),
                GlfwError::InitializationError(ref err) => Display::fmt(err, f),
            }
        }
    }

    impl Error for GlfwError {
        fn description(&self) -> &str {
            match *self {
                GlfwError::GenericError(ref err) => err.description(),
                GlfwError::InitializationError(ref err) => err.description(),
            }
        }
    }

    impl From<glfw::Error> for GlfwError {
        fn from(err: glfw::Error) -> GlfwError {
            GlfwError::GenericError(err)
        }
    }

    impl From<glfw::InitError> for GlfwError {
        fn from(err: glfw::InitError) -> GlfwError {
            GlfwError::InitializationError(err)
        }
    }
}

/// Glutin errors wrapper
#[cfg(feature = "glutin")]
pub mod glutin_error {
    use std::error::Error;
    use std::fmt::{Display, Formatter, Result as FmtResult};

    use glutin::{CreationError, ContextError};

    /// Glutin errors wrapper
    #[derive(Debug)]
    pub enum GlutinError {
        /// Gluton `CreationError`
        CreationError(CreationError),
        /// Glutin `ContextError`
        ContextError(ContextError),
    }

    impl Display for GlutinError {
        fn fmt(&self, f: &mut Formatter) -> FmtResult {
            match *self {
                GlutinError::ContextError(ref err) => Display::fmt(err, f),
                GlutinError::CreationError(ref err) => Display::fmt(err, f),
            }
        }
    }

    impl Error for GlutinError {
        fn description(&self) -> &str {
            match *self {
                GlutinError::ContextError(ref err) => err.description(),
                GlutinError::CreationError(ref err) => err.description(),
            }
        }
    }

    impl From<CreationError> for GlutinError {
        fn from(err: CreationError) -> GlutinError {
            GlutinError::CreationError(err)
        }
    }

    impl From<ContextError> for GlutinError {
        fn from(err: ContextError) -> GlutinError {
            GlutinError::ContextError(err)
        }
    }
}

/// Error kinds that might occur when working with windows
#[derive(Debug)]
pub enum WindowError {
    /// IO Errors
    Io(io::Error),
    /// OS Errors
    OsError(String),
    /// GLFW Errors
    #[cfg(feature = "glfw")]
    GlfwError(glfw_error::GlfwError),
    /// Glutin Errors
    #[cfg(feature = "glutin")]
    GlutinError(glutin_error::GlutinError),
    /// Winit Errors
    #[cfg(feature = "winit")]
    WinitError(winit::CreationError),
}

impl Display for WindowError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str(self.description())
    }
}

impl Error for WindowError {
    fn description(&self) -> &str {
        match *self {
            WindowError::Io(ref err) => err.description(),
            WindowError::OsError(ref s) => s.as_str(),
            #[cfg(feature = "glfw")]
            WindowError::GlfwError(ref err) => err.description(),
            #[cfg(feature = "glutin")]
            WindowError::GlutinError(ref err) => err.description(),
            #[cfg(feature = "winit")]
            WindowError::WinitError(ref err) => err.description(),
        }
    }
}

#[cfg(feature = "glfw")]
impl From<glfw::Error> for WindowError {
    fn from(err: glfw::Error) -> WindowError {
        WindowError::GlfwError(err.into())
    }
}

#[cfg(feature = "glfw")]
impl From<glfw::InitError> for WindowError {
    fn from(err: glfw::InitError) -> WindowError {
        WindowError::GlfwError(err.into())
    }
}

#[cfg(feature = "glutin")]
impl From<glutin::CreationError> for WindowError {
    fn from(err: glutin::CreationError) -> WindowError {
        match err {
            glutin::CreationError::OsError(s) => WindowError::OsError(s),
            _ => WindowError::GlutinError(err.into())
        }
    }
}

#[cfg(feature = "glutin")]
impl From<glutin::ContextError> for WindowError {
    fn from(err: glutin::ContextError) -> WindowError {
        match err {
            glutin::ContextError::IoError(err) => WindowError::Io(err),
            _ => WindowError::GlutinError(err.into())
        }
    }
}

#[cfg(feature = "winit")]
impl From<winit::CreationError> for WindowError {
    fn from(err: winit::CreationError) -> WindowError {
        match err {
            winit::CreationError::OsError(s) => WindowError::OsError(s),
            _ => WindowError::WinitError(err),
        }
    }
}