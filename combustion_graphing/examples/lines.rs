#![allow(unused_imports)]

extern crate combustion_graphing as graphing;
extern crate combustion_common as common;

extern crate image;

use common::num_utils::ClampExt;

use common::color::Color;
use common::color::blend::ColorBlend;
use common::color::blend::{BlendOp, BlendMode, BlendModes, SeparateBlendModes};
use common::color::blend::PREFER_DESTINATION_BLEND_MODES;
use common::color::tonemap::aces_filmic_tonemap;
use common::color::image::Image;

use image::RgbaImage;

fn main() {
    let background = Color::from_name("purple").unwrap();
    let foreground = Color::from_name("yellow").unwrap();

    let mut image = Image::with_pixel(1000, 1000, background);

    {
        let mut plot = |x, y, alpha: f64| {
            let p: Color = *image.pixel(x, y).expect("Invalid pixel index");

            *image.pixel_mut(x, y).unwrap() = p.under(foreground.with_alpha(alpha.clamp(0.0, 1.0) as f32));
        };

        graphing::graph::line::draw_line_bresenham(50, 300, 950, 950, &mut plot);

        graphing::graph::line::draw_line_bresenham_aa(50, 400, 950, 850, &mut plot);
        graphing::graph::line::draw_line_xiaolin_wu2(50, 500, 950, 740, &mut plot);
        graphing::graph::line::draw_line_xiaolin_wu2(50, 740, 950, 500, &mut plot);

        // These are effectively the same
        graphing::graph::line::draw_line_bresenham_aa(400, 20, 500, 950, &mut plot);
        graphing::graph::line::draw_line_xiaolin_wu(405, 20, 505, 950, &mut plot);

        // Comparing versions, even though they're the same
        graphing::graph::line::draw_line_xiaolin_wu2(300, 20, 600, 950, &mut plot);
        graphing::graph::line::draw_line_xiaolin_wu(305, 20, 605, 950, &mut plot);
    }

    RgbaImage::from_raw(image.width(), image.height(),
                        image//.map(|color| aces_filmic_tonemap(color, 1.0))
                             .into_u8_component_vec())
        .unwrap()
        .save("test.png")
        .unwrap();
}