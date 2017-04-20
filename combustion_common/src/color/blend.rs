//! Blend operations for colors

#![allow(missing_docs, unused_imports)]

use ::num_utils::{LerpExt, LerpGenericExt, ClampExt, min_max};

use super::Color;
use super::ext::ColorExt;

/// Blend modes for alpha blending.
///
/// These are as close to identical to OpenGL's blend modes as possible.
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum BlendMode {
    /// C * 0
    Zero,
    /// C * 1
    One,
    /// C * sC
    SourceColor,
    /// C * (1 - sC)
    OneMinusSourceColor,
    /// C * dC
    DestinationColor,
    /// C * (1 - dC)
    OneMinusDestinationColor,
    /// C * sA
    SourceAlpha,
    /// C * (1 - sA)
    OneMinusSourceAlpha,
    /// C * dA
    DestinationAlpha,
    /// C * (1 - dA)
    OneMinusDestinationAlpha,
}

impl BlendMode {
    fn apply_alpha(&self, x: f32, sc: f32, dc: f32, sa: f32, da: f32) -> f32 {
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

/// Separate blend modes for both source and destination colors
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct BlendModes { pub source: BlendMode, pub destination: BlendMode }

/// Separate blend modes for color and alpha channels
pub struct SeparateBlendModes { pub color: BlendModes, pub alpha: BlendModes }

impl Default for SeparateBlendModes {
    /// Returns `DEFAULT_BLEND_MODES`
    fn default() -> SeparateBlendModes { DEFAULT_BLEND_MODES }
}

/// Standard blending modes that only multiple the colors by their alphas
pub const DEFAULT_BLEND_MODES: SeparateBlendModes = SeparateBlendModes {
    color: BlendModes {
        source: BlendMode::SourceAlpha,
        destination: BlendMode::DestinationAlpha,
    },
    alpha: BlendModes {
        source: BlendMode::One,
        destination: BlendMode::One,
    }
};

/// Blend modes that used the source alpha for blending
pub const PREFER_SOURCE_BLEND_MODES: SeparateBlendModes = SeparateBlendModes {
    color: BlendModes {
        source: BlendMode::SourceAlpha,
        destination: BlendMode::OneMinusSourceAlpha,
    },
    alpha: BlendModes {
        source: BlendMode::One,
        destination: BlendMode::One,
    }
};

/// The opposite of `PREFER_SOURCE_BLEND_MODES`,
/// this uses the destination alpha for blending.
pub const PREFER_DESTINATION_BLEND_MODES: SeparateBlendModes = SeparateBlendModes {
    color: BlendModes {
        source: BlendMode::OneMinusDestinationAlpha,
        destination: BlendMode::DestinationAlpha,
    },
    alpha: BlendModes {
        source: BlendMode::One,
        destination: BlendMode::One,
    }
};

/// Ignore alpha blending
pub const IGNORE_ALPHA_BLEND_MODES: SeparateBlendModes = SeparateBlendModes {
    color: BlendModes {
        source: BlendMode::One,
        destination: BlendMode::One,
    },
    alpha: BlendModes {
        source: BlendMode::One,
        destination: BlendMode::One,
    }
};

/// Standard blend operations for colors.
///
/// These generally match all functions provided by the `ColorBlend` trait, with the exception of `over`.
///
/// Use these values with the `complex` method on `ColorBlend` to blend color and alpha channels separately.
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum BlendOp {
    Normal,
    Add,
    Subtract,
    Difference,
    Multiply,
    Average,
    Negate,
    Exclusion,
    Lighten,
    Darken,
    Screen,
    Overlay,
    ColorDodge,
    ColorBurn,
    LinearDodge,
    LinearBurn,
    Phoenix,
}

/// Color blending extensions
///
/// All of these take into account alpha blending,
/// so if the results are not as expected,
/// check your alpha values and blending options.
pub trait ColorBlend: Sized {
    /// Perform a combination of blend operations for color and alpha channels separately.
    fn complex(self, color_op: BlendOp, alpha_op: BlendOp, other: Color, modes: SeparateBlendModes) -> Color;

    fn normal(self, other: Color, modes: SeparateBlendModes) -> Color;

    fn add(self, other: Color, modes: SeparateBlendModes) -> Color;
    fn subtract(self, other: Color, modes: SeparateBlendModes) -> Color;
    fn difference(self, other: Color, modes: SeparateBlendModes) -> Color;
    fn multiply(self, other: Color, modes: SeparateBlendModes) -> Color;
    fn average(self, other: Color, modes: SeparateBlendModes) -> Color;
    fn negate(self, other: Color, modes: SeparateBlendModes) -> Color;
    fn exclusion(self, other: Color, modes: SeparateBlendModes) -> Color;

    fn lighten(self, other: Color, modes: SeparateBlendModes) -> Color;
    fn darken(self, other: Color, modes: SeparateBlendModes) -> Color;

    fn screen(self, other: Color, modes: SeparateBlendModes) -> Color;
    fn overlay(self, other: Color, modes: SeparateBlendModes) -> Color;

    fn color_dodge(self, other: Color, modes: SeparateBlendModes) -> Color;
    fn color_burn(self, other: Color, modes: SeparateBlendModes) -> Color;

    fn linear_dodge(self, other: Color, modes: SeparateBlendModes) -> Color {
        self.add(other, modes)
    }

    fn linear_burn(self, other: Color, modes: SeparateBlendModes) -> Color {
        self.subtract(other, modes)
    }

    fn phoenix(self, other: Color, modes: SeparateBlendModes) -> Color;

    /// "Over" operation taken from [https://en.wikipedia.org/wiki/Alpha_compositing](https://en.wikipedia.org/wiki/Alpha_compositing)
    fn over(self, other: Color) -> Color;

    /// Short for `other.over(self)`
    fn under(self, other: Color) -> Color;
}

fn alpha_blend_components(source: Color, destination: Color, modes: SeparateBlendModes) -> (Color, Color) {
    let alpha_blended_source = Color {
        r: modes.color.source.apply_alpha(source.r, source.r, destination.r, source.a, destination.a),
        g: modes.color.source.apply_alpha(source.g, source.g, destination.g, source.a, destination.a),
        b: modes.color.source.apply_alpha(source.b, source.b, destination.b, source.a, destination.a),
        a: modes.alpha.source.apply_alpha(source.a, source.a, destination.a, source.a, destination.a)
                             .clamp(0.0, 1.0),
    };

    let alpha_blended_destination = Color {
        r: modes.color.destination.apply_alpha(destination.r, source.r, destination.r, source.a, destination.a),
        g: modes.color.destination.apply_alpha(destination.g, source.g, destination.g, source.a, destination.a),
        b: modes.color.destination.apply_alpha(destination.b, source.b, destination.b, source.a, destination.a),
        a: modes.alpha.destination.apply_alpha(destination.a, source.a, destination.a, source.a, destination.a)
                                  .clamp(0.0, 1.0),
    };

    (alpha_blended_source, alpha_blended_destination)
}

/// More complex blend ops that are best put into their own functions
mod blend_ops {
    // Simple functions get #[inline]
    // Simple functions with no conditionals get #[inline(always)]
    // Complex functions with conditionals get no inlining

    #[inline]
    pub fn subtract_component(source: f32, destination: f32) -> f32 {
        (source + destination - 1.0).max(0.0)
    }

    #[inline]
    pub fn negate_component(source: f32, destination: f32) -> f32 {
        (1.0 - (1.0 - source - destination).abs()).abs()
    }

    #[inline(always)]
    pub fn exclusion_component(source: f32, destination: f32) -> f32 {
        source + destination - 2.0 * source * destination
    }

    #[inline(always)]
    pub fn screen_component(source: f32, destination: f32) -> f32 {
        1.0 - ((1.0 - source) * (1.0 - destination))
    }

    pub fn overlay_component(source: f32, destination: f32) -> f32 {
        if source < 0.5 {
            2.0 * source * destination
        } else {
            1.0 - 2.0 * (1.0 - source) * (1.0 - destination)
        }
    }

    pub fn color_dodge_component(source: f32, destination: f32) -> f32 {
        if destination == 1.0 { destination } else {
            source / (1.0 - destination)
        }
    }

    pub fn color_burn_component(source: f32, destination: f32) -> f32 {
        if destination == 0.0 { destination } else {
            (1.0 - ((1.0 - source) / destination)).min(0.0)
        }
    }
}

#[inline]
fn blend_component(source: f32, destination: f32, op: BlendOp) -> f32 {
    match op {
        BlendOp::Normal => { destination }
        BlendOp::Add |
        BlendOp::LinearDodge => { source + destination }
        BlendOp::Subtract |
        BlendOp::LinearBurn => { blend_ops::subtract_component(source, destination) }
        BlendOp::Difference => { (source - destination).abs() }
        BlendOp::Multiply => { source * destination }
        BlendOp::Average => { (source + destination) / 2.0 }
        BlendOp::Negate => { blend_ops::negate_component(source, destination) }
        BlendOp::Exclusion => { blend_ops::exclusion_component(source, destination) }
        BlendOp::Lighten => { source.max(destination) }
        BlendOp::Darken => { source.min(destination) }
        BlendOp::Screen => { blend_ops::screen_component(source, destination) }
        BlendOp::Overlay => { blend_ops::overlay_component(source, destination) }
        BlendOp::ColorDodge => { blend_ops::color_dodge_component(source, destination) }
        BlendOp::ColorBurn => { blend_ops::color_burn_component(source, destination) }
        BlendOp::Phoenix => {
            let (min, max) = min_max(source, destination);

            min - max + 1.0
        }
    }
}

impl ColorBlend for Color {
    #[inline]
    fn complex(self, color_op: BlendOp, alpha_op: BlendOp, other: Color, modes: SeparateBlendModes) -> Color {
        let (s, d) = alpha_blend_components(self, other, modes);

        Color {
            r: blend_component(s.r, d.r, color_op),
            g: blend_component(s.g, d.g, color_op),
            b: blend_component(s.b, d.b, color_op),
            a: blend_component(s.a, d.a, alpha_op),
        }
    }

    fn normal(self, other: Color, modes: SeparateBlendModes) -> Color {
        let (_, d) = alpha_blend_components(self, other, modes);

        d
    }

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
            r: blend_ops::subtract_component(s.r, d.r),
            g: blend_ops::subtract_component(s.g, d.g),
            b: blend_ops::subtract_component(s.b, d.b),
            a: blend_ops::subtract_component(s.a, d.a),
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

    fn negate(self, other: Color, modes: SeparateBlendModes) -> Color {
        let (s, d) = alpha_blend_components(self, other, modes);

        Color {
            r: blend_ops::negate_component(s.r, d.r),
            g: blend_ops::negate_component(s.g, d.g),
            b: blend_ops::negate_component(s.b, d.b),
            a: blend_ops::negate_component(s.a, d.a),
        }
    }

    fn exclusion(self, other: Color, modes: SeparateBlendModes) -> Color {
        let (s, d) = alpha_blend_components(self, other, modes);

        Color {
            r: blend_ops::exclusion_component(s.r, d.r),
            g: blend_ops::exclusion_component(s.g, d.g),
            b: blend_ops::exclusion_component(s.b, d.b),
            a: blend_ops::exclusion_component(s.a, d.a),
        }
    }

    fn lighten(self, other: Color, modes: SeparateBlendModes) -> Color {
        let (s, d) = alpha_blend_components(self, other, modes);

        Color {
            r: s.r.max(d.r),
            g: s.g.max(d.g),
            b: s.b.max(d.b),
            a: s.a.max(d.a),
        }
    }

    fn darken(self, other: Color, modes: SeparateBlendModes) -> Color {
        let (s, d) = alpha_blend_components(self, other, modes);

        Color {
            r: s.r.min(d.r),
            g: s.g.min(d.g),
            b: s.b.min(d.b),
            a: s.a.min(d.a),
        }
    }

    fn screen(self, other: Color, modes: SeparateBlendModes) -> Color {
        let (s, d) = alpha_blend_components(self, other, modes);

        Color {
            r: blend_ops::screen_component(s.r, d.r),
            g: blend_ops::screen_component(s.g, d.g),
            b: blend_ops::screen_component(s.b, d.b),
            a: blend_ops::screen_component(s.a, d.a),
        }
    }

    fn overlay(self, other: Color, modes: SeparateBlendModes) -> Color {
        let (s, d) = alpha_blend_components(self, other, modes);

        Color {
            r: blend_ops::overlay_component(s.r, d.r),
            g: blend_ops::overlay_component(s.g, d.g),
            b: blend_ops::overlay_component(s.b, d.b),
            a: blend_ops::overlay_component(s.a, d.a),
        }
    }

    fn color_dodge(self, other: Color, modes: SeparateBlendModes) -> Color {
        let (s, d) = alpha_blend_components(self, other, modes);

        Color {
            r: blend_ops::color_dodge_component(s.r, d.r),
            g: blend_ops::color_dodge_component(s.g, d.g),
            b: blend_ops::color_dodge_component(s.b, d.b),
            a: blend_ops::color_dodge_component(s.a, d.a),
        }
    }

    fn color_burn(self, other: Color, modes: SeparateBlendModes) -> Color {
        let (s, d) = alpha_blend_components(self, other, modes);

        Color {
            r: blend_ops::color_burn_component(s.r, d.r),
            g: blend_ops::color_burn_component(s.g, d.g),
            b: blend_ops::color_burn_component(s.b, d.b),
            a: blend_ops::color_burn_component(s.a, d.a),
        }
    }

    fn phoenix(self, other: Color, modes: SeparateBlendModes) -> Color {
        let (s, d) = alpha_blend_components(self, other, modes);

        let (rmin, rmax) = min_max(s.r, d.r);
        let (gmin, gmax) = min_max(s.g, d.g);
        let (bmin, bmax) = min_max(s.b, d.b);
        let (amin, amax) = min_max(s.a, d.a);

        Color {
            r: rmin - rmax + 1.0,
            g: gmin - gmax + 1.0,
            b: bmin - bmax + 1.0,
            a: amin - amax + 1.0,
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

    fn under(self, other: Color) -> Color {
        other.over(self)
    }
}