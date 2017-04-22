use std::ops::Range;

use common::num_utils::LerpExt;

use super::function::{graph_parametric_equation, graph_linear_equation};

/// Draws two axis vertically and horizontally across the entire x and y domains
pub fn draw_axis<L>(width: u32, height: u32, domain_x: Range<f64>, domain_y: Range<f64>, mut draw_line: L) where L: FnMut(f64, f64, f64, f64) {
    graph_parametric_equation(width, height, 0.0..1.0, domain_x.clone(), domain_y.clone(), 2, |t| (domain_x.start.lerp(t, domain_x.end), 0.0), &mut draw_line);
    graph_parametric_equation(width, height, 0.0..1.0, domain_x.clone(), domain_y.clone(), 2, |t| (0.0, domain_y.start.lerp(t, domain_y.end)), &mut draw_line);
}

pub fn line_intersection() {}

pub fn draw_polar_axis<L>(width: u32, height: u32, domain_x: Range<f64>, domain_y: Range<f64>, divisions: usize, mut draw_line: L) where L: FnMut(f64, f64, f64, f64) {
    use ::std::f64::consts::{PI, FRAC_PI_2};

    let da = ((PI - FRAC_PI_2).abs() * 2.0) / divisions as f64;

    let mut a: f64 = PI;

    while a > FRAC_PI_2 {
        let slope = a.tan();

        graph_linear_equation(width, height, domain_x.clone(), domain_y.clone(), divisions, false, |x| { slope * x }, &mut draw_line);
        graph_linear_equation(width, height, domain_x.clone(), domain_y.clone(), divisions, false, |x| { -slope * x }, &mut draw_line);

        a -= da;
    }
}
