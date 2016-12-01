use super::bindings::types::*;
use super::bindings::*;

use std::mem;
use std::ptr;

use super::gl_error::*;
use super::gl_shader::*;

#[repr(C)]
pub struct GLMaterial {
    pub ambient: f32,   //ambient light intensity. Zero or near-zero is recommended
    pub roughness: f32, //material roughness from 0 to 1, 0 being perfectly smooth.
    pub albedo: f32,    //diffuse absorbtion amount
    pub ior: f32,       //index of refraction
    pub metallic: f32,  //material conductivity
    //TODO: Not currently implemented
    pub thickness: f32  //thickness of subsurface area.
}