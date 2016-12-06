use std::ops::Deref;

use num_traits::float::Float;

use palette::*;
use palette::white_point::WhitePoint;
use palette::pixel::RgbPixel;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct GLColor {
    /// Red component
    pub r: f32,
    /// Green component
    pub g: f32,
    /// Blue component
    pub b: f32,
    /// Alpha component
    pub a: f32
}

impl GLColor {
    #[inline(always)]
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> GLColor {
        GLColor { r: r, g: g, b: b, a: a }
    }

    #[inline(always)]
    pub fn from_tuple(rgba: (f32, f32, f32, f32)) -> GLColor {
        GLColor::new(rgba.0, rgba.1, rgba.2, rgba.3)
    }

    #[inline(always)]
    pub fn none() -> GLColor {
        GLColor::new(0.0, 0.0, 0.0, 0.0)
    }
}

impl Default for GLColor {
    #[inline(always)]
    fn default() -> GLColor { GLColor::none() }
}

macro_rules! impl_from_color {
    ($($color:ident),+) => {
        $(
            impl<Wp, T> From<$color<Wp, T>> for GLColor where T: Float, Wp: WhitePoint<T> {
                #[inline(always)]
                fn from(other: $color<Wp, T>) -> GLColor {
                    GLColor::from_tuple(Rgb::from(other).to_pixel::<(f32, f32, f32, f32)>().to_rgba())
                }
            }

            //Convert Alpha types, too
            impl<Wp, T> From<Alpha<$color<Wp, T>, T>> for GLColor where T: Float, Wp: WhitePoint<T> {
                #[inline(always)]
                fn from(other: Alpha<$color<Wp, T>, T>) -> GLColor {
                    GLColor::from_tuple(Rgba::from(other).to_pixel::<(f32, f32, f32, f32)>().to_rgba())
                }
            }
        )+
    }
}

//Convert from Palette color types to GLColor
impl_from_color!(Color, Luma, Rgb, Xyz, Yxy, Lab, Lch, Hsv, Hsl, Hwb);

macro_rules! impl_from_pixel {
    ($pixel_ty:ty) => {
        impl From<$pixel_ty> for GLColor where $pixel_ty: RgbPixel<f32> {
            #[inline(always)]
            fn from(other: $pixel_ty) -> GLColor {
                GLColor::from_tuple(other.to_rgba())
            }
        }
    }
}

//Convert from primitive pixel types to GLColor
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
