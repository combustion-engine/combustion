//! Rust equivalents to mesh.capnp protocol structures

use std::fmt::{Debug, Formatter, Result as FmtResult};

use nalgebra::*;

fn skip_serializing_if_none_or_empty<T>(value: &Option<Vec<T>>) -> bool {
    match *value {
        Some(ref seq) => seq.is_empty(),
        None => true,
    }
}

/// Whole mesh with vertices, indices and material indices
#[derive(Clone, Serialize, Deserialize)]
pub struct Mesh {
    /// Vertex data
    pub vertices: MeshVertices,
    /// Vertex indices
    #[serde(skip_serializing_if = "skip_serializing_if_none_or_empty")]
    #[serde(default)]
    pub indices: Option<Vec<u32>>,
    /// Layered material indices
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub materials: Vec<u32>,
}

impl Debug for Mesh {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Mesh {{vertices: {:?}, indices: {:?}, materials: {}}}",
               self.vertices,
               self.indices.as_ref().map(|indices| indices.len()),
               self.materials.len())
    }
}

/// Enum for different vertex layouts
#[derive(Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MeshVertices {
    /// Represents vertices as multiple discrete arrays of data.
    ///
    /// ```text
    /// V1, V2, V3, V4...
    /// N1, N2, N3, N4...
    /// T1, T2, T3, T4...
    /// ```
    #[serde(rename = "discrete")]
    Discrete(Vertices),
    /// Represents vertices as interleaved data in a single array.
    ///
    /// `V1, N1, T1, V2, N2, T2, V3, N3, T3, V4, N4, T4...`
    ///
    /// However, if Normals, TexCoords and so forth are not given, they just waste space, so
    /// perhaps Discrete data streams would be more appropriate
    #[serde(rename = "interleaved")]
    Interleaved(Vec<Vertex>),
}

impl Debug for MeshVertices {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "MeshVertices {{ {} }}", match *self {
            MeshVertices::Discrete(ref vertices) => {
                format!("Discrete {{ {:?} }}", vertices)
            },
            MeshVertices::Interleaved(ref vertices) => {
                format!("Interleaved {{ {} vertices }}", vertices.len())
            }
        })
    }
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

impl TexCoord {
    /// Create a new `TexCoord` from its corresponding `u` and `v` parts
    pub fn new(u: f32, v: f32) -> TexCoord {
        TexCoord { u: u, v: v }
    }
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
#[derive(Clone, Serialize, Deserialize)]
pub struct Vertices {
    /// Vertex positions
    pub positions: Vec<Point3<f32>>,
    /// Optional vertex normals
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub normals: Option<Vec<Vector3<f32>>>,
    /// Optional vertex texture coordinates
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub uvs: Option<Vec<TexCoord>>,
}

impl Debug for Vertices {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Vertices {{ positions: {}, normals: {:?}, uvs: {:?} }}",
               self.positions.len(),
               self.normals.as_ref().map(|normals| normals.len()),
               self.uvs.as_ref().map(|uvs| uvs.len()))
    }
}