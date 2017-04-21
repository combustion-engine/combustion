#![allow(unused_imports)]

extern crate combustion_graphing;
extern crate combustion_common as common;

extern crate image;

use image::RgbaImage;

use common::num_utils::ClampExt;

use common::color::Color;
use common::color::blend::ColorBlend;
use common::color::blend::{BlendOp, BlendMode, BlendModes, SeparateBlendModes};
use common::color::blend::PREFER_DESTINATION_BLEND_MODES;
use common::color::tonemap::aces_filmic_tonemap;
use common::color::image::Image;

use combustion_graphing::stat::gaussian_dot_pdf;

fn main() {
    let background = Color::from_name("purple").unwrap();
    let foreground = Color::from_name("yellow").unwrap();

    let mut image = Image::with_pixel(1000, 1000, background);

    {
        let mut plot = |x, y, alpha: f64, _: f64| {
            if let Some(p) = image.pixel(x, y).cloned() {
                *image.pixel_mut(x, y).unwrap() = p.under(foreground.with_alpha(alpha.clamp(0.0, 1.0) as f32));
            }
        };

        let width = 2.0;
        let hardness = 1.0;

        let func = |x: f64| -> f64 { x.sin() / x };

        combustion_graphing::graph::shape::graph_circle(1000, 1000, -15.0..15.0, -0.25..1.1, 501, func, |x0, y0, x1, y1| {
            combustion_graphing::graph::line::draw_line_thick_gaussian(x0, y0, x1, y1, width, hardness, &mut plot);
        });
    }

    RgbaImage::from_raw(image.width(), image.height(),
                        image.into_u8_component_vec())
        .unwrap()
        .save("test.png")
        .unwrap();
}