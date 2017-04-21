#![allow(unused_imports)]

extern crate combustion_graphing as graphing;
extern crate combustion_common as common;

extern crate image;

use image::RgbaImage;
use common::color::Color;

use graphing::graph::{LineStyle, Graph, RectangularGraph, Plotter};

fn main() {
    let mut graph = Graph::with_background(1000, 1000, Color::white(), -10.0..10.0, -10.0..10.0);

    let samples = graph.width() as usize + 1;

    graph.set_foreground(Color::black());

    graph.draw_axis(LineStyle::Thin);

    graph.draw_circle(300, 300, 10, LineStyle::thick(5.0, 1.0).aa());

    graph.draw_ellipse(200, 700, 400, 800, LineStyle::thick(2.0, 1.0).aa());

    graph.set_foreground(Color::from_name("red").unwrap());

    graph.linear_equation(samples, LineStyle::thick(4.0, 1.0), |x| {
        2.0f64.powf(x.sin())
    });

    graph.set_foreground(Color::from_name("blue").unwrap());

    graph.linear_equation(samples, LineStyle::thick(4.0, 1.0), |x| {
        -2.0f64.powf(-x.sin())
    });

    graph.set_foreground(Color::from_name("green").unwrap());

    graph.linear_equation(samples, LineStyle::thick(2.0, 1.0).aa(), |x| {
        x.sin() + (x * x).sin()
    });

    let image = graph.into_plotter().into_image();

    RgbaImage::from_raw(image.width(), image.height(),
                        image.into_u8_component_vec())
        .unwrap()
        .save("test.png")
        .unwrap();
}