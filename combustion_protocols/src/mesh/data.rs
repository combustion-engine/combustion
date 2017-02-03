//! Rust equivalents to mesh.capnp protocol structures

use nalgebra::*;

/// Whole mesh with vertices, indices and material indices
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Mesh {
    /// Vertex data
    pub vertices: MeshVertices,
    /// Vertex indices
    pub indices: Option<Vec<u32>>,
    /// Layered material indices
    pub materials: Vec<u32>,
}

/// Enum for different vertex layouts
#[derive(Clone, Debug, Serialize, Deserialize)]
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
#[derive(Default, Copy, Clone, Debug, Serialize, Deserialize)]
#[repr(C)]
pub struct TexCoord {
    /// U Coordinate
    pub u: f32,
    /// V Coordinate
    pub v: f32,
}

/// Structure for a single vertex.
///
/// This struct is marked as `repr(C)` so it can
/// be passed directly to the GPU in a single buffer
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[repr(C)]
pub struct Vertex {
    /// Vertex position
    pub position: Point3<f32>,
    /// Vertex normal
    pub normal: Vector3<f32>,
    /// Vertex texture coordinate
    pub uv: TexCoord,
}

impl Default for Vertex {
    fn default() -> Vertex {
        Vertex {
            position: Point3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0),
            uv: TexCoord::default(),
        }
    }
}

/// Structure for many vertices with non-interleaved data
///
/// Data from this must be passed though multiple buffers
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vertices {
    /// Vertex positions
    pub positions: Vec<Point3<f32>>,
    /// Optional vertex normals
    pub normals: Option<Vec<Vector3<f32>>>,
    /// Optional vertex texture coordinates
    pub uvs: Option<Vec<TexCoord>>,
}