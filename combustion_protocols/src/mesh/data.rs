//! Rust equivalents to mesh.capnp protocol structures

use nalgebra::*;

pub struct Mesh {
    pub vertices: MeshVertices,
    pub indices: Option<Vec<u32>>,
}

pub enum MeshVertices {
    Discrete(Vertices),
    Interleaved(Vec<Vertex>),
}

/// UV-coordinate structure
#[repr(C)]
pub struct TexCoord {
    pub u: f32,
    pub v: f32,
}

#[repr(C)]
pub struct Vertex {
    pub position: Point3<f32>,
    pub normal: Option<Vector3<f32>>,
    pub uv: Option<TexCoord>,
}

pub struct Vertices {
    pub positions: Vec<Point3<f32>>,
    pub normals: Option<Vec<Vector3<f32>>>,
    pub uvs: Option<Vec<TexCoord>>,
}