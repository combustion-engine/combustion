#![allow(unknown_lints, unused_imports, dead_code, inline_always)]
#![feature(proc_macro)]

#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate time;
extern crate gl;
extern crate glfw;
extern crate nice_glfw;
extern crate nalgebra;
extern crate rusttype;
extern crate image;
extern crate palette;
extern crate num_traits;
extern crate regex;
extern crate assimp;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::os::raw::c_void;
use std::ffi::{CStr, CString};
use std::mem;
use std::ptr;
use std::str;
use std::thread;
use std::sync::mpsc;
use std::path::Path;
use std::time::Duration as StdDuration;
use std::f32::consts::PI;

use std::mem::size_of;

use time::{Duration, PreciseTime};

use glfw::{Glfw, Action, Context, Key, WindowHint, WindowEvent};

use image::{DynamicImage, ConvertBuffer};
use image::imageops::{resize, Nearest};

use nalgebra::*;

use assimp::*;
use assimp::postprocess as pp;

//static ICON: &'static [u8] = include_bytes!("./../icon.png");

pub mod bindings;
pub mod extra_bindings;

pub mod preprocessor;

#[macro_use]
pub mod gl_error;
pub mod gl_uniform;
pub mod gl_shader;
pub mod gl_shader_program;
pub mod gl_buffer;
pub mod gl_color;
pub mod gl_light;
pub mod gl_material;
pub mod gl_texture;
pub mod gl_object;
pub mod gl_scene;

pub mod fullscreen;

use bindings::types::*;
use bindings::*;
use extra_bindings::*;

use gl_error::*;
use gl_shader::*;
use gl_shader_program::*;
use gl_light::*;
use gl_material::*;
use gl_texture::*;
use gl_object::*;

enum RenderSignal {
    Stop,
    Pause,
    Resume,
    ViewportResize(i32, i32),
    CursorPosition(i32, i32),
    Movement(i32, i32),
    SetFOV(f32),
    SmoothnessAdjust(f32),
    AlbedoAdjust(f32),
    AdjustMetallic(f32),
    ToggleRotation
}

fn main() {
    let mut glfw: Glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    println!("Vulkan Supported: {}", glfw.vulkan_supported());

    //Create the window
    let (mut window, events): (glfw::Window, _) = nice_glfw::WindowBuilder::new(&mut glfw)
        .try_modern_context_hints()
        .size(1280, 720)
        .aspect_ratio(16, 9)
        .common_hints(&[
            WindowHint::Visible(true),
            //WindowHint::Samples(Some(4)),
            WindowHint::DoubleBuffer(true),
        ])
        .title("OpenGL Testing")
        .create()
        .expect("Couldn't create window");

    //Enable keyboard interactivity
    window.set_all_polling(true);

    //Load all the OpenGL function pointers
    bindings::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    extra_bindings::load_extras_with(|symbol| window.get_proc_address(symbol) as *const _);

    //Create signalling channel between main thread and render thread
    let (tx, rx) = mpsc::channel();

    // Disconnect current context
    glfw::make_context_current(None);

    //Get render context from the window that can be moved between threads
    let context = window.render_context();

    //spawn new thread to handle all the rendering
    thread::spawn(move || {
        println!("Rendering thread started...");

        //Active context on current thread
        glfw::make_context_current(Some(&context));

        //Start rendering
        render_task(context, rx);
    });

    let mut fullscreen = fullscreen::Toggle::new();

    //Listen for events on the main thread
    while !window.should_close() {
        //Instead of polling for events, block and wait for them
        glfw.wait_events();

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                }
                WindowEvent::FramebufferSize(width, height) | WindowEvent::Size(width, height) => {
                    tx.send(RenderSignal::ViewportResize(width, height)).unwrap();
                }
                WindowEvent::Key(Key::F11, _, Action::Press, _) => {
                    fullscreen.toggle(&mut glfw, &mut window);
                }
                WindowEvent::Key(Key::M, _, Action::Press, _) => {
                    tx.send(RenderSignal::ToggleRotation).unwrap();
                }
                WindowEvent::Key(key, _, action @ Action::Press, _) |
                WindowEvent::Key(key, _, action @ Action::Repeat, _) => {
                    match key {
                        Key::Down | Key::Up => {
                            let inc: f32 = if action == Action::Press { 5.0 } else { 1.0 };
                            let mul: f32 = if key == Key::Down { -1.0 } else { 1.0 };

                            tx.send(RenderSignal::SetFOV(inc * mul)).unwrap();
                        }
                        Key::P | Key::O => {
                            let inc: f32 = if action == Action::Press { 0.05 } else { 0.01 };
                            let mul: f32 = if key == Key::P { 1.0 } else { -1.0 };

                            tx.send(RenderSignal::SmoothnessAdjust(inc * mul)).unwrap();
                        }
                        Key::L | Key::K => {
                            let inc: f32 = if action == Action::Press { 0.1 } else { 0.01 };
                            let mul: f32 = if key == Key::K { 1.0 } else { -1.0 };

                            tx.send(RenderSignal::AlbedoAdjust(inc * mul)).unwrap();
                        }
                        Key::Q | Key::W => {
                            let inc: f32 = if action == Action::Press { 0.1 } else { 0.01 };
                            let mul: f32 = if key == Key::W { 1.0 } else { -1.0 };

                            tx.send(RenderSignal::AdjustMetallic(inc * mul)).unwrap();
                        }
                        _ => {
                            /*Add other Press|Repeat keys here*/
                        }
                    }
                }
                WindowEvent::Iconify(iconified) if iconified => {
                    tx.send(RenderSignal::Pause).unwrap();
                }
                WindowEvent::Focus(focus) if focus => {
                    tx.send(RenderSignal::Resume).unwrap();
                }
                _ => {}
            }
        }
    }

    //Signal the render thread to close
    tx.send(RenderSignal::Stop).expect("Failed to signal render task.");
}

fn render_task(mut context: glfw::RenderContext, rx: mpsc::Receiver<RenderSignal>) {
    ////////////////////////////////

    let lighting_vs = GLShader::from_file("shaders/brdf.vert", GLShaderVariant::VertexShader).unwrap();
    let lighting_fs = GLShader::from_file("shaders/brdf.frag", GLShaderVariant::FragmentShader).unwrap();

    let skybox_vs = GLShader::from_file("shaders/skybox.vert", GLShaderVariant::VertexShader).unwrap();
    let skybox_fs = GLShader::from_file("shaders/skybox.frag", GLShaderVariant::FragmentShader).unwrap();

    let mut lighting: GLShaderProgram = GLShaderProgramBuilder::new().unwrap()
                                                                     .attach_shader(lighting_vs).unwrap()
                                                                     .attach_shader(lighting_fs).unwrap()
                                                                     .link().unwrap()
                                                                     .finish();

    let mut skybox: GLShaderProgram = GLShaderProgramBuilder::new().unwrap()
                                                                   .attach_shader(skybox_vs).unwrap()
                                                                   .attach_shader(skybox_fs).unwrap()
                                                                   .link().unwrap()
                                                                   .finish();

    ////////////////////////////////

    let camera_pos: Point3<f32> = Point3::new(0.0, 0.2, 2.0);
    let origin: Point3<f32> = Point3::new(0.0, 0.0, 0.0);

    let up: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);

    let camera = Isometry3::look_at_rh(&camera_pos, &origin, &up);

    //////////////////

    println!("Loading scene...");

    let effects = pp::PostprocessEffectBuilder::target_realtime_fast()
        .optimize_meshes(true)
        .find_instances(true)
        .calc_tangent_space(true)
        .gen_smooth_normals(true)
        .improve_cache_locality(true);

    let scene: Scene = Scene::import("models/happy_buddha.dae", Some(effects.build())).unwrap();

    let mesh = scene.meshes().unwrap().next().unwrap();

    //////////////////

    println!("Loading textures...");

    let mut texture: GLTexture = GLTexture::new(GLTextureKind::Texture2D).unwrap();

    texture.load_from_file("models/uv_test.png", None).expect("Couldn't load texture");

    texture.set_filter(GLTextureFilter::Linear, Some(GLTextureFilter::Linear)).expect("Couldn't set texture filtering");

    let max_anisotropy = texture.get_max_anisotropy().expect("Couldn't get max anisotropy value");
    texture.set_anisotropy(max_anisotropy).expect("Couldn't set max anisotropy");

    texture.generate_mipmap().expect("Couldn't generate mipmaps");

    //////////////////

    //let mut skybox_cubemap: GLTexture = GLTexture::new(GLTextureKind::Cubemap).unwrap();

    //let skybox_paths = GLCubemapPaths {
    //    right: "textures/skybox/right.jpg",
    //    left: "textures/skybox/left.jpg",
    //    top: "textures/skybox/top.jpg",
    //    bottom: "textures/skybox/bottom.jpg",
    //    back: "textures/skybox/back.jpg",
    //    front: "textures/skybox/front.jpg",
    //};

    //skybox_paths.load_into(&mut skybox_cubemap).unwrap();

    //skybox_cubemap.set_filter(GLTextureFilter::Linear, Some(GLTextureFilter::Linear)).expect("Couldn't set texture filtering");

    //let max_anisotropy = skybox_cubemap.get_max_anisotropy().expect("Couldn't get max anisotropy value");
    //skybox_cubemap.set_anisotropy(max_anisotropy).expect("Couldn't set max anisotropy");

    //skybox_cubemap.generate_mipmap().expect("Couldn't generate mipmaps");

    ////////////////////

    let num_indices;

    let mut vao = 0;

    let mut vbo = 0;
    let mut nbo = 0;
    let mut tbo = 0;
    let mut btbo = 0;
    let mut ibo = 0;
    let mut ubo = 0;

    println!("Buffering data...");

    unsafe {
        // Create Vertex Array Object
        GenVertexArrays(1, &mut vao);
        BindVertexArray(vao);

        let ref indices = mesh.indices().unwrap();
        let ref vertices = mesh.vertices().unwrap();
        let ref normals = mesh.normals().unwrap();
        let ref tangents = mesh.tangents().unwrap();
        let ref bitangents = mesh.bitangents().unwrap();
        let (dimensions, ref uvs) = mesh.uv_channel(0).unwrap();

        println!("Loading model with {} indices", indices.len());

        num_indices = indices.len();

        GenBuffers(1, &mut vbo);
        BindBuffer(ARRAY_BUFFER, vbo);
        BufferData(ARRAY_BUFFER,
                   (vertices.len() * size_of::<assimp::AiVector3D>()) as GLsizeiptr,
                   vertices.as_ptr() as *const c_void,
                   STATIC_DRAW);

        GenBuffers(1, &mut nbo);
        BindBuffer(ARRAY_BUFFER, nbo);
        BufferData(ARRAY_BUFFER,
                   (normals.len() * size_of::<assimp::AiVector3D>()) as GLsizeiptr,
                   normals.as_ptr() as *const c_void,
                   STATIC_DRAW
        );

        GenBuffers(1, &mut tbo);
        BindBuffer(ARRAY_BUFFER, tbo);
        BufferData(ARRAY_BUFFER,
                   (tangents.len() * size_of::<assimp::AiVector3D>()) as GLsizeiptr,
                   tangents.as_ptr() as *const c_void,
                   STATIC_DRAW
        );

        GenBuffers(1, &mut btbo);
        BindBuffer(ARRAY_BUFFER, btbo);
        BufferData(ARRAY_BUFFER,
                   (bitangents.len() * size_of::<assimp::AiVector3D>()) as GLsizeiptr,
                   bitangents.as_ptr() as *const c_void,
                   STATIC_DRAW
        );

        GenBuffers(1, &mut ubo);
        BindBuffer(ELEMENT_ARRAY_BUFFER, ubo);
        BufferData(ELEMENT_ARRAY_BUFFER,
                   (uvs.len() * size_of::<assimp::AiVector3D>()) as GLsizeiptr,
                   uvs.as_ptr() as *const c_void,
                   STATIC_DRAW
        );

        GenBuffers(1, &mut ibo);
        BindBuffer(ELEMENT_ARRAY_BUFFER, ibo);
        BufferData(ELEMENT_ARRAY_BUFFER,
                   (indices.len() * size_of::<u32>()) as GLsizeiptr,
                   indices.as_ptr() as *const c_void,
                   STATIC_DRAW
        );

        EnableVertexAttribArray(0);
        BindBuffer(ARRAY_BUFFER, vbo);
        VertexAttribPointer(0, 3, FLOAT, FALSE, 0, ptr::null());

        EnableVertexAttribArray(1);
        BindBuffer(ARRAY_BUFFER, nbo);
        VertexAttribPointer(1, 3, FLOAT, FALSE, 0, ptr::null());

        EnableVertexAttribArray(2);
        BindBuffer(ARRAY_BUFFER, ubo);
        VertexAttribPointer(2, 3, FLOAT, FALSE, 0, ptr::null());

        EnableVertexAttribArray(3);
        BindBuffer(ARRAY_BUFFER, tbo);
        VertexAttribPointer(3, 3, FLOAT, FALSE, 0, ptr::null());

        EnableVertexAttribArray(4);
        BindBuffer(ARRAY_BUFFER, btbo);
        VertexAttribPointer(4, 3, FLOAT, FALSE, 0, ptr::null());

        Enable(DEPTH_TEST);
        DepthFunc(LESS);

        Enable(CULL_FACE);
        CullFace(BACK);

        Enable(MULTISAMPLE);

        Enable(BLEND);
        BlendFunc(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
    }

    // Use shader lighting
    lighting.use_program().unwrap();

    let mut rot_degrees: f32 = 0.0;
    let mut should_render = true;

    let mut aspect_ratio = 16.0f32 / 9.0;

    let base_fov = 60f32;

    let mut fov_modifier = 0.0f32;

    let mut smoothness: f32 = 0.9;

    let mut albedo: f32 = 1.0;

    let mut metallic: f32 = 0.0;

    let mut rotate = true;

    let target_diff = Duration::nanoseconds(1000000000 / 144);
    let mut last = PreciseTime::now();

    unsafe {
        wglSwapIntervalEXT(1);
    }

    let mut u_projection = lighting.get_uniform("projection").unwrap();
    let mut u_view = lighting.get_uniform("view").unwrap();
    let mut u_model = lighting.get_uniform("model").unwrap();
    let mut u_mit = lighting.get_uniform("mit").unwrap();
    let mut u_mvp = lighting.get_uniform("mvp").unwrap();
    let mut u_camera_position = lighting.get_uniform("camera_position").unwrap();
    let mut u_pi = lighting.get_uniform("PI").unwrap();
    let mut u_smoothness = lighting.get_uniform("smoothness").unwrap();
    let mut u_albedo = lighting.get_uniform("albedo").unwrap();
    let mut u_metallic = lighting.get_uniform("metallic").unwrap();

    u_pi.float1(PI).unwrap();

    println!("Starting render loop...");

    'render: loop {
        let mut resize_to: Option<(i32, i32)> = None;

        //Consume all signals, allowing the latest data to overwrite older data
        while let Ok(signal) = rx.try_recv() {
            match signal {
                RenderSignal::Stop => break 'render,
                RenderSignal::ViewportResize(width, height) => {
                    resize_to = Some((width, height));

                    if height > 0 {
                        aspect_ratio = width as f32 / height as f32;
                    }
                },
                RenderSignal::Resume => {
                    should_render = true;
                    println!("Resuming render");
                },
                RenderSignal::Pause => {
                    should_render = false;
                    println!("Pausing render");
                }
                RenderSignal::SetFOV(value) => {
                    fov_modifier += value;
                }
                RenderSignal::AlbedoAdjust(adj) => {
                    albedo = clamp(albedo + adj, 0.0, 1.0);
                }
                RenderSignal::SmoothnessAdjust(adj) => {
                    smoothness = clamp(smoothness + adj, 0.0001, 0.99999);
                }
                RenderSignal::AdjustMetallic(adj) => {
                    metallic = clamp(metallic + adj, 0.0, 1.0);
                }
                RenderSignal::ToggleRotation => {
                    rotate = !rotate;
                }
                _ => {}
            }
        }

        if should_render {
            if let Some((width, height)) = resize_to {
                unsafe { Viewport(0, 0, width as GLsizei, height as GLsizei); }

                println!("Viewport resized to {}x{}", width, height);
            }

            let rot = rot_degrees.to_radians();
            //                                                     aspect ratio,       field of view                    znear, zfar
            let perspective: Perspective3<f32> = Perspective3::new(aspect_ratio, (base_fov + fov_modifier).to_radians(), 0.01, 1000.0);
            //let perspective: Orthographic3<f32> = Orthographic3::new(-10.0, 10.0, -10.0 / aspect_ratio, 10.0 / aspect_ratio, 0.0, 100.0);

            let model_isometry: Isometry3<f32> = Isometry3::new(Vector3::new(0.0, 0.0, 0.0), //Position component
                                                                Vector3::new(0.0, rot, 0.0)); //Rotation component

            let model_mat: Matrix4<f32> = model_isometry.to_homogeneous();
            let model_mit: Matrix4<f32> = model_mat.inverse().unwrap();
            let view_mat: Matrix4<f32> = camera.to_homogeneous();
            let perspective_mat: Matrix4<f32> = perspective.to_matrix();

            let mvp: Matrix4<f32> = perspective_mat * view_mat * model_mat;

            let before = PreciseTime::now();

            unsafe {
                ClearColor(0.25, 0.25, 0.25, 1.0);

                Clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT | STENCIL_BUFFER_BIT);

                u_projection.mat4(&perspective_mat, false).unwrap();
                u_view.mat4(&view_mat, false).unwrap();
                u_model.mat4(&model_mat, false).unwrap();
                u_mit.mat4(&model_mit, true).unwrap(); //transposed
                u_mvp.mat4(&mvp, false).unwrap();

                u_camera_position.point3f(&camera_pos).unwrap();

                u_smoothness.float1(smoothness).unwrap();
                u_albedo.float1(albedo).unwrap();
                u_metallic.float1(metallic).unwrap();


                DrawElements(
                    TRIANGLES,
                    num_indices as GLint,
                    UNSIGNED_INT,
                    ptr::null()
                );
            }

            context.swap_buffers();

            let now = PreciseTime::now();

            let cpu_diff = last.to(now);
            let gpu_diff = before.to(now);

            if target_diff > gpu_diff {
                thread::park_timeout((target_diff - gpu_diff).to_std().unwrap());
            }

            if rotate {
                rot_degrees += (cpu_diff.num_microseconds().unwrap() as f64 / 20000.0) as f32;
            }

            last = now;
        }
    }

    // Cleanup
    //unsafe {
    //    DeleteBuffers(1, &vbo);
    //    DeleteVertexArrays(1, &vao);
    //}
}
