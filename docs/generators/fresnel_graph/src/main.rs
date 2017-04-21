#![feature(step_by)]

extern crate image;
extern crate rusttype;
extern crate rayon;

extern crate combustion_common as common;
extern crate combustion_graphing as graphing;

use std::cell::Cell;

use image::RgbaImage;

use common::num_utils::ClampExt;

use common::color::Color;
use common::color::blend::ColorBlend;
use common::color::image::Image;

pub mod tables;
pub mod wavelength;
pub mod calculus;
pub mod fresnel;
pub mod weighted;

pub use self::fresnel::{fresnel, fresnel_schlick};

/*
use std::ops::Neg;
use std::io::Write;

use rayon::prelude::*;

use image::{RgbImage, Rgb, Pixel, ImageBuffer};

use palette::{Hsv, RgbHue, IntoColor};

use rusttype::{FontCollection, Scale, point, PositionedGlyph};

use self::utils::scale;

pub enum Wavelength {
    Red,
    Green,
    Blue
}

type FloatRgbImage = ImageBuffer<Rgb<f64>, Vec<f64>>;
*/

pub fn wavelength_reflectance(cos_theta: f64, eta_i: f64, rgb_response: weighted::RGBResponse) -> (f64, f64, f64) {
    let (red_response, green_response, blue_response) = rgb_response;

    let (eta, k) = red_response;
    let (rs, rp) = fresnel(cos_theta, eta, k, eta_i);
    let r_red = (rs + rp) / 2.0;

    let (eta, k) = green_response;
    let (rs, rp) = fresnel(cos_theta, eta, k, eta_i);
    let r_green = (rs + rp) / 2.0;

    let (eta, k) = blue_response;
    let (rs, rp) = fresnel(cos_theta, eta, k, eta_i);
    let r_blue = (rs + rp) / 2.0;

    (r_red, r_green, r_blue)
}

fn main() {
    let background = Color::from_name("white").unwrap();
    let foreground = Cell::new(Color::from_name("black").unwrap());

    let (w, h) = (1000, 1000);

    let mut image = Image::with_pixel(w, h, background);

    {
        let mut plot = |x, y, alpha: f64| {
            if x >= 0 && y >= 0 {
                let x = x as u32;
                let y = y as u32;

                if let Some(p) = image.pixel(x, y).cloned() {
                    *image.pixel_mut(x, y).unwrap() = p.under(foreground.get().with_alpha(alpha.clamp(0.0, 1.0) as f32));
                }
            }
        };

        let width = 1.5;
        let hardness = 2.0;

        let x_domain = -0.25..1.0;
        let y_domain = -0.25..1.0;

        let samples = 750;

        let n_t = 1.45;
        let k_t = 0.0;
        let n_i = 1.0;

        graphing::graph::axis::draw_axis(w, h, x_domain.clone(), y_domain.clone(), |x0, y0, x1, y1| {
            graphing::graph::line::draw_line_bresenham(x0, y0, x1, y1, &mut plot);
        });

        let mut draw_thick_line = |x0, y0, x1, y1| {
            graphing::graph::line::draw_line_thick_gaussian(x0, y0, x1, y1, width, hardness, &mut plot);
        };

        foreground.set(Color::from_name("blue").unwrap());

        graphing::graph::function::graph_linear_equation(w, h, x_domain.clone(), y_domain.clone(), samples, |x| {
            let cos_theta = (x * 90.0).to_radians().cos();

            let (rs, rp) = fresnel(cos_theta, n_t, k_t, n_i);

            (rs + rp) / 2.0
        }, &mut draw_thick_line);

        foreground.set(Color::from_name("red").unwrap());

        graphing::graph::function::graph_linear_equation(w, h, x_domain.clone(), y_domain.clone(), samples, |x| {
            let cos_theta = (x * 90.0).to_radians().cos();

            fresnel_schlick(cos_theta, n_t, n_i)
        }, &mut draw_thick_line);
    }

    RgbaImage::from_raw(image.width(), image.height(),
                        image.into_u8_component_vec())
        .unwrap()
        .save("test.png")
        .unwrap();
}

/*
fn main2() {
    let font_data = include_bytes!("Arial Unicode.ttf");
    let collection = FontCollection::from_bytes(font_data as &[u8]);
    let font = collection.into_font().unwrap(); // only succeeds if collection consists of one font

    // Desired font pixel height
    let height: f32 = 60.0; // to get 80 chars across (fits most terminals); adjust as desired
    let pixel_height = height.ceil() as usize;

    // 2x scale in x direction to counter the aspect ratio of monospace characters.
    let font_scale = Scale { x: height, y: height };

    // The origin of a line of text is at the baseline (roughly where non-descending letters sit).
    // We don't want to clip the text, so we shift it down with an offset when laying it out.
    // v_metrics.ascent is the distance between the baseline and the highest edge of any glyph in
    // the font. That's enough to guarantee that there's no clipping.
    let v_metrics = font.v_metrics(font_scale);
    let offset = point(0.0, v_metrics.ascent);

    let rgb_response = weighted_wavelength_response(tables::GOLD_IOR_TABLE, 1024);

    let frames: usize = 60 * 6;

    (0..frames + 1).into_par_iter().for_each(|i| {
        let theta = (i as f64 / frames as f64) * 90.0;
        let cos_theta = theta.to_radians().cos();

        let mut image = make_graphic(2000, 1000, |x, y| {
            let wavelength = scale(x, 0.0, 1.0, BLUE_WAVELENGTH_MIN, RED_WAVELENGTH_MAX);

            let (eta, k) = tables::get_ior(tables::GOLD_IOR_TABLE, wavelength).unwrap();
            let (rs, rp) = fresnel(cos_theta, eta, k, 1.0);
            let r = ((rs + rp) / 2.0);

            let (red, green, blue) = wavelength::w_to_color(wavelength);

            if (r - y).abs() < 0.0025 {
                (1.0, 1.0, 1.0)
            } else {
                let r = r.powi(2);

                (red * r, green * r, blue * r)
            }
        });

        // Glyphs to draw for "RustType". Feel free to try other strings.
        let glyphs: Vec<PositionedGlyph> = font.layout(format!("theta = {:.2}°", theta).as_str(), font_scale, offset).collect();

        // Find the most visually pleasing width to display
        let width = glyphs.iter().rev()
                          .filter_map(|g| g.pixel_bounding_box().map(|b| b.min.x as f32 + g.unpositioned().h_metrics().advance_width))
                          .next().unwrap_or(0.0).ceil() as usize;

        for g in glyphs {
            if let Some(bb) = g.pixel_bounding_box() {
                g.draw(|x, y, v| {
                    let x = (x as i32 + bb.min.x) as u32 + (image.width() - 500);
                    let y = (y as i32 + bb.min.y) as u32 + (image.height() - 500);

                    let p = image.get_pixel(x, y).map(|subpixel| {
                        subpixel.saturating_add((v * 255.0) as u8)
                    });

                    *image.get_pixel_mut(x, y) = p;
                });
            }
        }

        image.save(format!("animated/frame{}.png", i + 1)).unwrap();

        println!("Finished frame {}", i + 1);
    });
}
*/