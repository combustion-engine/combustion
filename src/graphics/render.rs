use glfw::{self, Context, WindowEvent};
use std::mem;
use std::ptr;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::ops::Deref;
use std::time::Duration as StdDuration;
use time::{Duration, PreciseTime};
use nalgebra::*;
use lazy;

use num_cpus;
use specs;
use specs::Join;

use ::backend::gl;
use ::backend::gl::gl_error::*;
use ::backend::gl::types::*;
use ::backend::gl::bindings as glb;

use error::*;

use utils;

use components;
use resources;
use systems;

use scene::{Scene, SourceMap};

use super::pipeline::Pipeline;

pub enum RenderSignal {
    Stop,
    Pause,
    Resume,
    ViewportResize(i32, i32),
    Event(WindowEvent)
}

pub struct RenderLoopState {
    total_frames: u64,
    refresh_rate: f64,
    target_diff: Duration,
    paused: bool,
}

impl<'a> RenderLoopState {
    pub fn new(refresh_rate: f64) -> RenderLoopState {
        RenderLoopState {
            total_frames: 0,
            refresh_rate: refresh_rate, //utils::round_multiple(refresh_rate, 10) as f32
            target_diff: Duration::nanoseconds((1000000000.0 / refresh_rate) as i64),
            paused: true,
        }
    }

    #[inline(always)]
    pub fn paused(&self) -> bool { self.paused }

    #[inline(always)]
    pub fn pause(&mut self) { self.paused = true; }

    #[inline(always)]
    pub fn unpause(&mut self) { self.paused = false; }

    #[inline(always)]
    pub fn total_frames(&self) -> u64 { self.total_frames }

    #[inline(always)]
    pub fn refresh_rate(&self) -> f64 { self.refresh_rate }

    #[inline(always)]
    pub fn set_refresh_rate(&mut self, refresh_rate: f64) {
        self.refresh_rate = refresh_rate;
        self.target_diff = Duration::nanoseconds((1000000000.0 / refresh_rate) as i64);
    }
}

pub fn start(mut state: &mut RenderLoopState, mut context: glfw::RenderContext, rx: mpsc::Receiver<RenderSignal>) -> AppResult<()> {
    println!("Targeting {}Hz", state.refresh_rate);

    let mut scene = try!(Scene::new());
    let mut pipeline = try!(Pipeline::new(1280, 720));

    //TODO: Remove this
    try!(::game::entities::test_entities::load(&mut scene));

    let mut delta: systems::Delta = 0.0;
    let mut last = PreciseTime::now();

    //This is constantly swapped out for the render queue resource
    let mut final_render_queue = Vec::with_capacity(resources::render_queue::RENDER_QUEUE_SIZE);

    'render: loop {
        let mut viewport_size = None;

        // Step one: process events
        if scene.with_world(|world| -> bool {
            use resources::event_queue::{Event, Resource as EventQueue};

            let mut event_queue = world.write_resource::<EventQueue>();

            for signal in rx.try_iter() {
                match signal {
                    RenderSignal::Stop => {
                        //TODO: Clean up entities
                        return true;
                    },
                    RenderSignal::ViewportResize(width, height) => {
                        viewport_size = Some((width, height));
                    },
                    RenderSignal::Resume => {
                        state.unpause();
                        println!("Resuming...");
                    },
                    RenderSignal::Pause => {
                        state.pause();
                        println!("Pausing...");
                    }
                    RenderSignal::Event(event) => {
                        event_queue.push(Event::WindowEvent(event));
                    }
                }
            }

            false
        }) {
            break 'render;
        }

        let before = PreciseTime::now();

        if state.paused {
            //Run the scene planner, but with a zero delta because it's paused.
            scene.update(0.0);
        } else {
            // Steps two, buffer GPU data, get render items, and get the view/projection matrices
            let (view, projection) = try!(scene.with_world_sources(|world: &mut specs::World, mut sources: &mut SourceMap| -> AppResult<_> {
                use resources::render_queue::{RenderItem, Resource as RenderQueue};

                use components::transform::Component as Transform;
                use components::mesh::Component as Mesh;
                use components::gpu_buffer::Component as GPU_Buffer;
                use components::renderable::Component as Renderable;
                use components::camera::Component as Camera;

                use resources::camera::Resource as CameraResource;

                let ref transforms = world.read::<Transform>();
                let ref meshes = world.read::<Mesh>();

                let ref mut gpu_buffers = world.write::<GPU_Buffer>();
                let ref mut renderables = world.write::<Renderable>();

                let ref entities = world.entities();

                let mut render_queue = world.write_resource::<RenderQueue>();

                for (ref mut renderable, ref mut gpu_buffer, entity) in (renderables, gpu_buffers, entities).iter() {
                    if renderable.dirty {
                        if let Some(mesh) = meshes.get(entity) {
                            if let Some(ref mesh) = sources.mesh(mesh.source, mesh.index)? {
                                let mut buffer_lock = gpu_buffer.write();
                                let buffer = try!(buffer_lock.get_mut());

                                try!(buffer.buffer_from_mesh(mesh, gl::GLBufferUsage::StaticDraw));
                                debug_println!("Buffered renderable to GPU!");
                            }
                        }

                        renderable.dirty = false;
                    }

                    let (matrix, inverse) = if let Some(transform) = transforms.get(entity) {
                        (
                            transform.matrix,
                            transform.inverse
                        )
                    } else {
                        (
                            Matrix4::new_identity(4),
                            Some(Matrix4::new_identity(4))
                        )
                    };

                    render_queue.push(RenderItem {
                        buffer: gpu_buffer.buffer(),
                        transform: matrix,
                        inverse: inverse
                    });
                }

                let ref mut cameras = world.write::<Camera>();

                let camera_entity = world.read_resource::<CameraResource>().entity();

                let mut view_matrix = Matrix4::new_identity(4);
                let mut projection_matrix = Matrix4::new_identity(4);

                if let Some(mut camera) = cameras.get_mut(camera_entity) {
                    use components::camera::Kind;

                    //If the window was resized, adjust the projection accordingly.
                    if let Some((width, height)) = viewport_size {
                        camera.kind.resize(width as f32, height as f32, None);
                    }

                    projection_matrix = camera.kind.to_homogeneous();
                }

                if let Some(transform) = transforms.get(camera_entity) {
                    view_matrix = transform.matrix;
                }

                render_queue.swap(&mut final_render_queue);

                Ok((view_matrix, projection_matrix))
            }));

            //Step three, set off the system updates
            scene.update(delta);

            //Step four, resize viewport and buffers if necessary
            if let Some((width, height)) = viewport_size {
                use ::resources::projection::Resource as Projection;

                unsafe { glb::Viewport(0, 0, width as GLsizei, height as GLsizei); }

                check_errors!();

                try!(pipeline.resize(width as usize, height as usize));

                println!("Viewport resized to {}x{}", width, height);
            }

            //Step five, the geometry rendering
            try!(pipeline.geometry_pass(|shader: &gl::GLShaderProgram| {
                use components::gpu_buffer::BufferField;

                let mut mvp_uniform = try!(shader.get_uniform("mvp"));
                let mut model_uniform = try!(shader.get_uniform("model"));
                let mut mit_uniform = try!(shader.get_uniform("mit"));

                //Draining the render queue instead of clearing it allows for the memory to be reused.
                for item in final_render_queue.drain(..) {
                    //TODO: Handle poison errors
                    let buffer_lock = item.buffer.read().unwrap();
                    let buffer = try!(buffer_lock.get());

                    try!(buffer.bind());

                    try!(buffer.bind_attrib_arrays(&[BufferField::Vertex, BufferField::Normal, BufferField::Uv, BufferField::Tangent, BufferField::Bitangent]));

                    let mvp = projection * view * item.transform;
                    let inverse = item.inverse.unwrap_or(Matrix4::new_identity(4));

                    try!(mvp_uniform.mat4(&mvp, false));
                    try!(model_uniform.mat4(&item.transform, false));
                    try!(mit_uniform.mat4(&inverse, true));

                    unsafe {
                        glb::DrawElements(
                            glb::TRIANGLES,
                            buffer.num_indices() as GLint,
                            glb::UNSIGNED_INT,
                            ptr::null()
                        );
                    }

                    check_errors!();
                }

                Ok(())
            }));

            //Step six, the lighting pass
            try!(pipeline.lighting_pass(|| {
                //TODO
                Ok(())
            }));

            //Step seven, render out to the screen
            try!(pipeline.final_pass());

            //Step eight, swap the buffers
            context.swap_buffers();

            //Done! kind of
            state.total_frames += 1;
        }

        //By having no less than target_diff on the GPU, we can maintain a steady frame rate near the monitor refresh rate

        let gpu_diff = before.to(PreciseTime::now());

        if state.target_diff > gpu_diff {
            thread::park_timeout((state.target_diff - gpu_diff).to_std().unwrap());
        }

        // Wait on planner to finish AFTER the GPU timeout has finished, so as to not incur double waiting
        scene.wait();

        // For delta times, use the CPU time difference since physics doesn't care about the GPU time
        let now = PreciseTime::now();
        let cpu_diff = last.to(now);

        delta = (cpu_diff.num_microseconds().unwrap_or(0) as systems::Delta) / 1_000_000.0;

        last = now;
    }

    Ok(())
}