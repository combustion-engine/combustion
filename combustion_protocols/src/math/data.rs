//! Data structures for manipulating math data

use nalgebra::{Vector3, Matrix4};

/// 3D Transformations
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Transform {
    /// translation
    #[serde(rename = "translation")]
    Translation(Vector3<f32>),
    /// rotation (using Euler angles)
    #[serde(rename = "rotation")]
    Rotation(Vector3<f32>),
    /// scale on each axis
    #[serde(rename = "scale")]
    Scale(Vector3<f32>),
    /// Arbitrary matrix transform
    #[serde(rename = "matrix")]
    Matrix(Matrix4<f32>),
}