#![allow(missing_docs)]

use nalgebra::{Vector3, Point3, Matrix4};

include!(concat!(env!("OUT_DIR"), "/protocols/math_capnp.rs"));

impl<'a> matrix4::Builder<'a> {
    pub fn set_matrix(&mut self, mat: &Matrix4<f32>) {
        self.set_m11(mat.m11);
        self.set_m21(mat.m21);
        self.set_m31(mat.m31);
        self.set_m41(mat.m41);
        self.set_m12(mat.m12);
        self.set_m22(mat.m22);
        self.set_m32(mat.m32);
        self.set_m42(mat.m42);
        self.set_m13(mat.m13);
        self.set_m23(mat.m23);
        self.set_m33(mat.m33);
        self.set_m43(mat.m43);
        self.set_m14(mat.m14);
        self.set_m24(mat.m24);
        self.set_m34(mat.m34);
        self.set_m44(mat.m44);
    }
}

impl<'a> matrix4::Reader<'a> {
    pub fn get_matrix(&self) -> Matrix4<f32> {
        Matrix4::new(
            self.get_m11(), self.get_m21(), self.get_m31(), self.get_m41(),
            self.get_m12(), self.get_m22(), self.get_m32(), self.get_m42(),
            self.get_m13(), self.get_m23(), self.get_m33(), self.get_m43(),
            self.get_m14(), self.get_m24(), self.get_m34(), self.get_m44(),
        )
    }
}

impl<'a> vector3::Builder<'a> {
    pub fn set_vector(&mut self, vector: &Vector3<f32>) {
        self.set_x(vector.x);
        self.set_y(vector.y);
        self.set_z(vector.z);
    }
}

impl<'a> vector3::Reader<'a> {
    #[inline]
    pub fn get_vector(&self) -> Vector3<f32> {
        Vector3::new(self.get_x(), self.get_y(), self.get_z())
    }
}

impl<'a> point3::Builder<'a> {
    pub fn set_point(&mut self, point: &Point3<f32>) {
        self.set_x(point.x);
        self.set_y(point.y);
        self.set_z(point.z);
    }
}

impl<'a> point3::Reader<'a> {
    #[inline]
    pub fn get_point(&self) -> Point3<f32> {
        Point3::new(self.get_x(), self.get_y(), self.get_z())
    }
}