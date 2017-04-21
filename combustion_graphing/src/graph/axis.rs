use std::ops::Range;

use common::num_utils::LerpExt;

use super::function::graph_parametric_equation;

pub fn draw_axis<L>(width: u32, height: u32, domain_x: Range<f64>, domain_y: Range<f64>, mut draw_line: L) where L: FnMut(i64, i64, i64, i64) {
    graph_parametric_equation(width, height, 0.0..1.0, domain_x.clone(), domain_y.clone(), 2, |t| (domain_x.start.lerp(t, domain_x.end), 0.0), &mut draw_line);
    graph_parametric_equation(width, height, 0.0..1.0, domain_x.clone(), domain_y.clone(), 2, |t| (0.0, domain_y.start.lerp(t, domain_y.end)), &mut draw_line);
}