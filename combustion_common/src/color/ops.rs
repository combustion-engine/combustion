//! Operator overloads for the `Color` struct
//!
//! Note that NONE of these do any clamping or tonemapping,
//! so they may result in invalid colors if used incorrectly.

use super::Color;

use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: self.a + rhs.a,
        }
    }
}

impl AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self.a += rhs.a;
    }
}

impl Sub<Color> for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Color {
        Color {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
            a: self.a - rhs.a,
        }
    }
}

impl SubAssign<Color> for Color {
    fn sub_assign(&mut self, rhs: Color) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
        self.a -= rhs.a;
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
            a: self.a * rhs.a,
        }
    }
}

impl MulAssign<Color> for Color {
    fn mul_assign(&mut self, rhs: Color) {
        self.r /= rhs.r;
        self.g /= rhs.g;
        self.b /= rhs.b;
        self.a /= rhs.a;
    }
}

impl Div<Color> for Color {
    type Output = Color;

    fn div(self, rhs: Color) -> Color {
        Color {
            r: self.r / rhs.r,
            g: self.g / rhs.g,
            b: self.b / rhs.b,
            a: self.a / rhs.a,
        }
    }
}

impl DivAssign<Color> for Color {
    fn div_assign(&mut self, rhs: Color) {
        self.r /= rhs.r;
        self.g /= rhs.g;
        self.b /= rhs.b;
        self.a /= rhs.a;
    }
}

macro_rules! impl_overloads {
    ($rhs:ty) => {
        impl Add<$rhs> for Color {
            type Output = Color;

            fn add(self, rhs: $rhs) -> Color {
                Color {
                    r: (self.r as $rhs + rhs) as f32,
                    g: (self.g as $rhs + rhs) as f32,
                    b: (self.b as $rhs + rhs) as f32,
                    a: (self.a as $rhs + rhs) as f32,
                }
            }
        }

        impl AddAssign<$rhs> for Color {
            fn add_assign(&mut self, rhs: $rhs) {
                self.r = (self.r as $rhs + rhs) as f32;
                self.g = (self.g as $rhs + rhs) as f32;
                self.b = (self.b as $rhs + rhs) as f32;
                self.a = (self.a as $rhs + rhs) as f32;
            }
        }

        impl Sub<$rhs> for Color {
            type Output = Color;

            fn sub(self, rhs: $rhs) -> Color {
                Color {
                    r: (self.r as $rhs - rhs) as f32,
                    g: (self.g as $rhs - rhs) as f32,
                    b: (self.b as $rhs - rhs) as f32,
                    a: (self.a as $rhs - rhs) as f32,
                }
            }
        }

        impl SubAssign<$rhs> for Color {
            fn sub_assign(&mut self, rhs: $rhs) {
                self.r = (self.r as $rhs - rhs) as f32;
                self.g = (self.g as $rhs - rhs) as f32;
                self.b = (self.b as $rhs - rhs) as f32;
                self.a = (self.a as $rhs - rhs) as f32;
            }
        }

        impl Mul<$rhs> for Color {
            type Output = Color;

            fn mul(self, rhs: $rhs) -> Color {
                Color {
                    r: (self.r as $rhs * rhs) as f32,
                    g: (self.g as $rhs * rhs) as f32,
                    b: (self.b as $rhs * rhs) as f32,
                    a: (self.a as $rhs * rhs) as f32,
                }
            }
        }

        impl MulAssign<$rhs> for Color {
            fn mul_assign(&mut self, rhs: $rhs) {
                self.r = (self.r as $rhs * rhs) as f32;
                self.g = (self.g as $rhs * rhs) as f32;
                self.b = (self.b as $rhs * rhs) as f32;
                self.a = (self.a as $rhs * rhs) as f32;
            }
        }

        impl Div<$rhs> for Color {
            type Output = Color;

            fn div(self, rhs: $rhs) -> Color {
                Color {
                    r: (self.r as $rhs / rhs) as f32,
                    g: (self.g as $rhs / rhs) as f32,
                    b: (self.b as $rhs / rhs) as f32,
                    a: (self.a as $rhs / rhs) as f32,
                }
            }
        }

        impl DivAssign<$rhs> for Color {
            fn div_assign(&mut self, rhs: $rhs) {
                self.r = (self.r as $rhs / rhs) as f32;
                self.g = (self.g as $rhs / rhs) as f32;
                self.b = (self.b as $rhs / rhs) as f32;
                self.a = (self.a as $rhs / rhs) as f32;
            }
        }
    };
}

impl_overloads!(f32);
impl_overloads!(f64);