//! Thin Color compatibility with the `image` crate

use image::{Pixel, ColorType, Rgb, Rgba, Luma, LumaA};

use super::Color;

/// To avoid polluting the `Color` type with the `Pixel` trait, this struct wraps it.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ColorPixel(pub Color);

impl From<Color> for ColorPixel {
    #[inline(always)]
    fn from(color: Color) -> ColorPixel {
        ColorPixel(color)
    }
}

impl From<ColorPixel> for Color {
    #[inline(always)]
    fn from(pixel: ColorPixel) -> Color {
        pixel.0
    }
}

impl Pixel for ColorPixel {
    type Subpixel = f32;

    #[inline(always)]
    fn channel_count() -> u8 { 4 }

    fn channels(&self) -> &[Self::Subpixel] {
        unsafe { ::std::slice::from_raw_parts(self as *const _ as *const f32, 4) }
    }

    fn channels_mut(&mut self) -> &mut [Self::Subpixel] {
        unsafe { ::std::slice::from_raw_parts_mut(self as *mut _ as *mut f32, 4) }
    }

    #[inline(always)]
    fn color_model() -> &'static str { "RGBA" }

    #[inline(always)]
    fn color_type() -> ColorType {
        ColorType::RGBA(32)
    }

    #[inline(always)]
    fn channels4(&self) -> (f32, f32, f32, f32) {
        self.0.into_tuple()
    }

    #[inline(always)]
    fn from_channels(a: f32, b: f32, c: f32, d: f32) -> ColorPixel {
        ColorPixel(Color::from_tuple((a, b, c, d)))
    }

    fn from_slice<'a>(slice: &'a [Self::Subpixel]) -> &'a Self {
        unsafe { ::std::mem::transmute(&*slice.as_ptr()) }
    }

    fn from_slice_mut<'a>(slice: &'a mut [Self::Subpixel]) -> &'a mut Self {
        unsafe { ::std::mem::transmute(&mut *slice.as_mut_ptr()) }
    }

    fn to_rgb(&self) -> Rgb<Self::Subpixel> {
        Rgb { data: [self.0.r, self.0.g, self.0.b] }
    }

    fn to_rgba(&self) -> Rgba<Self::Subpixel> {
        Rgba { data: [self.0.r, self.0.g, self.0.b, self.0.a] }
    }

    fn to_luma(&self) -> Luma<Self::Subpixel> {
        self.to_rgb().to_luma()
    }

    fn to_luma_alpha(&self) -> LumaA<Self::Subpixel> {
        self.to_rgba().to_luma_alpha()
    }

    fn map<F>(&self, _: F) -> Self where F: Fn(Self::Subpixel) -> Self::Subpixel {
        unimplemented!()
    }
    fn apply<F>(&mut self, _: F) where F: Fn(Self::Subpixel) -> Self::Subpixel {
        unimplemented!()
    }

    fn map_with_alpha<F, G>(&self, _: F, _: G) -> Self where F: Fn(Self::Subpixel) -> Self::Subpixel, G: Fn(Self::Subpixel) -> Self::Subpixel {
        unimplemented!()
    }

    fn apply_with_alpha<F, G>(&mut self, _: F, _: G) where F: Fn(Self::Subpixel) -> Self::Subpixel, G: Fn(Self::Subpixel) -> Self::Subpixel {
        unimplemented!()
    }

    fn map2<F>(&self, _: &Self, _: F) -> Self where F: Fn(Self::Subpixel, Self::Subpixel) -> Self::Subpixel {
        unimplemented!()
    }

    fn apply2<F>(&mut self, _: &Self, _: F) where F: Fn(Self::Subpixel, Self::Subpixel) -> Self::Subpixel {
        unimplemented!()
    }

    fn invert(&mut self) {
        unimplemented!()
    }

    fn blend(&mut self, _: &Self) {
        unimplemented!()
    }
}