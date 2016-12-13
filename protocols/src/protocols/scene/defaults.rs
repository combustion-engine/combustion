//! Contains all the default values for Scene objects

use nalgebra::*;
use super::*;

/// Since there are many named types, define a single trait for that
pub trait DefaultName {
    fn default_name() -> String;
}

impl DefaultName for Scene {
    fn default_name() -> String { "Untitled Scene".to_string() }
}

impl DefaultName for Light {
    fn default_name() -> String { "Untitled Light".to_string() }
}

impl DefaultName for Material {
    fn default_name() -> String { "Untitled Material".to_string() }
}

pub trait DefaultMaterial {}

impl DefaultMaterial for Material {}

pub trait DefaultLight {
    #[inline(always)]
    fn default_zdistance() -> (f32, f32) { (0.0, 1000.0) }

    #[inline(always)]
    fn default_position() -> Point3<f32> { Point3::new(0.0, 1.0, 0.0) }

    #[inline(always)]
    fn default_direction() -> Vector3<f32> { Vector3::new(0.0, -1.0, 0.0) }

    #[inline(always)]
    fn default_kind() -> LightKind { LightKind::Spotlight }

    #[inline(always)]
    fn default_radius() -> f32 { 50.0 }

    #[inline(always)]
    fn default_inner_cone() -> f32 { 0.0 }

    #[inline(always)]
    fn default_outer_cone() -> f32 { 15.0 }

    #[inline(always)]
    fn default_intensity() -> f32 { 1.0 }
}

impl DefaultLight for Light {}

impl Default for Light {
    fn default() -> Light {
        Light {
            name: Light::default_name(),
            zdistance: Light::default_zdistance(),
            position: Light::default_position(),
            direction: Light::default_direction(),
            kind: Light::default_kind(),
            radius: Light::default_radius(),
            inner_cone: Light::default_inner_cone(),
            outer_cone: Light::default_outer_cone(),
            intensity: Light::default_intensity()
        }
    }
}

impl Default for Material {
    fn default() -> Material {
        Material {
            name: Material::default_name()
        }
    }
}