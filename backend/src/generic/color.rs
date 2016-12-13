//! Flexible Color type suitable for using as a shader uniform

use std::ops::Deref;

use num_traits::float::Float;
use nalgebra::Vector4;

use palette::{Color as PaletteColor, Luma, Rgb, Rgba, Xyz, Yxy, Lab, Lch, Hsv, Hsl, Hwb, Alpha};
use palette::white_point::WhitePoint;
use palette::pixel::RgbPixel;

/// Reexported types from the Palette library. Most of these can be converted into a `Color` instance easily and automatically.
pub mod palette {
    pub use palette::{Color as PaletteColor, Luma, Rgb, Rgba, Xyz, Yxy, Lab, Lch, Hsv, Hsl, Hwb, Alpha};
    pub use palette::white_point::WhitePoint;
    pub use palette::pixel::RgbPixel;
}

/// C structure to store RGBA color information, suitable for using as a shader uniform
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Color {
    /// Red component
    pub r: f32,
    /// Green component
    pub g: f32,
    /// Blue component
    pub b: f32,
    /// Alpha component
    pub a: f32
}

impl Color {
    #[inline(always)]
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { r: r, g: g, b: b, a: a }
    }

    #[inline(always)]
    pub fn from_tuple(rgba: (f32, f32, f32, f32)) -> Color {
        Color::new(rgba.0, rgba.1, rgba.2, rgba.3)
    }

    /// Create a black transparent color
    #[inline(always)]
    pub fn none() -> Color {
        Color::new(0.0, 0.0, 0.0, 0.0)
    }
}

impl Default for Color {
    #[inline(always)]
    fn default() -> Color { Color::none() }
}

impl From<Color> for Vector4<f32> {
    #[inline(always)]
    fn from(color: Color) -> Vector4<f32> {
        Vector4::new(color.r, color.g, color.b, color.a)
    }
}

macro_rules! impl_from_color {
    ($($color:ident),+) => {
        $(
            impl<Wp, T> From<$color<Wp, T>> for Color where T: Float, Wp: WhitePoint<T> {
                #[inline(always)]
                fn from(other: $color<Wp, T>) -> Color {
                    Color::from_tuple(Rgb::from(other).to_pixel::<(f32, f32, f32, f32)>().to_rgba())
                }
            }

            //Convert Alpha types, too
            impl<Wp, T> From<Alpha<$color<Wp, T>, T>> for Color where T: Float, Wp: WhitePoint<T> {
                #[inline(always)]
                fn from(other: Alpha<$color<Wp, T>, T>) -> Color {
                    Color::from_tuple(Rgba::from(other).to_pixel::<(f32, f32, f32, f32)>().to_rgba())
                }
            }
        )+
    }
}

//Convert from Palette color types to Color
impl_from_color!(PaletteColor, Luma, Rgb, Xyz, Yxy, Lab, Lch, Hsv, Hsl, Hwb);

macro_rules! impl_from_pixel {
    ($pixel_ty:ty) => {
        impl From<$pixel_ty> for Color where $pixel_ty: RgbPixel<f32> {
            #[inline(always)]
            fn from(other: $pixel_ty) -> Color {
                Color::from_tuple(other.to_rgba())
            }
        }
    }
}

//Convert from primitive pixel types to Color
impl_from_pixel!((f64, f64, f64, f64));
impl_from_pixel!((f32, f32, f32, f32));
impl_from_pixel!((u8,  u8,  u8,  u8));
impl_from_pixel!((f64, f64, f64));
impl_from_pixel!((f32, f32, f32));
impl_from_pixel!((u8,  u8,  u8));
impl_from_pixel!([f64; 4]);
impl_from_pixel!([f32; 4]);
impl_from_pixel!([u8;  4]);
impl_from_pixel!([f64; 3]);
impl_from_pixel!([f32; 3]);
impl_from_pixel!([u8;  3]);
