use std::ops::Range;

use common::num_utils::ScaleExt;

/// Graphs a linear equation.
///
/// [https://en.wikipedia.org/wiki/Linear_equation](https://en.wikipedia.org/wiki/Linear_equation)
pub fn graph_linear_equation<F, L>(width: u32, height: u32, domain_x: Range<f64>, domain_y: Range<f64>, steps: usize, f: F, mut draw_line: L) where F: Fn(f64) -> f64,
                                                                                                                                                    L: FnMut(i64, i64, i64, i64) {
    let dx = (domain_x.end - domain_x.start) / steps as f64;

    let mut x0 = domain_x.start;
    let mut y0 = f(x0);

    let mut x1 = x0 + dx;

    loop {
        if x1 > domain_x.end { break };

        let y1 = f(x1);

        let on_graph = !(y0.is_nan() || y1.is_nan()) && (domain_y.contains(y0) || domain_y.contains(y1));

        if on_graph {
            let px0 = x0.scale(domain_x.start, domain_x.end, 0.0, width as f64) as i64;
            let py0 = y0.scale(domain_y.start, domain_y.end, 0.0, height as f64) as i64;

            let px1 = x1.scale(domain_x.start, domain_x.end, 0.0, width as f64) as i64;
            let py1 = y1.scale(domain_y.start, domain_y.end, 0.0, height as f64) as i64;

            draw_line(px0, py0, px1, py1);
        }

        if x1 == domain_x.end { break };

        x0 = x1;
        y0 = y1;

        let x1dx = x1 + dx;

        x1 = if x1dx > domain_x.end { domain_x.end } else { x1dx };
    }
}

/// Graphs a parametric equation.
///
/// [https://en.wikipedia.org/wiki/Parametric_equation](https://en.wikipedia.org/wiki/Parametric_equation)
pub fn graph_parametric_equation<F, L>(width: u32, height: u32, domain_t: Range<f64>, domain_x: Range<f64>, domain_y: Range<f64>, steps: usize, f: F, mut draw_line: L) where F: Fn(f64) -> (f64, f64),
                                                                                                                                                                              L: FnMut(i64, i64, i64, i64) {
    let dt = (domain_t.end - domain_t.start) / steps as f64;

    let mut xy0 = f(domain_t.start);

    let mut t1 = domain_t.start + dt;

    loop {
        if t1 > domain_t.end { break };

        let xy1 = f(t1);

        let (x0, y0) = xy0;
        let (x1, y1) = xy1;

        let on_graph = !(x0.is_nan() || y0.is_nan() || x1.is_nan() || y1.is_nan()) &&
            ((domain_x.contains(x0) && domain_y.contains(y0)) ||
                (domain_x.contains(x1) && domain_y.contains(y1)));

        if on_graph {
            let px0 = x0.scale(domain_x.start, domain_x.end, 0.0, width as f64) as i64;
            let py0 = y0.scale(domain_y.start, domain_y.end, 0.0, height as f64) as i64;

            let px1 = x1.scale(domain_x.start, domain_x.end, 0.0, width as f64) as i64;
            let py1 = y1.scale(domain_y.start, domain_y.end, 0.0, height as f64) as i64;

            draw_line(px0, py0, px1, py1);
        }

        if t1 == domain_t.end { break };

        xy0 = xy1;

        let t1dt = t1 + dt;

        t1 = if t1dt > domain_t.end { domain_t.end } else { t1dt };
    }
}

/// Graphs a polar equation.
///
/// [https://en.wikipedia.org/wiki/Polar_coordinate_system](https://en.wikipedia.org/wiki/Polar_coordinate_system)
pub fn graph_polar_equation<F, L>(width: u32, height: u32, x: f64, y: f64, rotation: f64, domain_x: Range<f64>, domain_y: Range<f64>, steps: usize, f: F, draw_line: L) where F: Fn(f64) -> f64,
                                                                                                                                                                              L: FnMut(i64, i64, i64, i64) {
    graph_parametric_equation(width, height, 0.0..1.0, domain_x, domain_y, steps, |t: f64| {
        let angle = t * 2.0 * ::std::f64::consts::PI;

        let r = f(angle);

        (x + r * (angle + rotation).cos(),
         y + r * (angle + rotation).sin())
    }, draw_line);
}

/// WIP, do not use yet
#[allow(unused_variables)]
pub fn graph_planar_equation<F, P>(width: u32, height: u32, domain_k: Range<f64>, domain_x: Range<f64>, domain_y: Range<f64>, x_step: usize, y_step: usize, f: F, mut plot: P) where F: Fn(f64, f64) -> f64,
                                                                                                                                                                                     P: FnMut(i64, i64, f64) {
    let (w, h) = (width as i64, height as i64);
    let (wf, hf) = (width as f64, height as f64);

    let mut x = 0;

    while x < w {
        let mut y = 0;

        while y < h {
            let fx = (x as f64).scale(0.0, wf, domain_x.start, domain_x.end);
            let fy = (y as f64).scale(0.0, hf, domain_y.start, domain_y.end);

            let k = f(fx, fy);

            if k.is_nan() {
                continue;
            }

            let k = k.scale(domain_k.start, domain_k.end, 0.0, 1.0);

            plot(x, y, k);

            y += 1;
        }

        x += 1;
    }

    return;
}