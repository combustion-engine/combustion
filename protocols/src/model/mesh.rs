use std::sync::{Arc, RwLock};

use nalgebra::{Point3, Vector3};

use assimp;

pub struct Mesh {
    indices: Option<Vec<u32>>,
    vertices: Vec<Point3<f32>>,
    normals: Option<Vec<Vector3<f32>>>,
    uvs: Option<Vec<(f32, f32)>>,

}

pub enum AbstractMesh {
    //Standard(super::protocol::Mesh)
}