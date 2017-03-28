use std::fmt::Debug;

pub mod mesh;
pub mod texture;

pub trait Resource: Sized + Debug {}