use common::num_utils::min_max;

pub fn draw_line_naive<F>(x0: u32, y0: u32, x1: u32, y1: u32, mut plot: F) where F: FnMut(u32, u32, f64) {
    let (xmin, xmax) = min_max(x0, x1);

    let dx = xmax - xmin;
    let dy = y1 - y0;

    let mut x = xmin;

    while x < xmax {
        let y = y0 + dy * (x - x0) / dx;
        plot(x as u32, y as u32, 1.0);

        x += 1;
    }
}

pub fn draw_line_stamp<S>(x0: u32, y0: u32, x1: u32, y1: u32, interval: f64, mut stamp: S) where S: FnMut(u32, u32, f64) {
    let (xmin, xmax) = min_max(x0, x1);

    let dx = xmax - xmin;
    let dy = y1 - y0;

    for x in xmin..xmax {
        let y = y0 + dy * (x - x0) / dx;
        stamp(x as u32, y as u32, 1.0);
    }
}

pub fn draw_line_bresenham<F>(x0: u32, y0: u32, x1: u32, y1: u32, mut plot: F) where F: FnMut(u32, u32, f64) {
    let mut x0 = x0 as i64;
    let mut y0 = y0 as i64;
    let x1 = x1 as i64;
    let y1 = y1 as i64;

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();

    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = dx + dy;

    loop {
        plot(x0 as u32, y0 as u32, 1.0);

        if x0 == x1 && y0 == y1 { break; }

        let e2 = 2 * err;

        if e2 >= dy {
            err += dy;
            x0 += sx;
        }

        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

pub fn draw_line_bresenham_aa<F>(x0: u32, y0: u32, x1: u32, y1: u32, mut plot: F) where F: FnMut(u32, u32, f64) {
    let mut x0 = x0 as i64;
    let mut y0 = y0 as i64;
    let x1 = x1 as i64;
    let y1 = y1 as i64;

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();

    let dxf = dx as f64;
    let dyf = dy as f64;

    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = dx - dy;

    let ed = if dx == 0 && dy == 0 { 1.0 } else { dxf.hypot(dyf) };

    loop {
        plot(x0 as u32, y0 as u32, 1.0 - (err - dx + dy).abs() as f64 / ed);

        let e2 = err;
        let x2 = x0;

        let e22 = 2 * e2;

        if e22 >= -dx {
            if x0 == x1 { break; }

            let a = (e2 + dy) as f64;

            if a < ed {
                plot(x0 as u32, (y0 + sy) as u32, 1.0 - a / ed);
            }

            err -= dy;
            x0 += sx;
        }

        if e22 <= dy {
            if y0 == y1 { break; }

            let a = (dx - e2) as f64;

            if a < ed {
                plot((x2 + sx) as u32, y0 as u32, 1.0 - a / ed);
            }

            err += dx;
            y0 += sy;
        }
    }
}

pub fn draw_line_bresenham_thick_aa<F>(x0: u32, y0: u32, x1: u32, y1: u32, width: f64, mut plot: F) where F: FnMut(u32, u32, f64) {
    if width <= 1.0 {
        draw_line_bresenham_aa(x0, y0, x1, y1, plot);
    } else {
        let mut x0: i64 = x0 as i64;
        let mut y0: i64 = y0 as i64;
        let x1: i64 = x1 as i64;
        let y1: i64 = y1 as i64;

        let dx: i64 = (x1 - x0).abs();
        let dy: i64 = (y1 - y0).abs();

        let sx: i64 = if x0 < x1 { 1 } else { -1 };
        let sy: i64 = if y0 < y1 { 1 } else { -1 };

        let mut err: i64 = dx - dy;

        let ed: f64 = if dx == 0 && dy == 0 { 1.0 } else { (dx as f64).hypot(dy as f64) };

        let wd: f64 = (width + 1.0) / 2.0;

        loop {
            plot(x0 as u32, y0 as u32, 1.0 - ((err - dx + dy).abs() as f64 / ed - wd + 1.0).max(0.0));

            let mut e2: i64 = err;
            let mut x2: i64 = x0;

            if (2 * e2) >= -dx {
                e2 += dy;

                let mut y2: i64 = y0;

                while (e2 as f64) < (ed * wd) && (y1 != y2 || dx > dy) {
                    y2 += sy;

                    plot(x0 as u32, y2 as u32, 1.0 - (e2.abs() as f64 / ed - wd + 1.0).max(0.0));

                    e2 += dx;
                }

                if x0 == x1 { break; }

                e2 = err;
                err -= dy;
                x0 += sx;
            }

            if (2 * e2) <= dy {
                e2 = dx - e2;

                while (e2 as f64) < (ed * wd) && (x1 != x2 || dx < dy) {
                    x2 += sx;

                    plot(x2 as u32, y0 as u32, 1.0 - (e2.abs() as f64 / ed - wd + 1.0).max(0.0));

                    e2 += dy;
                }

                if y0 == y1 { break; }

                err += dx;
                y0 += sy;
            }
        }
    }
}

pub fn draw_line_xiaolin_wu<F>(x0: u32, y0: u32, x1: u32, y1: u32, mut plot: F) where F: FnMut(u32, u32, f64) {
    use std::mem::swap;

    let mut plot_float = |x: f64, y: f64, opacity: f64| {
        plot(x as u32, y as u32, opacity)
    };

    let mut x0 = x0 as f64;
    let mut y0 = y0 as f64;
    let mut x1 = x1 as f64;
    let mut y1 = y1 as f64;

    let steep = (y1 - y0).abs() > (x1 - x0).abs();

    if steep {
        swap(&mut x0, &mut y0);
        swap(&mut x1, &mut y1);
    }

    if x0 > x1 {
        swap(&mut x0, &mut x1);
        swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0;
    let dy = y1 - y0;

    let gradient = if dx < 0.0001 { 1.0 } else { dy / dx };

    let xend = x0.round();
    let yend = y0 + gradient * (xend - x0);

    let xgap = 1.0 - (x0 + 0.5).fract();

    let xpxl1 = xend;
    let ypxl1 = yend.trunc();

    if steep {
        plot_float(ypxl1, xpxl1, (1.0 - yend.fract()) * xgap);
        plot_float(ypxl1 + 1.0, xpxl1, yend.fract() * xgap);
    } else {
        plot_float(xpxl1, ypxl1, (1.0 - yend.fract()) * xgap);
        plot_float(xpxl1, ypxl1 + 1.0, yend.fract() * xgap);
    }

    let mut intery = yend + gradient;

    let xend = x1.round();
    let yend = y1 + gradient * (xend - x1);
    let xgap = (x1 + 0.5).fract();

    let xpxl2 = xend;
    let ypxl2 = yend.trunc();

    if steep {
        plot_float(ypxl2, xpxl2, (1.0 - yend.fract()) * xgap);
        plot_float(ypxl2 + 1.0, xpxl2, yend.fract() * xgap);
    } else {
        plot_float(xpxl2, ypxl2, (1.0 - yend.fract()) * xgap);
        plot_float(xpxl2, ypxl2 + 1.0, yend.fract() * xgap);
    }

    let mut x = xpxl1 + 1.0;

    if steep {
        while x <= (xpxl2 - 1.0) {
            let y = intery.trunc();

            plot_float(y, x, 1.0 - intery.fract());
            plot_float(y + 1.0, x, intery.fract());

            intery += gradient;
            x += 1.0;
        }
    } else {
        while x <= (xpxl2 - 1.0) {
            let y = intery.trunc();

            plot_float(x, y, 1.0 - intery.fract());
            plot_float(x, y + 1.0, intery.fract());

            intery += gradient;
            x += 1.0;
        }
    }
}

pub fn draw_line_xiaolin_wu2<F>(x0: u32, y0: u32, x1: u32, y1: u32, mut plot: F) where F: FnMut(u32, u32, f64) {
    use std::mem::swap;

    let mut plot_float = |x: f64, y: f64, opacity: f64| {
        plot(x as u32, y as u32, opacity)
    };

    let mut x0 = x0 as f64;
    let mut y0 = y0 as f64;
    let mut x1 = x1 as f64;
    let mut y1 = y1 as f64;

    let dx = x1 - x0;
    let dy = y1 - y0;

    if dx.abs() > dy.abs() {
        if x1 < x0 {
            swap(&mut x0, &mut x1);
            swap(&mut y0, &mut y1);
        }

        let gradient = dy / dx;

        let xend = x0.round();
        let yend = y0 + gradient * (xend - x0);

        let xgap = 1.0 - (x0 + 0.5).fract();

        let xpxl1 = xend;
        let ypxl1 = yend.trunc();

        plot_float(xpxl1, ypxl1, (1.0 - yend.fract()) * xgap);
        plot_float(xpxl1, ypxl1 + 1.0, yend.fract() * xgap);

        let mut intery = yend + gradient;

        let xend = x1.round();
        let yend = y1 + gradient * (xend - x1);

        let xgap = (x1 + 0.5).fract();

        let xpxl2 = xend;
        let ypxl2 = yend.trunc();

        plot_float(xpxl2, ypxl2, (1.0 - yend.fract()) * xgap);
        plot_float(xpxl2, ypxl2 + 1.0, yend.fract() * xgap);

        let mut x = xpxl1 + 1.0;

        while x <= (xpxl2 - 1.0) {
            let y = intery.trunc();

            plot_float(x, y, 1.0 - intery.fract());
            plot_float(x, y + 1.0, intery.fract());

            intery += gradient;
            x += 1.0;
        }
    } else {
        if x1 < x0 {
            swap(&mut x0, &mut x1);
            swap(&mut y0, &mut y1);
        }

        let gradient = dx / dy;

        let yend = y0.round();
        let xend = x0 + gradient * (yend - y0);

        let ygap = 1.0 - (y0 + 0.5).fract();

        let ypxl1 = yend;
        let xpxl1 = xend.trunc();

        plot_float(xpxl1, ypxl1, (1.0 - xend.fract()) * ygap);
        plot_float(xpxl1, ypxl1 + 1.0, xend.fract() * ygap);

        let mut interx = xend + gradient;

        let yend = y1.round();
        let xend = x1 + gradient * (yend - y1);

        let ygap = (y1 + 0.5).fract();

        let ypxl2 = yend;
        let xpxl2 = xend.trunc();

        plot_float(xpxl2, ypxl2, (1.0 - xend.fract()) * ygap);
        plot_float(xpxl2, ypxl2 + 1.0, xend.fract() * ygap);

        let mut y = ypxl1 + 1.0;

        while y <= (ypxl2 - 1.0) {
            let x = interx.trunc();

            plot_float(x, y, 1.0 - interx.fract());
            plot_float(x + 1.0, y, interx.fract());

            interx += gradient;
            y += 1.0;
        }
    }
}