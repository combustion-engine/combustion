//! Extensions to the `Color` type

use ::num_utils::{LerpExt, LerpGenericExt};

use super::Color;

/// Extensions to the `Color` type
pub trait ColorExt: Sized {
    /// Desaturate the color by the given amount,
    /// where 0.0 is fully colored and 1.0 is fully desaturated
    fn desaturate(self, amount: f32) -> Self;

    /// Modifies Brightness, Contrast and Saturation all at once
    fn brightness_contrast_saturation(self, brightness: f32, contrast: f32, saturation: f32) -> Self;

    /// Modify Brightness
    fn brightness(self, brightness: f32) -> Self {
        self.brightness_contrast_saturation(brightness, 1.0, 1.0)
    }

    /// Modify Contrast
    fn contrast(self, contrast: f32) -> Self {
        self.brightness_contrast_saturation(1.0, contrast, 1.0)
    }

    /// Modify Saturation
    fn saturation(self, saturation: f32) -> Self {
        self.brightness_contrast_saturation(1.0, 1.0, saturation)
    }
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

    fn brightness_contrast_saturation(self, brightness: f32, contrast: f32, saturation: f32) -> Color {
        const AVG_LUM_R: f32 = 0.5;
        const AVG_LUM_G: f32 = 0.5;
        const AVG_LUM_B: f32 = 0.5;

        const LUM_COEFF: Color = Color { r: 0.2125, g: 0.7154, b: 0.0721, a: 1.0 };

        let bright_color = self * brightness;

        let intensity = self.r * LUM_COEFF.r + self.g + LUM_COEFF.g + self.b * LUM_COEFF.b;

        let saturated_color = Color {
            r: intensity.lerp(bright_color.r, saturation),
            g: intensity.lerp(bright_color.g, saturation),
            b: intensity.lerp(bright_color.b, saturation),
            a: self.a,
        };

        let contrast_color = Color {
            r: AVG_LUM_R.lerp(saturated_color.r, contrast),
            g: AVG_LUM_G.lerp(saturated_color.g, contrast),
            b: AVG_LUM_B.lerp(saturated_color.b, contrast),
            a: self.a,
        };

        contrast_color
    }
}