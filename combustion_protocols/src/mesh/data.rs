//! Rust equivalents to mesh.capnp protocol structures

use nalgebra::*;

#[derive(Clone, Debug)]
pub struct Mesh {
    /// Vertex data
    pub vertices: MeshVertices,
    /// Vertex indices
    pub indices: Option<Vec<u32>>,
    /// Layered material indices
    pub materials: Vec<u32>,
}

/// Enum for different vertex layouts
#[derive(Clone, Debug)]
pub enum MeshVertices {
    /// Represents vertices as multiple discrete arrays of data.
    ///
    /// ```text
    /// V1, V2, V3, V4...
    /// N1, N2, N3, N4...
    /// T1, T2, T3, T4...
    /// ```
    Discrete(Vertices),
    /// Represents vertices as interleaved data in a single array.
    ///
    /// `V1, N1, T1, V2, N2, T2, V3, N3, T3, V4, N4, T4...`
    ///
    /// However, if Normals, TexCoords and so forth are not given, they just waste space, so
    /// perhaps Discrete data streams would be more appropriate
    Interleaved(Vec<Vertex>),
}

/// UV-coordinate structure
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct TexCoord {
    pub u: f32,
    pub v: f32,
}

/// Structure for a single vertex.
///
/// This struct is marked as `repr(C)` so it can
/// be passed directly to the GPU in a single buffer
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Vertex {
    pub position: Point3<f32>,
    pub normal: Option<Vector3<f32>>,
    pub uv: Option<TexCoord>,
}

/// Structure for many vertices with non-interleaved data
///
/// Data from this must be passed though multiple buffers
#[derive(Clone, Debug)]
pub struct Vertices {
    pub positions: Vec<Point3<f32>>,
    pub normals: Option<Vec<Vector3<f32>>>,
    pub uvs: Option<Vec<TexCoord>>,
}