use std::ops::Range;

use common::num_utils::min_max;

use super::function::graph_polar_equation;

/// Draw any n-sided regular polygon
pub fn draw_regular_polygon<L>(width: u32, height: u32, x: f64, y: f64, radius: f64, rotation: f64,
                               domain_x: Range<f64>, domain_y: Range<f64>, sides: usize, draw_line: L) where L: FnMut(i64, i64, i64, i64) {
    graph_polar_equation(width, height, x, y, rotation, domain_x, domain_y, sides, |_: f64| radius, draw_line);
}

/// Draw's a smooth circle using the midpoint circle algorithm
pub fn draw_circle<P>(xm: i64, ym: i64, mut radius: i64, mut plot: P) where P: FnMut(i64, i64, f64) {
    let mut x = -radius;
    let mut y = 0;
    let mut err = 2 - 2 * radius;

    loop {
        plot(xm - x, ym + y, 1.0); /*   I. Quadrant */
        plot(xm - y, ym - x, 1.0); /*  II. Quadrant */
        plot(xm + x, ym - y, 1.0); /* III. Quadrant */
        plot(xm + y, ym + x, 1.0); /*  IV. Quadrant */

        radius = err;

        if radius <= y {
            y += 1;
            err += y * 2 + 1;
        }

        if radius > x || err > y {
            x += 1;
            err += x * 2 + 1;
        }

        if x >= 0 { break; }
    }
}

pub fn draw_circle_aa<P>(xm: i64, ym: i64, mut radius: i64, mut plot: P) where P: FnMut(i64, i64, f64) {
    let mut x = radius;
    let mut y = 0;
    let mut err = 2 - 2 * radius;
    radius = 1 - err;

    plot(xm, ym + (radius / 2 + 1), 1.0);
    plot(xm, ym - (radius / 2 + 1), 1.0);
    plot(xm + (radius / 2 + 1), ym, 1.0);
    plot(xm - (radius / 2 + 1), ym, 1.0);

    let mut plot = |x, y, alpha| {
        if !(x == xm || y == ym) {
            plot(x, y, alpha);
        }
    };

    loop {
        let alpha = 1.0 - (err + 2 * (x + y) - 2).abs() as f64 / radius as f64;

        plot(xm + x, ym - y, alpha); /*   I. Quadrant */
        plot(xm + y, ym + x, alpha); /*  II. Quadrant */
        plot(xm - x, ym + y, alpha); /* III. Quadrant */
        plot(xm - y, ym - x, alpha); /*  IV. Quadrant */

        if x == 0 { break; }

        let e2 = err;
        let mut x2 = x;

        if e2 > y {
            let alpha = 1.0 - (err + 2 * x - 1) as f64 / radius as f64;

            if alpha > 0.0 {
                plot(xm + x, ym - y + 1, alpha);
                plot(xm + y - 1, ym + x, alpha);
                plot(xm - x, ym + y - 1, alpha);
                plot(xm - y + 1, ym - x, alpha);
            }

            x -= 1;
            err -= x * 2 - 1;
        }

        if e2 <= {
            let tmp = x2;
            x2 -= 1;
            tmp
        } {
            let alpha = 1.0 - (1 - 2 * y - e2) as f64 / radius as f64;

            if alpha > 0.0 {
                plot(xm + x2, ym - y, alpha);
                plot(xm + y, ym + x2, alpha);
                plot(xm - x2, ym + y, alpha);
                plot(xm - y, ym - x2, alpha);
            }

            y -= 1;
            err -= y * 2 - 1;
        }
    }
}

/// Draw's a smooth ellipse using the midpoint circle algorithm
pub fn draw_ellipse<P>(mut x0: i64, mut y0: i64, mut x1: i64, mut y1: i64, mut plot: P) where P: FnMut(i64, i64, f64) {
    let mut a = (x1 - x0).abs();
    let b = (y1 - x0).abs();

    if a == 0 || b == 0 {
        super::line::draw_line_bresenham(x0, y0, x1, y1, plot);
    } else {
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
            plot(x1, y0, 1.0); /*   I. Quadrant */
            plot(x0, y0, 1.0); /*  II. Quadrant */
            plot(x0, y1, 1.0); /* III. Quadrant */
            plot(x1, y1, 1.0); /*  IV. Quadrant */

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
            plot(x0 - 1, y0, 1.0);
            plot(x1 + 1, y0, 1.0);
            plot(x0 - 1, y1, 1.0);
            plot(x1 + 1, y1, 1.0);

            y0 += 1;
            y1 -= 1;
        }
    }
}

pub fn draw_ellipse_aa<P>(mut x0: i64, mut y0: i64, mut x1: i64, mut y1: i64, mut plot: P) where P: FnMut(i64, i64, f64) {
    let mut a = (x1 - x0).abs();
    let b = (y1 - y0).abs();

    if a == 0 || b == 0 {
        super::line::draw_line_xiaolin_wu(x0, y0, x1, y1, plot);
    } else {
        let aa = (x1 + x0) / 2;
        let bb = (y1 + y0) / 2;

        plot(aa, y0, 1.0);
        plot(x0, bb, 1.0);
        plot(aa, y1, 1.0);
        plot(x1, bb, 1.0);

        let mut plot = |x, y, alpha| {
            if !(x == aa || y == bb) {
                plot(x, y, alpha);
            }
        };

        // 1 if odd, 0 if even
        let mut b1 = b & 0x1;

        let mut dx = 4 * (a - 1) * b * b;
        let mut dy = 4 * (b1 + 1) * a * a;

        let mut err = b1 * a * a - dx + dy;

        if x0 > x1 {
            x0 = x1;
            x1 += a;
        }

        if y0 > y1 {
            y0 = y1;
        }

        y0 += (b + 1) >> 1;

        y1 = y0 - b1;

        a = 8 * a * a;
        b1 = 8 * b * b;

        loop {
            let (i, ed) = min_max(dx, dy);

            let ed: f64 = if y0 == (y1 + 1) && err > dy && a > b1 {
                4.0 / a as f64
            } else {
                let (ed, i) = (ed as f64, i as f64);

                1.0 / (ed + 2.0 * ed * i * i / (4.0 * ed * ed + i * i))
            };

            let alpha: f64 = 1.0 - ed * (err + dx - dy).abs() as f64;

            plot(x0, y0, alpha);
            plot(x0, y1, alpha);
            plot(x1, y0, alpha);
            plot(x1, y1, alpha);

            let f = 2 * err + dy;

            if f >= 0 {
                if x0 == x1 { break; }

                let alpha = 1.0 - ed * (err + dx) as f64;

                if alpha > 0.0 {
                    plot(x0, y0 + 1, alpha);
                    plot(x0, y1 - 1, alpha);
                    plot(x1, y0 + 1, alpha);
                    plot(x1, y1 - 1, alpha);
                }
            }

            if 2 * err <= dx {
                let alpha = 1.0 - ed * (dy - err) as f64;

                if alpha > 0.0 {
                    plot(x0 + 1, y0, alpha);
                    plot(x1 - 1, y0, alpha);
                    plot(x0 + 1, y1, alpha);
                    plot(x1 - 1, y1, alpha);
                }

                y0 += 1;
                y1 -= 1;
                dy += a;
                err += dy;
            }

            if f > 0 {
                x0 += 1;
                x1 -= 1;
                dx -= b1;
                err -= dx;
            }
        }

        x0 -= 1;

        if x0 == {
            let tmp = x1;
            x1 += 1;
            tmp
        } {
            while (y0 - y1) < b {
                let alpha = 1.0 - 4.0 * (err + dx).abs() as f64 / b1 as f64;

                y0 += 1;

                plot(x0, y0, alpha);
                plot(x1, y0, alpha);

                y1 -= 1;

                plot(x0, y1, alpha);
                plot(x1, y1, alpha);

                dy += a;
                err += dy;
            }
        }
    }
}

/// Draws a rectangle at the points given
pub fn draw_rectangle<L>(x0: i64, y0: i64, x1: i64, y1: i64, mut draw_line: L) where L: FnMut(i64, i64, i64, i64) {
    draw_line(x0, y0, x1, y0);
    draw_line(x0, y0, x0, y1);
    draw_line(x1, y0, x1, y1);
    draw_line(x0, y1, x1, y1);
}