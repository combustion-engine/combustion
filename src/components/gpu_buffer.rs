//! GPU Buffer component
//!
//! This component stores references to all OpenGL buffers used for the entity

use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::ptr;
use std::mem;

use specs;
use assimp::components::mesh::Mesh;

use ::backend::gl::*;
use ::backend::gl::types::*;
use ::backend::gl::bindings as glb;

use lazy;

/// Fields to be used in specifying shader layouts
pub enum BufferField {
    Vertex,
    Normal,
    Uv,
    Tangent,
    Bitangent,
}

/// Contains all the OpenGL buffers for an entity
pub struct Buffer {
    vao: GLVertexArray,
    num_indices: usize,
    index_buffer: Option<GLBuffer>,
    vertex_buffer: Option<GLBuffer>,
    normal_buffer: Option<GLBuffer>,
    uv_buffer: Option<GLBuffer>,
    tangent_buffer: Option<GLBuffer>,
    bitangent_buffer: Option<GLBuffer>,
}

impl lazy::LazyInit<Buffer, GLError> for Buffer {
    fn init() -> Result<Buffer, GLError> {
        Buffer::new()
    }
}

impl Buffer {
    pub fn new() -> GLResult<Buffer> {
        Ok(Buffer {
            vao: try!(GLVertexArray::new()),
            num_indices: 0,
            index_buffer: None,
            vertex_buffer: None,
            normal_buffer: None,
            uv_buffer: None,
            tangent_buffer: None,
            bitangent_buffer: None,
        })
    }

    /// Get the number of indices for the buffered entity
    #[inline(always)]
    pub fn num_indices(&self) -> usize { self.num_indices }

    /// Binds the entity VAO, allowing it to be rendered
    #[inline(always)]
    pub fn bind(&self) -> GLResult<()> { self.vao.bind() }

    /// Buffer data to the GPU from an Assimp Mesh structure
    pub fn buffer_from_mesh<'a>(&mut self, mesh: &'a Mesh<'a>, usage: GLBufferUsage) -> GLResult<()> {
        try!(self.bind());

        try!(self.buffer_vertices(mesh, usage));
        try!(self.buffer_normals(mesh, usage));
        try!(self.buffer_uvs(mesh, usage));
        try!(self.buffer_tangents(mesh, usage));
        try!(self.buffer_bitangents(mesh, usage));
        try!(self.buffer_indices(mesh, usage));

        Ok(())
    }

    /// For the bound shader, bind the buffers to it using the given layout order
    pub fn bind_attrib_arrays(&self, order: &[BufferField]) -> GLResult<()> {
        for (i, field) in order.iter().enumerate() {
            let optional_buffer = match *field {
                BufferField::Vertex => &self.vertex_buffer,
                BufferField::Normal => &self.normal_buffer,
                BufferField::Uv => &self.uv_buffer,
                BufferField::Tangent => &self.tangent_buffer,
                BufferField::Bitangent => &self.bitangent_buffer,
            };

            if let Some(ref buffer) = *optional_buffer {
                unsafe {
                    glb::EnableVertexAttribArray(i as GLuint);
                }

                check_errors!();

                try!(buffer.bind());

                unsafe {
                    glb::VertexAttribPointer(i as GLuint, 3, glb::FLOAT, glb::FALSE, 0, ptr::null());
                }

                check_errors!();
            }
        }
        Ok(())
    }

    fn buffer_vertices<'a>(&mut self, mesh: &'a Mesh<'a>, usage: GLBufferUsage) -> GLResult<()> {
        if let Some(vertices) = mesh.vertices() {
            let mut missing_buffer = false;

            if let Some(mut buffer) = self.vertex_buffer.as_mut() {
                try!(buffer.buffer_slice(vertices, usage));
            } else {
                missing_buffer = true;
            }

            if missing_buffer {
                self.vertex_buffer = Some(GLBuffer::new(GLBufferTarget::ArrayBuffer)?);

                return self.buffer_vertices(mesh, usage);
            }
        }

        Ok(())
    }

    fn buffer_normals<'a>(&mut self, mesh: &'a Mesh<'a>, usage: GLBufferUsage) -> GLResult<()> {
        if let Some(normals) = mesh.normals() {
            let mut missing_buffer = false;

            if let Some(mut buffer) = self.normal_buffer.as_mut() {
                try!(buffer.buffer_slice(normals, usage));
            } else {
                missing_buffer = true;
            }

            if missing_buffer {
                self.normal_buffer = Some(GLBuffer::new(GLBufferTarget::ArrayBuffer)?);

                return self.buffer_normals(mesh, usage);
            }
        }

        Ok(())
    }

    fn buffer_tangents<'a>(&mut self, mesh: &'a Mesh<'a>, usage: GLBufferUsage) -> GLResult<()> {
        if let Some(tangents) = mesh.tangents() {
            let mut missing_buffer = false;

            if let Some(mut buffer) = self.tangent_buffer.as_mut() {
                try!(buffer.buffer_slice(tangents, usage));
            } else {
                missing_buffer = true;
            }

            if missing_buffer {
                self.tangent_buffer = Some(GLBuffer::new(GLBufferTarget::ArrayBuffer)?);

                return self.buffer_tangents(mesh, usage);
            }
        }

        Ok(())
    }

    fn buffer_bitangents<'a>(&mut self, mesh: &'a Mesh<'a>, usage: GLBufferUsage) -> GLResult<()> {
        if let Some(bitangents) = mesh.bitangents() {
            let mut missing_buffer = false;

            if let Some(mut buffer) = self.bitangent_buffer.as_mut() {
                try!(buffer.buffer_slice(bitangents, usage));
            } else {
                missing_buffer = true;
            }

            if missing_buffer {
                self.bitangent_buffer = Some(GLBuffer::new(GLBufferTarget::ArrayBuffer)?);

                return self.buffer_bitangents(mesh, usage);
            }
        }

        Ok(())
    }

    fn buffer_uvs<'a>(&mut self, mesh: &'a Mesh<'a>, usage: GLBufferUsage) -> GLResult<()> {
        if let Some((_, uvs)) = mesh.uv_channel(0) {
            let mut missing_buffer = false;

            if let Some(mut buffer) = self.uv_buffer.as_mut() {
                try!(buffer.buffer_slice(uvs, usage));
            } else {
                missing_buffer = true;
            }

            if missing_buffer {
                self.uv_buffer = Some(GLBuffer::new(GLBufferTarget::ArrayBuffer)?);

                return self.buffer_uvs(mesh, usage);
            }
        }

        Ok(())
    }

    fn buffer_indices<'a>(&mut self, mesh: &'a Mesh<'a>, usage: GLBufferUsage) -> GLResult<()> {
        if let Some(indices) = mesh.indices() {
            let mut missing_buffer = false;

            if let Some(mut buffer) = self.index_buffer.as_mut() {
                try!(buffer.buffer_elements(&indices, usage));
            } else {
                missing_buffer = true;
            }

            if missing_buffer {
                let mut buffer = try!(GLBuffer::new(GLBufferTarget::ElementArrayBuffer));

                try!(buffer.buffer_elements(&indices, usage));

                self.index_buffer = Some(buffer);
            }

            self.num_indices = indices.len();
        }

        Ok(())
    }
}

/// Lazily created buffer to avoid excess OpenGL objects for unused entities.
pub type LazyBuffer = lazy::Lazy<Buffer, GLError>;

/// Send/Sync wrapper for a `LazyBuffer` allowing it to be accessed from multiple threads.
pub type LazyBufferSync = Arc<RwLock<LazyBuffer>>;

pub struct Component {
    buffer: LazyBufferSync,
}

unsafe impl Sync for Component {}

unsafe impl Send for Component {}

impl specs::Component for Component {
    type Storage = specs::VecStorage<Component>;
}

impl Default for Component {
    #[inline(always)]
    fn default() -> Component { Component::new() }
}

impl Component {
    pub fn new() -> Component {
        Component { buffer: LazyBufferSync::default() }
    }

    /// Acquire synchronized read access to the buffer
    pub fn read(&self) -> RwLockReadGuard<LazyBuffer> {
        self.buffer.read().unwrap()
    }

    /// Acquire synchronized write access to the buffer
    pub fn write(&self) -> RwLockWriteGuard<LazyBuffer> {
        self.buffer.write().unwrap()
    }

    /// Clone a reference to the buffer
    pub fn buffer(&self) -> LazyBufferSync {
        self.buffer.clone()
    }
}