//! Bare-bones image structure

use std::fmt::{Debug, Formatter, Result as FmtResult};

use super::Color;

/// Bare-bones image structure
#[derive(Clone)]
pub struct Image {
    data: Vec<Color>,
    width: u32,
    height: u32,
}

impl Debug for Image {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Image {{width: {}, height: {}}}", self.width, self.height)
    }
}

impl Image {
    /// Create new `Image`
    pub fn new(width: u32, height: u32) -> Image {
        Image::with_pixel(width, height, Color::black())
    }

    /// Create new `Image` with a default pixel color
    pub fn with_pixel(width: u32, height: u32, pixel: Color) -> Image {
        let len = width * height;

        Image {
            data: vec![pixel; len as usize],
            width: width,
            height: height,
        }
    }

    /// Image width
    pub fn width(&self) -> u32 { self.width }

    /// Image height
    pub fn height(&self) -> u32 { self.height }

    /// Get immutable reference to a pixel
    pub fn pixel(&self, x: u32, y: u32) -> Option<&Color> {
        if x < self.width && y < self.height {
            let i = x + self.width * (self.height - y - 1);

            Some(&self.data[i as usize])
        } else {
            None
        }
    }

    /// Get mutable reference to a pixel
    pub fn pixel_mut(&mut self, x: u32, y: u32) -> Option<&mut Color> {
        if x < self.width && y < self.height {
            let i = x + self.width * (self.height - y - 1);

            Some(&mut self.data[i as usize])
        } else {
            None
        }
    }

    /// Map a function to all pixels in an image to return a new image
    pub fn map<F>(self, f: F) -> Image where F: Fn(Color) -> Color {
        Image {
            data: self.data.into_iter().map(f).collect(),
            ..self
        }
    }

    /// Apply a function to all pixels in an image in-place
    pub fn apply<F>(&mut self, f: F) where F: Fn(&mut Color) {
        for mut pixel in &mut self.data {
            f(pixel)
        }
    }

    /// Convert into `Vec<Color>`
    pub fn into_vec(self) -> Vec<Color> {
        self.data
    }

    /// Convert into vector of every subpixel converted into `u8`
    pub fn into_u8_component_vec(self) -> Vec<u8> {
        let mut res = Vec::with_capacity(self.data.len() * 4);

        for color in &self.data {
            res.push((color.r * 255.0) as u8);
            res.push((color.g * 255.0) as u8);
            res.push((color.b * 255.0) as u8);
            res.push((color.a * 255.0) as u8);
        }

        res
    }
}