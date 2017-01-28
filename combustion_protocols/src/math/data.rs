use nalgebra::{Vector3, Matrix4};

/// 3D Transformations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Transform {
    /// translation
    Translation(Vector3<f32>),
    /// rotation (using Euler angles)
    Rotation(Vector3<f32>),
    /// scale on each axis
    Scale(Vector3<f32>),
    /// Arbitrary matrix transform
    Matrix(Matrix4<f32>),
}