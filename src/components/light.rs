//! Lighted component

use specs;
use nalgebra::Point3;

#[derive(Copy, Clone, Debug)]
pub enum Kind {
    /// Directional light infinitely far away, with all rays parallel
    Directional,
    /// One-dimensional point light in space
    Point {
        /// Spherical radius of light from point. Used for attenuation.
        radius: f32,
    },
    /// Conical spotlight
    Spotlight {
        /// Spherical radius of light from point. Used for attenuation.
        radius: f32,
        /// Spotlight inner cone angle, in radians
        inner_cone: f32,
        /// Spotlight outer cone angle, in radians
        outer_cone: f32,
        /// Efficiency of conical reflector in virtual spotlight
        reflector_efficiency: f32,
    },
    /// Rectangular textured spotlight
    TexturedSpotlight {
        /// Spherical radius of light from point. Used for attenuation.
        radius: f32,
        /// Width of light box, in radians
        width: f32,
        /// Height of light box, in radians
        height: f32,
        /// Efficiency of reflector in virtual spotlight
        reflector_efficiency: f32
    },
    /// TODO: Screen-space object emitter
    Emitter,
    /// TODO: Area light
    ///
    /// https://eheitzresearch.wordpress.com/415-2/
    Area {
        top_left: Point3<f32>,
        bottom_right: Point3<f32>,
    },
    /// TODO: Shape light
    ///
    /// Generalization of the Area light for arbitrary polygons and even textures
    ///
    /// Must be combined with a Sprite component to acquire the shape.
    ///
    /// https://eheitzresearch.wordpress.com/415-2/
    Shape,
}

#[derive(Clone, Debug)]
pub struct Component {
    /// Pretty obvious
    pub enabled: bool,
    /// Light type
    pub kind: Kind,
    /// Light intensity
    pub intensity: f32,
}

impl specs::Component for Component {
    type Storage = specs::VecStorage<Component>;
}
