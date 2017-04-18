//! Tonemapping functions for HDR colors

use super::Color;

/// Defines a filmic tonemap with curve values
#[derive(Debug, Clone, Copy)]
pub struct FilmicTonemap {
    shoulder_strength: f32,
    linear_strength: f32,
    linear_angle: f32,
    toe_strength: f32,
    toe_numerator: f32,
    toe_denominator: f32,
}

/// Tonemap using in Uncharted 2 as found here [http://filmicgames.com/archives/75](http://filmicgames.com/archives/75)
pub const FILMIC_UNCHARTED2_TONEMAP: FilmicTonemap = FilmicTonemap {
    shoulder_strength: 0.15,
    linear_strength: 0.50,
    linear_angle: 0.10,
    toe_strength: 0.20,
    toe_numerator: 0.02,
    toe_denominator: 0.30,
};

/// Another Filmic tonemap found somewhere
pub const FILMIC_REFERENCE_TONEMAP: FilmicTonemap = FilmicTonemap {
    shoulder_strength: 0.22,
    linear_strength: 0.30,
    linear_angle: 0.10,
    toe_strength: 0.20,
    toe_numerator: 0.01,
    toe_denominator: 0.30,
};

fn filmic_tonemap_component(x: f32, tonemap: &FilmicTonemap) -> f32 {
    let a = tonemap.shoulder_strength;
    let b = tonemap.linear_strength;
    let c = tonemap.linear_angle;
    let d = tonemap.toe_strength;
    let e = tonemap.toe_numerator;
    let f = tonemap.toe_denominator;

    ((x * (a * x + c * b) + d * e) / (x * (a * x + b) + d * f)) - (e / f)
}

/// Maps an HDR color to a linear color using the Filmic tonemapping equation and some predefined tonemap
pub fn filmic_tonemap(color: Color, tonemap: &FilmicTonemap) -> Color {
    Color {
        r: filmic_tonemap_component(color.r, tonemap),
        g: filmic_tonemap_component(color.g, tonemap),
        b: filmic_tonemap_component(color.b, tonemap),
        a: color.a
    }
}

/// Simple Reinhard tonemapping
pub fn reinhard_tonemap(color: Color) -> Color {
    Color {
        r: color.r / (color.r + 1.0),
        g: color.g / (color.g + 1.0),
        b: color.b / (color.b + 1.0),
        a: color.a
    }
}

/// Exposure-based tonemapping
pub fn exposure_tonemap(color: Color, exposure: f32) -> Color {
    Color {
        r: 1.0 - (-color.r * exposure).exp(),
        g: 1.0 - (-color.g * exposure).exp(),
        b: 1.0 - (-color.b * exposure).exp(),
        a: color.a
    }
}

/// Whitepoint used in Uncharted 2
pub const UNCHARTED_2_WHITEPOINT: f32 = 11.2;

/// Variation of the Filmic tonemap that supports variable exposure
pub fn filmic_exposure_tonemap(color: Color, exposure: f32, white_point: f32, tonemap: &FilmicTonemap) -> Color {
    const HARD_BIAS: f32 = 16.0;

    Color {
        r: filmic_tonemap_component(color.r * HARD_BIAS * exposure, tonemap) / white_point,
        g: filmic_tonemap_component(color.g * HARD_BIAS * exposure, tonemap) / white_point,
        b: filmic_tonemap_component(color.b * HARD_BIAS * exposure, tonemap) / white_point,
        a: color.a
    }
}

fn aces_filmic_tonemap_component(x: f32) -> f32 {
    const A: f32 = 2.51;
    const B: f32 = 0.03;
    const C: f32 = 2.43;
    const D: f32 = 0.59;
    const E: f32 = 0.14;

    (x * (A * x + B)) / (x * (C * x + D) + E)
}

/// ACES Tonemap from [https://knarkowicz.wordpress.com/2016/01/06/aces-filmic-tone-mapping-curve/](https://knarkowicz.wordpress.com/2016/01/06/aces-filmic-tone-mapping-curve/)
pub fn aces_filmic_tonemap(color: Color, exposure: f32) -> Color {
    Color {
        r: aces_filmic_tonemap_component(color.r * exposure),
        g: aces_filmic_tonemap_component(color.g * exposure),
        b: aces_filmic_tonemap_component(color.b * exposure),
        a: color.a
    }.clamp()
}