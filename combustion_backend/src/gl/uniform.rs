use super::bindings::types::*;
use super::bindings::*;

use super::error::*;

use nalgebra::*;

macro_rules! impl_scalar_uniform {
    ($glFunc:ident:$rustType:ty as $glType:ty => $name:ident($($field:ident),+)) => {
        pub fn $name(&mut self, $($field: $rustType),+) -> GLResult<()> {
            unsafe {
                $glFunc(self.0, $($field as $glType),+);
            }

            check_gl_errors!();

            Ok(())
        }
    }
}

macro_rules! impl_array_uniform {
    ($glFunc:ident:$rustType:ty as $glType:ty => $name:ident) => {
        pub fn $name(&mut self, values: Vec<$rustType>) -> GLResult<()> {
            unsafe {
                $glFunc(self.0,
                        values.len() as GLsizei,
                        values.as_ptr() as *const _ as *const $glType
                );
            }

            check_gl_errors!();

            Ok(())
        }
    }
}

macro_rules! impl_matrix_uniform {
    ($glFunc:ident:$rustType:ty as $glType:ty => $name:ident($field:ident)) => {
        pub fn $name(&mut self, mat: &$field<$rustType>, transpose: bool) -> GLResult<()> {
            unsafe {
                $glFunc(self.0, 1, if transpose { TRUE } else { FALSE }, mat.as_ref() as *const _ as *const $glType);
            }

            check_gl_errors!();

            Ok(())
        }
    }
}

macro_rules! impl_nalgebra_alias_uniform {
    ($uFunc:ident:$rustType:ty => $name:ident($field:ident => ($($inner_field:ident),+))) => {
        #[inline(always)]
        pub fn $name(&mut self, value: &$field<$rustType>) -> GLResult<()> {
            self.$uFunc($(value.$inner_field),+)
        }
    }
}

#[derive(Eq, PartialEq)]
pub struct GLUniform(pub GLint);

impl GLUniform {
    impl_scalar_uniform!(Uniform1f:f32 as GLfloat => float1(x));
    impl_scalar_uniform!(Uniform2f:f32 as GLfloat => float2(x, y));
    impl_scalar_uniform!(Uniform3f:f32 as GLfloat => float3(x, y, z));
    impl_scalar_uniform!(Uniform4f:f32 as GLfloat => float4(x, y, z, w));

    impl_scalar_uniform!(Uniform1i:i32 as GLint => int1(x));
    impl_scalar_uniform!(Uniform2i:i32 as GLint => int2(x, y));
    impl_scalar_uniform!(Uniform3i:i32 as GLint => int3(x, y, z));
    impl_scalar_uniform!(Uniform4i:i32 as GLint => int4(x, y, z, w));

    impl_scalar_uniform!(Uniform1ui:u32 as GLuint => uint1(x));
    impl_scalar_uniform!(Uniform2ui:u32 as GLuint => uint2(x, y));
    impl_scalar_uniform!(Uniform3ui:u32 as GLuint => uint3(x, y, z));
    impl_scalar_uniform!(Uniform4ui:u32 as GLuint => uint4(x, y, z, w));

    /////////////////

    impl_array_uniform!(Uniform1fv:f32 as GLfloat => float1v);
    impl_array_uniform!(Uniform2fv:f32 as GLfloat => float2v);
    impl_array_uniform!(Uniform3fv:f32 as GLfloat => float3v);
    impl_array_uniform!(Uniform4fv:f32 as GLfloat => float4v);

    impl_array_uniform!(Uniform1iv:i32 as GLint => int1v);
    impl_array_uniform!(Uniform2iv:i32 as GLint => int2v);
    impl_array_uniform!(Uniform3iv:i32 as GLint => int3v);
    impl_array_uniform!(Uniform4iv:i32 as GLint => int4v);

    impl_array_uniform!(Uniform1uiv:u32 as GLuint => uint1v);
    impl_array_uniform!(Uniform2uiv:u32 as GLuint => uint2v);
    impl_array_uniform!(Uniform3uiv:u32 as GLuint => uint3v);
    impl_array_uniform!(Uniform4uiv:u32 as GLuint => uint4v);

    /////////////////

    impl_matrix_uniform!(UniformMatrix2fv:f32 as GLfloat => mat2(Matrix2));
    impl_matrix_uniform!(UniformMatrix3fv:f32 as GLfloat => mat3(Matrix3));
    impl_matrix_uniform!(UniformMatrix4fv:f32 as GLfloat => mat4(Matrix4));

    /////////////////

    impl_nalgebra_alias_uniform!(float1:f32 => vec1f(Vector1 => (x)));
    impl_nalgebra_alias_uniform!(float2:f32 => vec2f(Vector2 => (x, y)));
    impl_nalgebra_alias_uniform!(float3:f32 => vec3f(Vector3 => (x, y, z)));
    impl_nalgebra_alias_uniform!(float4:f32 => vec4f(Vector4 => (x, y, z, w)));

    impl_nalgebra_alias_uniform!(int1:i32 => vec1i(Vector1 => (x)));
    impl_nalgebra_alias_uniform!(int2:i32 => vec2i(Vector2 => (x, y)));
    impl_nalgebra_alias_uniform!(int3:i32 => vec3i(Vector3 => (x, y, z)));
    impl_nalgebra_alias_uniform!(int4:i32 => vec4i(Vector4 => (x, y, z, w)));

    impl_nalgebra_alias_uniform!(uint1:u32 => vec1ui(Vector1 => (x)));
    impl_nalgebra_alias_uniform!(uint2:u32 => vec2ui(Vector2 => (x, y)));
    impl_nalgebra_alias_uniform!(uint3:u32 => vec3ui(Vector3 => (x, y, z)));
    impl_nalgebra_alias_uniform!(uint4:u32 => vec4ui(Vector4 => (x, y, z, w)));

    /////////////////

    impl_nalgebra_alias_uniform!(float1:f32 => point1f(Point1 => (x)));
    impl_nalgebra_alias_uniform!(float2:f32 => point2f(Point2 => (x, y)));
    impl_nalgebra_alias_uniform!(float3:f32 => point3f(Point3 => (x, y, z)));
    impl_nalgebra_alias_uniform!(float4:f32 => point4f(Point4 => (x, y, z, w)));

    impl_nalgebra_alias_uniform!(int1:i32 => point1i(Point1 => (x)));
    impl_nalgebra_alias_uniform!(int2:i32 => point2i(Point2 => (x, y)));
    impl_nalgebra_alias_uniform!(int3:i32 => point3i(Point3 => (x, y, z)));
    impl_nalgebra_alias_uniform!(int4:i32 => point4i(Point4 => (x, y, z, w)));

    impl_nalgebra_alias_uniform!(uint1:u32 => point1ui(Point1 => (x)));
    impl_nalgebra_alias_uniform!(uint2:u32 => point2ui(Point2 => (x, y)));
    impl_nalgebra_alias_uniform!(uint3:u32 => point3ui(Point3 => (x, y, z)));
    impl_nalgebra_alias_uniform!(uint4:u32 => point4ui(Point4 => (x, y, z, w)));
}