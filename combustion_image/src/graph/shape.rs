use std::ops::Range;

use super::function::graph_parametric_equation;

#[inline(always)]
fn polar_to_parametric(offset: (f64, f64), radius: f64, polar: f64, angle: f64, rotation: f64) -> (f64, f64) {
    let x = offset.0 + radius * polar * (angle + rotation).cos();
    let y = offset.1 + radius * polar * (angle + rotation).sin();

    (x, y)
}

/// Totally not just a polar equation `r = 1`
pub fn draw_regular_polygon<L>(width: u32, height: u32, x: f64, y: f64, radius: f64, rotation: f64, domain_x: Range<f64>, domain_y: Range<f64>, sides: usize, mut draw_line: L) where L: FnMut(i64, i64, i64, i64) {
    graph_parametric_equation(width, height, 0.0..1.0, domain_x, domain_y, sides, |t: f64| {
        let t = t * 2.0 * ::std::f64::consts::PI;

        polar_to_parametric((x, y), radius, 1.0, t, rotation)
    }, draw_line);
}

pub fn draw_circle<P>(x: i64, y: i64, mut radius: i64, mut plot: P) where P: FnMut(i64, i64, f64, f64) {
    let mut x1 = -radius;
    let mut y1 = 0;
    let mut err = 2 - 2 * radius;

    loop {
        plot(x - x1, y + y1, 1.0, 0.0); /*   I. Quadrant */
        plot(x - y1, y - x1, 1.0, 0.0); /*  II. Quadrant */
        plot(x + x1, y - y1, 1.0, 0.0); /* III. Quadrant */
        plot(x + y1, y + x1, 1.0, 0.0); /*  IV. Quadrant */

        radius = err;

        if radius <= y1 {
            y1 += 1;
            err += y1 * 2 + 1;
        }

        if radius > x1 || err > y1 {
            x1 += 1;
            err += x1 * 2 + 1;
        }

        if x1 >= 0 { break; }
    }
}

pub fn draw_ellipse<P>(mut x0: i64, mut y0: i64, mut x1: i64, mut y1: i64, mut plot: P) where P: FnMut(i64, i64, f64, f64) {
    let mut a = (x1 - x0).abs();
    let b = (y1 - x0).abs();

    // 1 if odd, 0 if even
    let mut b1 = b & 0x1;

    let mut dx = 4 * (1 - a) * b * b;
    let mut dy = 4 * (b1 + 1) * a * a;

    let mut err = dx + dy + b1 * a * a;

    if x0 > x1 {
        x0 = x1;
        x1 += a;
    }

    if y0 > y1 {
        y0 = y1;
    }

    y0 += (b + 1) / 2;
    y1 = y0 - b1;

    a *= 8 * a;
    b1 = 8 * b * b;

    loop {
        plot(x1, y0, 1.0, 0.0); /*   I. Quadrant */
        plot(x0, y0, 1.0, 0.0); /*  II. Quadrant */
        plot(x0, y1, 1.0, 0.0); /* III. Quadrant */
        plot(x1, y1, 1.0, 0.0); /*  IV. Quadrant */

        let e2 = 2 * err;

        if e2 <= dy {
            y0 += 1;
            y1 -= 1;
            dy += a;
            err += dy;
        }
        if e2 >= dx || 2 * err > dy {
            x0 += 1;
            x1 -= 1;
            dx += b1;
            err += dx;
        }

        if x0 > x1 { break; }
    }

    while (y0 - y1) < b {
        plot(x0 - 1, y0, 1.0, 0.0);
        plot(x1 + 1, y0, 1.0, 0.0);
        plot(x0 - 1, y1, 1.0, 0.0);
        plot(x1 + 1, y1, 1.0, 0.0);

        y0 += 1;
        y1 -= 1;
    }
}