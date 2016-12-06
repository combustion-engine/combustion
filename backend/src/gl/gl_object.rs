use super::bindings::types::*;
use super::bindings::*;

use std::mem;
use std::ptr;
use std::cell::{Cell, RefCell};
use std::sync::{Arc, Weak};

use nalgebra::*;

use super::gl_error::*;
use super::gl_shader::*;
use super::gl_color::*;
use super::gl_scene::*;
use super::gl_shader_program::*;
use super::gl_buffer::*;

pub trait GLDrawable {
    /// Load up buffers, set shaders, etc.
    fn prepare(&mut self, scene: &mut GLScene) -> GLResult<()>;

    /// Do the actual draw calls
    fn draw(&mut self, scene: &mut GLScene) -> GLResult<()>;
}

pub struct GLMesh {
    pub vertices: Vec<f32>,
    pub normals: Vec<f32>,
    pub uvs: Vec<f32>,
    pub indices: Vec<u32>
}

pub struct GLStaticMeshObject {
    pub mesh: GLMesh,
    pub transform: Similarity3<f32>,
    //buffers: GLObjectBuffers,
}

impl GLDrawable for GLStaticMeshObject {
    fn prepare(&mut self, scene: &mut GLScene) -> GLResult<()> {
        Ok(())
    }

    fn draw(&mut self, scene: &mut GLScene) -> GLResult<()> {
        Ok(())
    }
}