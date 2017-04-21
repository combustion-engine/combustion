#![allow(unused_imports)]

extern crate combustion_graphing as graphing;
extern crate combustion_common as common;

extern crate image;

use image::RgbaImage;
use common::color::Color;

use graphing::graph::{LineStyle, Graph, RectangularGraph, Plotter};

fn main() {
    let mut graph = Graph::with_background(1000, 1000, Color::white(), -10.0..10.0, -10.0..10.0);

    graph.set_foreground(Color::black());

    graph.draw_axis(LineStyle::Thin);

    graph.set_foreground(Color::from_name("red").unwrap());

    graph.linear_equation(1000, LineStyle::thick(2.0, 2.0), |x| {
        2.0f64.powf(x.sin())
    });

    graph.set_foreground(Color::from_name("blue").unwrap());

    graph.linear_equation(1000, LineStyle::thick(2.0, 2.0), |x| {
        -2.0f64.powf(-x.sin())
    });

    graph.set_foreground(Color::from_name("green").unwrap());

    graph.linear_equation(1000, LineStyle::thick(2.0, 2.0), |x| {
        x.sin() + (x * x).sin()
    });

    let image = graph.into_plotter().into_image();

    RgbaImage::from_raw(image.width(), image.height(),
                        image.into_u8_component_vec())
        .unwrap()
        .save("test.png")
        .unwrap();
}