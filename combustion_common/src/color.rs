//! Flexible Color type suitable for using as a shader uniform

use num_traits::{Zero, One, Float};
use std::f32::EPSILON;
use std::str::FromStr;
use void::Void;

use nalgebra::Vector4;

/// Reexported types from the Palette library. Most of these can be converted into a `Color` instance easily and automatically.
pub mod palette {
    pub use palette::{Color as PaletteColor, Luma, Rgb, Rgba, Xyz, Yxy, Lab, Lch, Hsv, Hsl, Hwb, Alpha};
    pub use palette::white_point::WhitePoint;
    pub use palette::pixel::{RgbPixel, Srgb};
    pub use palette::named::from_str;
    pub use palette::white_point::D65;
}

use self::palette::*;

use utils::AlmostEqExt;

#[inline(always)]
fn is_zero(value: &f32) -> bool {
    value.almost_eq_fast(0.0, 1e-6)
}

#[inline(always)]
fn is_one(value: &f32) -> bool {
    value.almost_eq_fast(1.0, 1e-6)
}

/// C structure to store RGBA color information in linear space, suitable for using as a shader uniform
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Color {
    /// Red component
    #[serde(skip_serializing_if = "is_zero")]
    #[serde(default = "Zero::zero")]
    pub r: f32,
    /// Green component
    #[serde(skip_serializing_if = "is_zero")]
    #[serde(default = "Zero::zero")]
    pub g: f32,
    /// Blue component
    #[serde(skip_serializing_if = "is_zero")]
    #[serde(default = "Zero::zero")]
    pub b: f32,
    /// Alpha (transparency) component
    #[serde(skip_serializing_if = "is_one")]
    #[serde(default = "One::one")]
    pub a: f32
}

impl Color {
    /// Create a new color in linear space
    #[inline(always)]
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { r: r, g: g, b: b, a: a }
    }

    /// Create a new color in linear space from tuple
    #[inline(always)]
    pub fn from_tuple(rgba: (f32, f32, f32, f32)) -> Color {
        Color::new(rgba.0, rgba.1, rgba.2, rgba.3)
    }

    /// Create a new color from its name.
    ///
    /// They are taken from the [SVG keyword colors](https://www.w3.org/TR/SVG/types.html#ColorKeywords) (same as in CSS3)
    pub fn from_name(name: &str) -> Option<Color> {
        from_str(name).map(Into::into)
    }

    /// Same as `from_name`, but returns `Color::none()` if the name wasn't found
    pub fn from_name_or_none(name: &str) -> Color {
        Color::from_name(name).unwrap_or_else(Color::none)
    }

    /// Create a black transparent color
    #[inline(always)]
    pub fn none() -> Color {
        Color::new(0.0, 0.0, 0.0, 0.0)
    }

    /// Create a white opaque color
    #[inline(always)]
    pub fn white() -> Color {
        Color::new(1.0, 1.0, 1.0, 1.0)
    }

    /// Create a black opaque color
    #[inline(always)]
    pub fn black() -> Color {
        Color::new(0.0, 0.0, 0.0, 1.0)
    }

    #[inline(always)]
    pub fn is_opaque(&self) -> bool {
        is_one(&self.a)
    }

    /// Returns true if all the components add up to near-zero
    pub fn is_none(&self) -> bool {
        (self.r + self.g + self.b + self.a) <= EPSILON
    }
}

impl Default for Color {
    #[inline(always)]
    fn default() -> Color {
        Color::none()
    }
}

impl FromStr for Color {
    type Err = Void;

    #[inline(always)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Color::from_name_or_none(s))
    }
}

impl From<String> for Color {
    #[inline(always)]
    fn from(s: String) -> Color {
        Color::from_name_or_none(s.as_str())
    }
}

impl<'a> From<&'a str> for Color {
    #[inline(always)]
    fn from(s: &'a str) -> Color {
        Color::from_name_or_none(s)
    }
}

impl From<Color> for Vector4<f32> {
    #[inline(always)]
    fn from(color: Color) -> Vector4<f32> {
        Vector4::new(color.r, color.g, color.b, color.a)
    }
}

impl From<Color> for Rgba<D65, f32> {
    #[inline(always)]
    fn from(color: Color) -> Rgba<D65, f32> {
        Rgba::new(color.r, color.g, color.b, color.a)
    }
}

impl From<Color> for Rgba<D65, f64> {
    #[inline(always)]
    fn from(color: Color) -> Rgba<D65, f64> {
        Rgba::new(color.r as f64, color.g as f64, color.b as f64, color.a as f64)
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

impl<Wp, T> From<Srgb<Wp, T>> for Color where T: Float, Wp: WhitePoint<T> {
    #[inline(always)]
    fn from(other: Srgb<Wp, T>) -> Color {
        other.to_linear().into()
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

pub mod de {
    //! Custom deserialization for colors, allowing them to be deserialized by name or RGBA values
    use serde::de::{self, Deserialize, Deserializer};
    use void::Void;
    use std::str::FromStr;
    use std::marker::PhantomData;

    pub fn from_name_or_value<T, D>(d: &mut D) -> Result<T, D::Error>
                                    where T: Deserialize + FromStr<Err = Void>,
                                          D: Deserializer {
        struct NameOrValue<T>(PhantomData<T>);

        impl<T> de::Visitor for NameOrValue<T> where T: Deserialize + FromStr<Err = Void> {
            type Value = T;

            fn visit_str<E>(&mut self, value: &str) -> Result<T, E> where E: de::Error {
                Ok(FromStr::from_str(value).unwrap())
            }

            fn visit_map<M>(&mut self, visitor: M) -> Result<T, M::Error> where M: de::MapVisitor {
                let mut mvd = de::value::MapVisitorDeserializer::new(visitor);

                Deserialize::deserialize(&mut mvd)
            }
        }

        d.deserialize(NameOrValue(PhantomData))
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn is_zero_test() {
        assert!(is_zero(&0.0));
        assert!(is_zero(&0.00000001));
    }

    #[test]
    fn is_one_test() {
        assert!(is_one(&1.0));
        assert!(is_one(&0.99999999));
    }

    #[test]
    #[should_panic]
    fn is_zero_test2() {
        assert!(is_zero(&0.001));
    }

    #[test]
    #[should_panic]
    fn is_one_test2() {
        assert!(is_one(&0.999));
    }
}