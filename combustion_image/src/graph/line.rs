use common::num_utils::min_max;

pub fn draw_line_naive<F>(x0: u32, y0: u32, x1: u32, y1: u32, mut plot: F) where F: FnMut(u32, u32, f64) {
    let (xmin, xmax) = min_max(x0, x1);

    let dx = xmax - xmin;
    let dy = y1 - y0;

    for x in xmin..xmax {
        let y = y0 + dy * (x - x0) / dx;
        plot(x as u32, y as u32, 1.0);
    }
}

pub fn draw_line_pdf<F>(x0: u32, y0: u32, x1: u32, y1: u32, mut plot: F) where F: FnMut(u32, u32, f64) {
    let (xmin, xmax) = min_max(x0, x1);

    let dx = xmax - xmin;
    let dy = y1 - y0;

    for x in xmin..xmax {
        let y = y0 + dy * (x - x0) / dx;
        plot(x as u32, y as u32, 1.0);
    }
}

pub fn draw_line_bresenham<F>(x0: u32, y0: u32, x1: u32, y1: u32, mut plot: F) where F: FnMut(u32, u32, f64) {
    let mut x0 = x0 as i64;
    let mut y0 = y0 as i64;
    let x1 = x1 as i64;
    let y1 = y1 as i64;

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();

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
        let aa = 1.0 - (err - dx + dy).abs() as f64 / ed;

        plot(x0 as u32, y0 as u32, aa);

        let e2 = err;

        let e22 = 2 * e2;

        if e22 >= -dx {
            if x0 == x1 { break; }

            let a = (e2 + dy) as f64;

            if a < ed {
                let aa = 1.0 - a / ed;

                plot(x0 as u32, (y0 + sy) as u32, aa);
            }

            err -= dy;
            x0 += sx;
        }

        if e22 <= dy {
            if y0 == y1 { break; }

            let a = (dx - e2) as f64;

            if a < ed {
                let aa = 1.0 - a / ed;

                plot((x0 + sx) as u32, y0 as u32, aa);
            }

            err += dx;
            y0 += sy;
        }
    }
}