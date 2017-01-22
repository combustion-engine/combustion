#![feature(test)]

extern crate test;

extern crate glfw;
#[macro_use]
extern crate combustion_common as common;
#[macro_use]
extern crate combustion_backend as backend;
#[macro_use]
extern crate lazy_static;

use std::sync::{Arc, RwLock, Mutex, Once, ONCE_INIT};
use test::Bencher;

use backend::gl::*;
use backend::window::WindowBuilder;

use glfw::{Window, WindowHint, Context};

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
/*
#[test]
fn test_requires() {
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

}

fn test_extension_supported() {
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
*/