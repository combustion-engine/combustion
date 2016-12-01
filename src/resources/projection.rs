use specs;
use nalgebra::*;

#[derive(Copy, Clone, Debug)]
pub enum Kind {
    Perspective(Perspective3<f32>),
    Orthographic(Orthographic3<f32>)
}

impl Kind {
    pub fn resize(&mut self, width: f32, height: f32, fovy: Option<f32>) {
        match &mut *self {
            &mut Kind::Orthographic(ref mut projection) => {
                projection.set_left(0.0);
                projection.set_right(width);
                projection.set_bottom(height);
                projection.set_top(0.0);
            }
            &mut Kind::Perspective(ref mut projection) => {
                if let Some(fovy) = fovy {
                    projection.set_fovy(fovy);
                }

                projection.set_aspect(width / height);
            }
        }
    }

    pub fn zdistance(&mut self, znear: f32, zfar: f32) {
        match &mut *self {
            &mut Kind::Orthographic(ref mut projection) => {
                projection.set_znear(znear);
                projection.set_zfar(zfar);
            }
            &mut Kind::Perspective(ref mut projection) => {
                projection.set_znear(znear);
                projection.set_zfar(zfar);
            }
        }
    }
}

impl ToHomogeneous<Matrix4<f32>> for Kind {
    fn to_homogeneous(&self) -> Matrix4<f32> {
        match *self {
            Kind::Perspective(projection) => {
                projection.to_matrix()
            }
            Kind::Orthographic(projection) => {
                projection.to_matrix()
            }
        }
    }
}

pub struct Resource {
    pub kind: Kind
}

impl Resource {
    #[inline(always)]
    pub fn new_perspective(aspect: f32, fovy: f32, znear: f32, zfar: f32) -> Resource {
        Resource { kind: Kind::Perspective(Perspective3::new(aspect, fovy, znear, zfar)) }
    }

    #[inline(always)]
    pub fn new_perspective_window(width: f32, height: f32, fovy: f32, znear: f32, zfar: f32) -> Resource {
        Resource::new_perspective(width / height, fovy, znear, zfar)
    }

    #[inline(always)]
    pub fn new_orthographic(left: f32, right: f32, bottom: f32, top: f32, znear: f32, zfar: f32) -> Resource {
        Resource { kind: Kind::Orthographic(Orthographic3::new(left, right, bottom, top, znear, zfar)) }
    }

    #[inline(always)]
    pub fn new_orthographic_window(width: f32, height: f32, znear: f32, zfar: f32) -> Resource {
        Resource::new_orthographic(0.0, width, height, 0.0, znear, zfar)
    }
}