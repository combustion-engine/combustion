use ::stat::gaussian_dot_pdf;

pub fn plot_gaussian_dot<P>(x: i64, y: i64, alpha: f64, distance: f64, width: f64, hardness: f64, mut plot: P) where P: FnMut(i64, i64, f64, f64) {
    let mut x0: i64 = 0;

    loop {
        let mut y0: i64 = 0;

        loop {
            let d = (x0 as f64).hypot(y0 as f64);

            let f = gaussian_dot_pdf(d, width, hardness);

            if x0 == 0 {
                plot(x, y + y0, f * alpha, distance);
                plot(x, y - y0, f * alpha, distance);
            } else if y0 == 0 {
                plot(x + x0, y, f * alpha, distance);
                plot(x - x0, y, f * alpha, distance);
            } else {
                plot(x + x0, y + y0, f * alpha, distance);
                plot(x + x0, y - y0, f * alpha, distance);
                plot(x - x0, y + y0, f * alpha, distance);
                plot(x - x0, y - y0, f * alpha, distance);
            }

            if f < 0.001 { break; }

            y0 += 1;
        }

        if gaussian_dot_pdf(x0 as f64, width, hardness) < 0.001 { break; }

        x0 += 1;
    }
}