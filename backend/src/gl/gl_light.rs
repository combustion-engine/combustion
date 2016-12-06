use super::bindings::types::*;
use super::bindings::*;

use std::mem;
use std::ptr;

use nalgebra::*;

use palette::named;

use super::gl_color::*;

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum GLLightKind {
    /// Like the Sun. All rays are parallel.
    Directional = 1,
    /// Like a normal light-bulb
    Point = 2,
    /// Like a flashlight
    Spotlight = 3
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct GLLight {
    /// Minimum and maximum distances the light can touch. Hard limit. Ignored for Directional Lights
    pub zdistance: (f32, f32),
    /// Position of light in space. Ignored for Directional lights
    pub position: Point3<f32>,
    /// Direction the light is pointing at. Used in Directional and Spot lights
    pub direction: Vector3<f32>,
    /// Light color
    pub color: GLColor,
    /// Ambient color
    pub ambient: GLColor,
    /// Directional, Point, Spot, etc
    pub kind: GLLightKind,
    /// Spherical radius of entire light. Soft limit. Ignored for Directional lights
    pub radius: f32,
    /// Cosine of angle of inner light cone. Only used for spotlights
    pub inner_cone_cos: f32,
    /// Cosine of angle of outer light cone. Only used for spotlights
    pub outer_cone_cos: f32,
    /// Light intensity
    pub intensity: f32
}

impl Default for GLLight {
    #[inline(always)]
    fn default() -> GLLight {
        GLLight {
            zdistance: (0.0, 0.0),
            position: Point3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(0.0, -1.0, 0.0),
            color: GLColor::from(named::WHITE),
            ambient: GLColor::none(),
            kind: GLLightKind::Directional,
            radius: 0.0,
            inner_cone_cos: 0.0,
            outer_cone_cos: 0.0,
            intensity: 1.0
        }
    }
}

impl GLLight {
    #[inline]
    pub fn directional(direction: Vector3<f32>, color: GLColor, ambient: Option<GLColor>, intensity: f32) -> GLLight {
        GLLight {
            direction: direction,
            color: color,
            ambient: ambient.unwrap_or(GLColor::none()),
            kind: GLLightKind::Directional,
            intensity: intensity,
            ..GLLight::default()
        }
    }

    #[inline]
    pub fn point(position: Point3<f32>, color: GLColor, ambient: Option<GLColor>, radius: f32, intensity: f32, zdistance: (f32, f32)) -> GLLight {
        assert!(zdistance.0 < zdistance.1);

        GLLight {
            zdistance: zdistance,
            position: position,
            color: color,
            ambient: ambient.unwrap_or(GLColor::none()),
            kind: GLLightKind::Point,
            radius: radius,
            intensity: intensity,
            ..GLLight::default()
        }
    }

    #[inline]
    pub fn spotlight(position: Point3<f32>, direction: Vector3<f32>, color: GLColor, ambient: Option<GLColor>,
                     radius: f32, intensity: f32, cone_angles: (f32, f32), zdistance: (f32, f32)) -> GLLight {
        assert!(zdistance.0 < zdistance.1);
        assert!(cone_angles.0 < cone_angles.1);

        GLLight {
            zdistance: zdistance,
            position: position,
            direction: direction,
            color: color,
            ambient: ambient.unwrap_or(GLColor::none()),
            kind: GLLightKind::Spotlight,
            radius: radius,
            inner_cone_cos: cone_angles.0.cos(),
            outer_cone_cos: cone_angles.1.cos(),
            intensity: intensity
        }
    }

    #[inline]
    pub fn spotlight_lookat(position: Point3<f32>, target: Point3<f32>, color: GLColor, ambient: Option<GLColor>,
                            radius: f32, intensity: f32, cone_angles: (f32, f32), zdistance: (f32, f32)) -> GLLight {
        assert!(zdistance.0 < zdistance.1);
        assert!(cone_angles.0 < cone_angles.1);

        GLLight {
            zdistance: zdistance,
            position: position,
            direction: (target - position).normalize(),
            color: color,
            ambient: ambient.unwrap_or(GLColor::none()),
            kind: GLLightKind::Spotlight,
            radius: radius,
            inner_cone_cos: cone_angles.0.cos(),
            outer_cone_cos: cone_angles.1.cos(),
            intensity: intensity
        }
    }
}