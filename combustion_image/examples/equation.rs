#![allow(unused_imports)]

extern crate combustion_image;
extern crate combustion_common as common;

extern crate image;

use std::cell::Cell;

use image::RgbaImage;

use common::num_utils::ClampExt;

use common::color::Color;
use common::color::blend::ColorBlend;
use common::color::blend::{BlendOp, BlendMode, BlendModes, SeparateBlendModes};
use common::color::blend::PREFER_DESTINATION_BLEND_MODES;
use common::color::tonemap::aces_filmic_tonemap;
use common::color::image::Image;

use combustion_image::stat::gaussian_dot_pdf;

fn main() {
    let background = Color::from_name("white").unwrap();
    let mut foreground = Cell::new(Color::from_name("black").unwrap());

    let (w, h) = (1000, 1000);

    let mut image = Image::with_pixel(w, h, background);

    {
        let mut plot = |x, y, alpha: f64, _: f64| {
            if x >= 0 && y >= 0 {
                let x = x as u32;
                let y = y as u32;

                if let Some(p) = image.pixel(x, y).cloned() {
                    *image.pixel_mut(x, y).unwrap() = p.under(foreground.get().with_alpha(alpha.clamp(0.0, 1.0) as f32));
                }
            }
        };

        let width = 2.0;
        let hardness = 2.0;

        combustion_image::graph::line::draw_line_bresenham(0, (h / 2) as i64, w as i64, (h / 2) as i64, &mut plot);
        combustion_image::graph::line::draw_line_bresenham((w / 2) as i64, 0, (w / 2) as i64, h as i64, &mut plot);

        let x_domain = -10.0..10.0;
        let y_domain = -10.0..10.0;

        let b = combustion_image::bezier::BezierCurve::new(vec![(1.0, 1.0),
                                                                (7.3, 4.4),
                                                                (3.2, 7.4)]);

        combustion_image::graph::curve::draw_bezier_curve(w, h, &b, x_domain.clone(), y_domain.clone(), 100, |x0, y0, x1, y1| {
            combustion_image::graph::line::draw_line_thick_gaussian(x0, y0, x1, y1, width, hardness, &mut plot);
        });

        combustion_image::graph::shape::draw_regular_polygon(w, h, 0.0, -3.0, 3.0, 45.0f64.to_radians(), x_domain.clone(), y_domain.clone(), 3, |x0, y0, x1, y1| {
            combustion_image::graph::line::draw_line_thick_gaussian(x0, y0, x1, y1, 10.0, hardness, &mut plot);
        });

        combustion_image::graph::shape::draw_circle(250, 400, 50, |x, y, a, d| {
            combustion_image::graph::plot::plot_gaussian_dot(x, y, a, d, 5.0, hardness, &mut plot);
        });

        combustion_image::graph::shape::draw_ellipse(100, 100, 400, 300, |x, y, a, d| {
            combustion_image::graph::plot::plot_gaussian_dot(x, y, a, d, 5.0, hardness, &mut plot);
        });

        let func = |x: f64| -> f64 {
            //x.sin() + (x * x).sin()
            2.0f64.powf(x.sin())
        };

        foreground.set(Color::from_name("red").unwrap());

        combustion_image::graph::function::graph_linear_equation(w, h, x_domain.clone(), y_domain.clone(), w as usize, func, |x0, y0, x1, y1| {
            combustion_image::graph::line::draw_line_thick_gaussian(x0, y0, x1, y1, width, hardness, &mut plot);
        });

        let func = |x: f64| -> f64 {
            //x.sin() + (x * x).sin()
            -2.0f64.powf(-x.sin())
        };

        foreground.set(Color::from_name("blue").unwrap());

        combustion_image::graph::function::graph_linear_equation(w, h, x_domain.clone(), y_domain.clone(), w as usize, func, |x0, y0, x1, y1| {
            combustion_image::graph::line::draw_line_thick_gaussian(x0, y0, x1, y1, width, hardness, &mut plot);
        });

        let func = |x: f64| -> f64 {
            x.sin() + (x * x).sin()
        };

        foreground.set(Color::from_name("green").unwrap());

        combustion_image::graph::function::graph_linear_equation(w, h, x_domain.clone(), y_domain.clone(), w as usize, func, |x0, y0, x1, y1| {
            combustion_image::graph::line::draw_line_thick_gaussian(x0, y0, x1, y1, width, hardness, &mut plot);
        });
    }

    RgbaImage::from_raw(image.width(), image.height(),
                        image.into_u8_component_vec())
        .unwrap()
        .save("test.png")
        .unwrap();
}