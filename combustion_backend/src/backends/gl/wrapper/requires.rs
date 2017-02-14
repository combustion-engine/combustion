use glfw::Glfw;
use fnv::FnvHashMap;

use std::mem;
use std::ptr;
use std::cell::{Cell, RefCell};
use std::sync::{Arc, RwLock};
use std::ops::Deref;

use super::error::*;

/// Small wrapper structure for a lazy global GLFW instance
#[derive(Clone, Default)]
pub struct LazyGlfwInstance {
    pub instance: Arc<RwLock<Option<Arc<RwLock<Glfw>>>>>
}

impl LazyGlfwInstance {
    /// Set the Glfw instance used for extension testing.
    ///
    /// This may only be set once
    pub fn set(&self, glfw: Arc<RwLock<Glfw>>) -> GLResult<()> {
        if let Ok(mut instance) = self.instance.write() {
            if instance.is_some() {
                throw!(GLError::AlreadyInitialized);
            }

            *instance = Some(glfw);

            Ok(())
        } else { throw!(GLError::PoisonError) }
    }

    fn get_thread_local(&self) -> GLResult<Option<Glfw>> {
        LOCAL_GLFW_INSTANCE.with(|local_instance| {
            let instance = local_instance.get();

            if instance.is_some() {
                Ok(instance)
            } else if let Ok(global_instance_lock) = self.instance.read() {
                if let Some(global_instance) = global_instance_lock.as_ref() {
                    if let Ok(glfw_lock) = global_instance.read() {
                        let instance = Some(*glfw_lock);

                        local_instance.set(instance);

                        Ok(instance)
                    } else { throw!(GLError::PoisonError) }
                } else {
                    Ok(None)
                }
            } else { throw!(GLError::PoisonError) }
        })
    }
}

unsafe impl Send for LazyGlfwInstance {}

unsafe impl Sync for LazyGlfwInstance {}

// This is here to speed up access to these from any single thread, since glfwExtensionSupported is thread-safe
thread_local! {
    static LOCAL_GLFW_INSTANCE: Cell<Option<Glfw>> = Cell::new(None);
}

lazy_static! {
    pub static ref GLFW_INSTANCE: LazyGlfwInstance = Default::default();
    static ref GL_EXTENSIONS: Arc<RwLock<FnvHashMap<&'static str, bool>>> = Arc::default();
}

/// Check if an OpenGL extension exists.
///
/// Results are cached in a global hash table (using FNV hash algorithm for fast lookup),
/// and there is a thread-local cache for the last checked extension, making repeated calls for the same
/// extension effectively no-cost.
///
/// This function only succeeds if there is a valid OpenGL context and the Glfw instance
/// has been set with `GLFW_INSTANCE.set(...)`
///
/// Performance on an i5-4690K:
///
/// ```ignore
/// test gl::requires::test::bench_raw                                   ...
///     bench:         408 ns/iter (+/- 13)
/// test gl::requires::test::bench_requires_with_last_check              ...
///     bench:           8 ns/iter (+/- 0)
/// test gl::requires::test::bench_requires_without_last_check_times_two ...
///     bench:         119 ns/iter (+/- 4)
/// ```
///
/// Where `bench_raw` is `glfw.extension_present`
///
/// The `bench_requires_without_last_check_times_two` bench checks for two extensions each iteration to
/// defeat the last check cache, so its time is doubled.
///
/// E.g.,
///
/// ```ignore
/// let has_debug = try_rethrow!(check_extension("GL_ARB_debug_output"));
/// ```
pub fn extension_supported(extension: &'static str) -> GLResult<bool> {
    thread_local! {
        static LAST_CHECKED: Cell<Option<(&'static str, bool)>> = Cell::new(None);
    }

    match LAST_CHECKED.with(|last_checked| last_checked.get()) {
        Some(last_checked) if last_checked.0 == extension => {
            return Ok(last_checked.1);
        }
        _ => {}
    }

    if let Ok(read_extensions_map) = GL_EXTENSIONS.read() {
        if let Some(supported) = read_extensions_map.get(extension) {
            LAST_CHECKED.with(|last_checked| last_checked.set(Some((extension, *supported))));

            return Ok(*supported);
        }
        // Drop the read guard so we can write to GL_EXTENSIONS
        drop(read_extensions_map);

        if let Ok(mut write_extensions_map) = GL_EXTENSIONS.write() {
            // Get the local instance really quick without dealing with the higher RwLocks
            if let Some(glfw) = try_rethrow!(GLFW_INSTANCE.get_thread_local()) {
                let supported = glfw.extension_supported(extension);

                write_extensions_map.insert(extension, supported);

                LAST_CHECKED.with(|last_checked| last_checked.set(Some((extension, supported))));

                return Ok(supported);
            } else { throw!(GLError::InvalidInstance) }
        } else { throw!(GLError::PoisonError) }
    } else { throw!(GLError::PoisonError) }
}

#[macro_export]
macro_rules! gl_requires {
    ($($extension:expr),+) => {{
        #[inline(never)]
        fn check_extensions() -> GLResult<()> {
            $(
                if ! try_rethrow!($crate::gl::requires::extension_supported($extension)) {
                    throw!($crate::gl::GLError::UnsupportedExtension($extension.into()));
                }
            )+

            Ok(())
        }

        thread_local! {
            static OPENGL_REQUIRES_EXTENSION: Cell<Option<()>> = Cell::new(None);
        }

        OPENGL_REQUIRES_EXTENSION.with(|requires| -> GLResult<()> {
            match requires.get() {
                None => { check_extensions() },
                _ => { Ok(()) }
            }
        })
    }};

    ($($extensions:expr),+,) => (gl_requires!($($extensions),+));
}