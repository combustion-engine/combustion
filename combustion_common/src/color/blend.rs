//! Blend operations for colors

use ::num_utils::LerpGenericExt;

use super::Color;

/// Extensions to the `Color` type
pub trait ColorExt {
    /// Desaturate the color by the given amount,
    /// where 0.0 is fully colored and 1.0 is fully desaturated
    fn desaturate(self, amount: f32) -> Self;
}

impl ColorExt for Color {
    fn desaturate(self, amount: f32) -> Color {
        let brightness = self.r * 0.3 + self.g * 0.59 + self.b * 0.11;

        let gray = Color {
            r: brightness,
            g: brightness,
            b: brightness,
            a: self.a,
        };

        self.lerp(gray, amount)
    }
}