//! Routines for drawing on a graph

use std::ops::{Deref, DerefMut, Range};

use ::common::color::Color;
use ::common::color::image::Image;

pub mod plot;
pub mod line;
pub mod axis;
pub mod curve;
pub mod function;
pub mod shape;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineStyle {
    Thin,
    ThinAA,
    Thick {
        width: f64,
        hardness: f64,
    },
    ThickAA {
        width: f64,
        hardness: f64,
    }
}

impl LineStyle {
    /// Convenience method for `LineStyle::Thick`
    pub fn thick(width: f64, hardness: f64) -> LineStyle {
        LineStyle::Thick { width: width, hardness: hardness }
    }

    /// Add anti-aliasing to the line
    pub fn aa(self) -> LineStyle {
        match self {
            LineStyle::Thin => LineStyle::ThinAA,
            LineStyle::Thick { width, hardness } => LineStyle::ThickAA { width: width, hardness: hardness },
            _ => self
        }
    }
}

pub trait PixelBlend {
    type Pixel;

    fn blend(Self::Pixel, Self::Pixel) -> Self::Pixel;
}

pub struct BlendOverPixel;

impl PixelBlend for BlendOverPixel {
    type Pixel = Color;

    fn blend(a: Color, b: Color) -> Color {
        use ::common::color::blend::ColorBlend;

        a.under(b)
    }
}

pub trait Plotter {
    type Blend: PixelBlend;

    fn width(&self) -> u32;
    fn height(&self) -> u32;

    fn draw_pixel(&mut self, x: i64, y: i64, alpha: f64);
    fn draw_dot(&mut self, x: i64, y: i64, alpha: f64, width: f64, hardness: f64);
    fn draw_line(&mut self, x0: i64, y0: i64, x1: i64, y1: i64, style: LineStyle);
    fn draw_circle(&mut self, x: i64, y: i64, radius: i64, style: LineStyle);
    fn draw_ellipse(&mut self, x0: i64, y0: i64, x1: i64, y1: i64, style: LineStyle);
    fn draw_rectangle(&mut self, x0: i64, y0: i64, x1: i64, y1: i64, style: LineStyle);
}

pub struct Plot {
    image: Image,
    foreground: Color,
}

impl Plot {
    pub fn new(width: u32, height: u32) -> Plot {
        Plot {
            image: Image::new(width, height),
            foreground: Color::default(),
        }
    }

    pub fn with_background(width: u32, height: u32, background: Color) -> Plot {
        Plot {
            image: Image::with_pixel(width, height, background),
            foreground: Color::default(),
        }
    }

    pub fn set_foreground(&mut self, foreground: Color) -> Color {
        ::std::mem::replace(&mut self.foreground, foreground)
    }

    pub fn foreground(&self) -> Color {
        self.foreground
    }

    pub fn image(&self) -> &Image { &self.image }

    pub fn image_mut(&mut self) -> &mut Image { &mut self.image }

    pub fn into_image(self) -> Image {
        self.image
    }
}

impl Plotter for Plot {
    type Blend = BlendOverPixel;

    fn width(&self) -> u32 { self.image.width() }

    fn height(&self) -> u32 { self.image.height() }

    fn draw_pixel(&mut self, x: i64, y: i64, alpha: f64) {
        use ::common::num_utils::ClampExt;

        if x >= 0 && y >= 0 {
            let x = x as u32;
            let y = y as u32;

            if let Some(p) = self.image.pixel(x, y).cloned() {
                *self.image.pixel_mut(x, y).unwrap() = <Self::Blend as PixelBlend>::blend(p, self.foreground.with_alpha(alpha.clamp(0.0, 1.0) as f32));
            }
        }
    }

    #[inline]
    fn draw_dot(&mut self, x: i64, y: i64, alpha: f64, width: f64, hardness: f64) {
        ::graph::plot::plot_gaussian_dot(x, y, alpha, width, hardness, |x, y, alpha| self.draw_pixel(x, y, alpha))
    }

    #[inline]
    fn draw_line(&mut self, x0: i64, y0: i64, x1: i64, y1: i64, style: LineStyle) {
        match style {
            LineStyle::Thin => {
                line::draw_line_bresenham(x0, y0, x1, y1, |x, y, alpha| {
                    self.draw_pixel(x, y, alpha)
                })
            }
            LineStyle::ThinAA => {
                line::draw_line_xiaolin_wu(x0, y0, x1, y1, |x, y, alpha| {
                    self.draw_pixel(x, y, alpha)
                })
            }
            LineStyle::Thick { width, hardness } => {
                line::draw_line_bresenham(x0, y0, x1, y1, |x, y, alpha| {
                    self.draw_dot(x, y, alpha, width, hardness)
                })
            }
            LineStyle::ThickAA { width, hardness } => {
                line::draw_line_xiaolin_wu(x0, y0, x1, y1, |x, y, alpha| {
                    self.draw_dot(x, y, alpha, width, hardness)
                })
            }
        }
    }

    #[inline]
    fn draw_circle(&mut self, x: i64, y: i64, radius: i64, style: LineStyle) {
        match style {
            LineStyle::Thin => {
                shape::draw_circle(x, y, radius, |x, y, alpha| self.draw_pixel(x, y, alpha))
            }
            LineStyle::ThinAA => {
                shape::draw_circle_aa(x, y, radius, |x, y, alpha| self.draw_pixel(x, y, alpha))
            }
            LineStyle::Thick { width, hardness } => {
                shape::draw_circle(x, y, radius, |x, y, alpha| self.draw_dot(x, y, alpha, width, hardness))
            }
            LineStyle::ThickAA { width, hardness } => {
                shape::draw_circle_aa(x, y, radius, |x, y, alpha| self.draw_dot(x, y, alpha, width, hardness))
            }
        }
    }

    #[inline]
    fn draw_ellipse(&mut self, x0: i64, y0: i64, x1: i64, y1: i64, style: LineStyle) {
        match style {
            LineStyle::Thin => {
                shape::draw_ellipse(x0, y0, x1, y1, |x, y, alpha| self.draw_pixel(x, y, alpha))
            }
            LineStyle::ThinAA => {
                shape::draw_ellipse_aa(x0, y0, x1, y1, |x, y, alpha| self.draw_pixel(x, y, alpha))
            }
            LineStyle::Thick { width, hardness } => {
                shape::draw_ellipse(x0, y0, x1, y1, |x, y, alpha| self.draw_dot(x, y, alpha, width, hardness))
            }
            LineStyle::ThickAA { width, hardness } => {
                shape::draw_ellipse_aa(x0, y0, x1, y1, |x, y, alpha| self.draw_dot(x, y, alpha, width, hardness))
            }
        }
    }

    #[inline]
    fn draw_rectangle(&mut self, x0: i64, y0: i64, x1: i64, y1: i64, style: LineStyle) {
        shape::draw_rectangle(x0, y0, x1, y1, |x0, y0, x1, y1| self.draw_line(x0, y0, x1, y1, style))
    }
}

pub trait RectangularGraph {
    fn linear_equation<F>(&mut self, samples: usize, style: LineStyle, f: F) where F: Fn(f64) -> f64;
    fn parametric_equation<F>(&mut self, t_domain: Range<f64>, samples: usize, style: LineStyle, f: F) where F: Fn(f64) -> (f64, f64);
    fn draw_axis(&mut self, style: LineStyle);
}

pub trait PolarGraph {}

pub struct Graph<P: Plotter> {
    plotter: P,
    x_domain: Range<f64>,
    y_domain: Range<f64>,
}

impl Graph<Plot> {
    pub fn new(width: u32, height: u32, x_domain: Range<f64>, y_domain: Range<f64>) -> Graph<Plot> {
        Graph::with_plotter(Plot::new(width, height), x_domain, y_domain)
    }

    pub fn with_background(width: u32, height: u32, background: Color, x_domain: Range<f64>, y_domain: Range<f64>) -> Graph<Plot> {
        Graph::with_plotter(Plot::with_background(width, height, background), x_domain, y_domain)
    }
}

impl<P: Plotter> Graph<P> {
    pub fn with_plotter(plotter: P, x_domain: Range<f64>, y_domain: Range<f64>) -> Graph<P> {
        Graph {
            plotter: plotter,
            x_domain: x_domain,
            y_domain: y_domain
        }
    }

    pub fn x_domain(&self) -> Range<f64> {
        self.x_domain.clone()
    }

    pub fn y_domain(&self) -> Range<f64> {
        self.y_domain.clone()
    }

    pub fn into_plotter(self) -> P {
        self.plotter
    }
}

impl<P: Plotter> RectangularGraph for Graph<P> {
    fn linear_equation<F>(&mut self, samples: usize, style: LineStyle, f: F) where F: Fn(f64) -> f64 {
        function::graph_linear_equation(self.plotter.width(), self.plotter.height(),
                                        self.x_domain(), self.y_domain(), samples, f,
                                        |x0, y0, x1, y1| self.plotter.draw_line(x0, y0, x1, y1, style))
    }

    fn parametric_equation<F>(&mut self, t_domain: Range<f64>, samples: usize, style: LineStyle, f: F) where F: Fn(f64) -> (f64, f64) {
        function::graph_parametric_equation(self.plotter.width(), self.plotter.height(), t_domain,
                                            self.x_domain(), self.y_domain(), samples, f,
                                            |x0, y0, x1, y1| self.plotter.draw_line(x0, y0, x1, y1, style))
    }

    fn draw_axis(&mut self, style: LineStyle) {
        axis::draw_axis(self.plotter.width(), self.plotter.height(),
                        self.x_domain(), self.y_domain(),
                        |x0, y0, x1, y1| self.plotter.draw_line(x0, y0, x1, y1, style));
    }
}

impl<P: Plotter> PolarGraph for Graph<P> {}

impl<P: Plotter> Deref for Graph<P> {
    type Target = P;

    fn deref(&self) -> &P {
        &self.plotter
    }
}

impl<P: Plotter> DerefMut for Graph<P> {
    fn deref_mut(&mut self) -> &mut P {
        &mut self.plotter
    }
}