use std::ops::Range;

use common::num_utils::{ScaleExt, ClampExt};

use ::geometry::Intersection;

/// Graphs a linear equation.
///
/// [https://en.wikipedia.org/wiki/Linear_equation](https://en.wikipedia.org/wiki/Linear_equation)
pub fn graph_linear_equation<F, L>(width: u32, height: u32, domain_x: Range<f64>, domain_y: Range<f64>,
                                   steps: usize, break_discontinuous: bool, f: F, mut draw_line: L) where F: Fn(f64) -> f64,
                                                                                                          L: FnMut(f64, f64, f64, f64) {
    let (w, h) = (width as f64, height as f64);

    let dx = (domain_x.end - domain_x.start) / steps as f64;
    let dy = (domain_y.end - domain_x.start).abs();

    let mut x0 = domain_x.start;

    // Get the first finite value as a starting point
    let mut y0 = {
        let mut y;

        // Keep trying until a finite value is created.
        loop {
            y = f(x0);

            if y.is_finite() {
                break;
            } else if x0 < domain_x.end {
                // If the value was not finite,
                // we can continue on the function
                x0 += dx;
            } else {
                // If there are no more inputs in the domain before we even start,
                // we're done here.
                return;
            }
        }

        y
    };

    // Get the next input value
    let mut x1 = x0 + dx;

    loop {
        if x1 > domain_x.end { break };

        let y1 = f(x1);

        let continuous = y1.is_finite();

        if continuous {
            // If either y-value if inside the domains, we'll graph the function,
            // otherwise the whole line segment would be
            // above or below the graphable space.
            if domain_y.contains(y0) || domain_y.contains(y1) ||
                (y0 < domain_y.start && y1 >= domain_y.end) ||
                (y1 < domain_y.start && y0 >= domain_y.end) {
                let mut px0 = x0.scale(domain_x.start, domain_x.end, 0.0, width as f64);
                let mut py0 = y0.scale(domain_y.start, domain_y.end, 0.0, height as f64);
                let mut px1 = x1.scale(domain_x.start, domain_x.end, 0.0, width as f64);
                let mut py1 = y1.scale(domain_y.start, domain_y.end, 0.0, height as f64);

                // If the difference between these is greater than the vertical space of the domain, consider it to be vertical
                let vertical = if (y0 - y1).abs() > dy as f64 {
                    true
                } else {
                    false
                };

                if !(break_discontinuous && vertical) {
                    // if any y pixel values are outside the plot area
                    if !(0.0 <= py1 && py0 <= h && 0.0 <= py1 && py1 <= h) {
                        if vertical {
                            // vertical lines are literally just clamped at the plot height
                            py0 = py0.clamp(0.0, h);
                            py1 = py1.clamp(0.0, h);
                        } else {
                            // Top border
                            match Intersection::line_line(px0, py0, px1, py1, 0.0, h, w, h) {
                                Intersection::Intersection(x, y) => {
                                    // if it's point 2 that is above the border, clamp it,
                                    // otherwise it's the other point
                                    if py1 > h {
                                        px1 = x;
                                        py1 = y;
                                    } else {
                                        px0 = x;
                                        py0 = y;
                                    }
                                }
                                _ => ()
                            }

                            match Intersection::line_line(px0, py0, px1, py1, 0.0, 0.0, w, 0.0) {
                                Intersection::Intersection(x, y) => {
                                    // if it's point 2 that is below the border, clamp it,
                                    // otherwise it's the other point
                                    if py1 < 0.0 {
                                        px1 = x;
                                        py1 = y;
                                    } else {
                                        px0 = x;
                                        py0 = y;
                                    }
                                }
                                _ => ()
                            }

                            // left and right borders do not need to be checked
                            // because the input x value is a known variable
                        }
                    }

                    draw_line(px0, py0, px1, py1);
                }
            }
        }

        if x1 == domain_x.end { break };

        if continuous || break_discontinuous {
            x0 = x1;
            y0 = y1;
        }

        let x1dx = x1 + dx;

        x1 = if x1dx > domain_x.end { domain_x.end } else { x1dx };
    }
}

/// Graphs a parametric equation.
///
/// [https://en.wikipedia.org/wiki/Parametric_equation](https://en.wikipedia.org/wiki/Parametric_equation)
pub fn graph_parametric_equation<F, L>(width: u32, height: u32, domain_t: Range<f64>, domain_x: Range<f64>, domain_y: Range<f64>, steps: usize, f: F, mut draw_line: L) where F: Fn(f64) -> (f64, f64),
                                                                                                                                                                              L: FnMut(f64, f64, f64, f64) {
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

        //TODO: For very long lines, find their intersection with the image border and only draw to that.

        if on_graph {
            let px0 = x0.scale(domain_x.start, domain_x.end, 0.0, width as f64);
            let py0 = y0.scale(domain_y.start, domain_y.end, 0.0, height as f64);

            let px1 = x1.scale(domain_x.start, domain_x.end, 0.0, width as f64);
            let py1 = y1.scale(domain_y.start, domain_y.end, 0.0, height as f64);

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
pub fn graph_polar_equation<F, L>(width: u32, height: u32, domain_a: Range<f64>, domain_x: Range<f64>, domain_y: Range<f64>, steps: usize, f: F, draw_line: L) where F: Fn(f64) -> f64,
                                                                                                                                                                     L: FnMut(f64, f64, f64, f64) {
    graph_parametric_equation(width, height, domain_a, domain_x, domain_y, steps, |angle: f64| {
        let r = f(angle);

        (r * (angle).cos(), r * (angle).sin())
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