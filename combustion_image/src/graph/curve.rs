use std::ops::Range;

use ::bezier::BezierCurve;

pub fn draw_bezier_curve<L>(width: u32, height: u32, curve: &BezierCurve,
                            domain_x: Range<f64>, domain_y: Range<f64>, steps: usize,
                            mut draw_line: L) where L: FnMut(i64, i64, i64, i64) {
    ::graph::function::graph_parametric_equation(width, height, 0.0..1.0, domain_x, domain_y, steps,
                                                 |t: f64| -> (f64, f64) { curve.evaluate(t) }, draw_line);
}