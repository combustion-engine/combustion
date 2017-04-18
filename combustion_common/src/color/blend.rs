//! Blend operations for colors

#![allow(missing_docs, unused_imports)]

use ::num_utils::{LerpExt, LerpGenericExt};

use super::Color;
use super::ext::ColorExt;

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum BlendMode {
    Zero,
    One,
    SourceColor,
    OneMinusSourceColor,
    DestinationColor,
    OneMinusDestinationColor,
    SourceAlpha,
    OneMinusSourceAlpha,
    DestinationAlpha,
    OneMinusDestinationAlpha,
}

impl BlendMode {
    pub fn apply_alpha(&self, x: f32, sc: f32, dc: f32, sa: f32, da: f32) -> f32 {
        match *self {
            BlendMode::Zero => { 0.0 }
            BlendMode::One => { x }
            BlendMode::SourceColor => { x * sc }
            BlendMode::OneMinusSourceColor => { x * (1.0 - sc) }
            BlendMode::DestinationColor => { x * dc }
            BlendMode::OneMinusDestinationColor => { x * (1.0 - dc) }
            BlendMode::SourceAlpha => { x * sa }
            BlendMode::OneMinusSourceAlpha => { x * (1.0 - sa) }
            BlendMode::DestinationAlpha => { x * da }
            BlendMode::OneMinusDestinationAlpha => { x * (1.0 - da) }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct BlendModes { pub source: BlendMode, pub destination: BlendMode }

pub struct SeparateBlendModes { pub color: BlendModes, pub alpha: BlendModes }

pub const DEFAULT_BLEND_MODES: SeparateBlendModes = SeparateBlendModes {
    color: BlendModes {
        source: BlendMode::SourceAlpha,
        destination: BlendMode::OneMinusSourceAlpha,
    },
    alpha: BlendModes {
        source: BlendMode::One,
        destination: BlendMode::One,
    }
};

/// Color blending extensions
pub trait ColorBlend: Sized {
    fn add(self, other: Color, modes: SeparateBlendModes) -> Color;
    fn subtract(self, other: Color, modes: SeparateBlendModes) -> Color;
    fn difference(self, other: Color, modes: SeparateBlendModes) -> Color;
    fn multiply(self, other: Color, modes: SeparateBlendModes) -> Color;
    fn average(self, other: Color, modes: SeparateBlendModes) -> Color;
    //fn negate(self, other: Color, modes: SeparateBlendModes) -> Color;
    //fn exclusion(self, other: Color, modes: SeparateBlendModes) -> Color;

    fn linear_dodge(self, other: Color, modes: SeparateBlendModes) -> Color {
        self.add(other, modes)
    }

    fn linear_burn(self, other: Color, modes: SeparateBlendModes) -> Color {
        self.subtract(other, modes)
    }

    fn over(self, other: Color) -> Color;
}

fn alpha_blend_components(source: Color, destination: Color, modes: SeparateBlendModes) -> (Color, Color) {
    let alpha_blended_source = Color {
        r: modes.color.source.apply_alpha(source.r, source.r, destination.r, source.a, destination.a),
        g: modes.color.source.apply_alpha(source.g, source.g, destination.g, source.a, destination.a),
        b: modes.color.source.apply_alpha(source.b, source.b, destination.b, source.a, destination.a),
        a: modes.alpha.source.apply_alpha(source.a, source.a, destination.a, source.a, destination.a),
    };

    let alpha_blended_destination = Color {
        r: modes.color.destination.apply_alpha(destination.r, source.r, destination.r, source.a, destination.a),
        g: modes.color.destination.apply_alpha(destination.g, source.g, destination.g, source.a, destination.a),
        b: modes.color.destination.apply_alpha(destination.b, source.b, destination.b, source.a, destination.a),
        a: modes.alpha.destination.apply_alpha(destination.a, source.a, destination.a, source.a, destination.a),
    };

    (alpha_blended_source, alpha_blended_destination)
}

impl ColorBlend for Color {
    fn add(self, other: Color, modes: SeparateBlendModes) -> Color {
        let (s, d) = alpha_blend_components(self, other, modes);

        Color {
            r: s.r + d.r,
            g: s.g + d.g,
            b: s.b + d.b,
            a: s.a + d.a,
        }
    }

    fn subtract(self, other: Color, modes: SeparateBlendModes) -> Color {
        let (s, d) = alpha_blend_components(self, other, modes);

        Color {
            r: (s.r + d.r - 1.0).max(0.0),
            g: (s.g + d.g - 1.0).max(0.0),
            b: (s.b + d.b - 1.0).max(0.0),
            a: (s.a + d.a - 1.0).max(0.0),
        }
    }

    fn difference(self, other: Color, modes: SeparateBlendModes) -> Color {
        let (s, d) = alpha_blend_components(self, other, modes);

        Color {
            r: (s.r - d.r).abs(),
            g: (s.g - d.g).abs(),
            b: (s.b - d.b).abs(),
            a: (s.a - d.a).abs(),
        }
    }

    fn multiply(self, other: Color, modes: SeparateBlendModes) -> Color {
        let (s, d) = alpha_blend_components(self, other, modes);

        Color {
            r: s.r * d.r,
            g: s.g * d.g,
            b: s.b * d.b,
            a: s.a * d.a,
        }
    }

    fn average(self, other: Color, modes: SeparateBlendModes) -> Color {
        let (s, d) = alpha_blend_components(self, other, modes);

        Color {
            r: (s.r + d.r) / 2.0,
            g: (s.g + d.g) / 2.0,
            b: (s.b + d.b) / 2.0,
            a: (s.a + d.a) / 2.0,
        }
    }

    fn over(self, other: Color) -> Color {
        fn over_component(x: f32, y: f32, a: f32, b: f32) -> f32 {
            let a1 = 1.0 - a;
            (x * a + y * b * a1) / (a + b * a1)
        }

        Color {
            r: over_component(self.r, other.r, self.a, other.a),
            g: over_component(self.g, other.g, self.a, other.a),
            b: over_component(self.b, other.b, self.a, other.a),
            a: self.a + other.a * (1.0 - self.a)
        }
    }
}