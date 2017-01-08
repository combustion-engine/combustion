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
    static ref GLFW_INSTANCE: LazyGlfwInstance = Default::default();
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

#[cfg(test)]
mod test {
    use super::super::error::*;
    use super::*;
    use test::Bencher;
    use std::sync::{Arc, RwLock, Mutex, Once, ONCE_INIT};
    use glfw::{self, Window, WindowHint, Context};
    use ::window::WindowBuilder;

    // These are just here to fill up the hash table a little bit with real data
    static TEST_EXTENSIONS: &'static [&'static str] = &[
        "GL_ARB_compressed_texture_pixel_storage",
        "GL_ARB_conservative_depth",
        "GL_ARB_ES2_compatibility",
        "GL_ARB_get_program_binary",
        "GL_ARB_explicit_uniform_location",
        "GL_ARB_internalformat_query",
        "GL_ARB_internalformat_query2",
        "GL_ARB_map_buffer_alignment",
        "GL_ARB_program_interface_query",
        "GL_ARB_separate_shader_objects",
        "GL_ARB_shading_language_420pack",
        "GL_ARB_shading_language_packing",
        "GL_ARB_texture_buffer_range",
        "GL_ARB_texture_storage",
        "GL_ARB_texture_view",
        "GL_ARB_vertex_attrib_binding",
        "GL_ARB_viewport_array",
        "GL_NV_texture_barrier",
        "WGL_ARB_extensions_string",
        "WGL_ARB_pixel_format",
        "WGL_ARB_framebuffer_sRGB",
        "GL_EXT_texture_filter_anisotropic",
        "GL_EXT_texture_compression_s3tc",
        "EXT_texture_sRGB",
        "GL_ARB_debug_output",
        "GL_KHR_debug",
        "GL_ARB_transform_feedback2",
        "GL_ARB_transform_feedback3",
        "GL_ARB_texture_buffer_object_rgb32",
        "GL_ARB_shader_precision",
        "GL_ARB_draw_buffers_blend",
        "GL_ARB_vertex_array_object",
        "GL_ARB_framebuffer_object",
        "GL_EXT_framebuffer_object",
        "GL_EXT_framebuffer_blit",
        "GL_EXT_framebuffer_multisample",
        "GL_EXT_packed_depth_stencil.",
        "GL_ARB_map_buffer_range",
        "GL_ARB_copy_buffer",
        "GL_ARB_texture_rectangle",
        "GL_ARB_color_buffer_float",
        "GL_ARB_half_float_pixel",
        "GL_ARB_sync",
        "GL_ARB_texture_rg",
        "GL_ARB_texture_compression_rgtc",
        "GL_EXT_bindable_uniform",
        "GL_EXT_draw_buffers2",
        "GL_EXT_geometry_shader4",
        "GL_EXT_gpu_shader4",
        "GL_EXT_framebuffer_sRGB",
    ];

    struct StaticWindowContainer {
        window: Arc<RwLock<Window>>,
    }

    unsafe impl Send for StaticWindowContainer {}

    unsafe impl Sync for StaticWindowContainer {}

    fn init() -> StaticWindowContainer {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Could not initialize GLFW!");

        let (window, _) = WindowBuilder::new(glfw)
            .try_modern_context_hints()
            .size(1, 1)
            .common_hints(&[
                WindowHint::Visible(false),
                WindowHint::OpenGlDebugContext(true)
            ])
            .title("test_requires")
            .create()
            .expect("Couldn't create window");

        window.write().unwrap().make_current();

        let sync_glfw = Arc::new(RwLock::new(glfw));

        GLFW_INSTANCE.set(sync_glfw).unwrap();

        StaticWindowContainer { window: window }
    }

    lazy_static! {
        static ref LOCK: Mutex<()> = Mutex::new(());
        static ref STATIC_WINDOW: StaticWindowContainer = init();
    }

    #[test]
    fn test_requires() {
        let _ = LOCK.lock().unwrap();
        STATIC_WINDOW.window.clone();

        let mut supported = 0;
        let mut unsupported = 0;

        for extension in TEST_EXTENSIONS {
            if extension_supported(extension).unwrap() {
                supported += 1;
            } else {
                unsupported += 1;
                println!("Unsupported: {}", extension);
            }
        }

        println!("Total Supported: {}", supported);
        println!("Total Unsupported: {}", unsupported);
    }

    #[bench]
    fn bench_requires_with_last_check(b: &mut Bencher) {
        let _ = LOCK.lock().unwrap();
        STATIC_WINDOW.window.clone();

        for extension in TEST_EXTENSIONS {
            extension_supported(extension).unwrap();
        }

        b.iter(|| {
            extension_supported("GL_ARB_debug_output").unwrap();
        });
    }

    #[bench]
    fn bench_requires_without_last_check_times_two(b: &mut Bencher) {
        let _ = LOCK.lock().unwrap();
        STATIC_WINDOW.window.clone();

        for extension in TEST_EXTENSIONS {
            extension_supported(extension).unwrap();
        }

        b.iter(|| {
            extension_supported("GL_ARB_debug_output").unwrap();
            extension_supported("GL_EXT_framebuffer_multisample").unwrap();
        });
    }

    #[bench]
    fn bench_raw(b: &mut Bencher) {
        let _ = LOCK.lock().unwrap();
        STATIC_WINDOW.window.clone();

        let lock = GLFW_INSTANCE.get_thread_local().unwrap().unwrap();

        let glfw: glfw::Glfw = lock;

        b.iter(|| {
            glfw.extension_supported("GL_ARB_debug_output")
        });
    }
}