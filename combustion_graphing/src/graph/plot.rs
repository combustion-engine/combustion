use ::stat::gaussian_dot_pdf;

const ALPHA_LIMIT: f64 = 1.0 / 255.0;

pub fn plot_gaussian_dot<P>(x: i64, y: i64, alpha: f64, width: f64, hardness: f64, mut plot: P) where P: FnMut(i64, i64, f64) {
    let mut x0: i64 = 0;

    loop {
        let mut y0: i64 = 0;

        loop {
            let d = (x0 as f64).hypot(y0 as f64);

            let f = gaussian_dot_pdf(d, width, hardness);

            if x0 == 0 {
                plot(x, y + y0, f * alpha);
                plot(x, y - y0, f * alpha);
            } else if y0 == 0 {
                plot(x + x0, y, f * alpha);
                plot(x - x0, y, f * alpha);
            } else {
                plot(x + x0, y + y0, f * alpha);
                plot(x + x0, y - y0, f * alpha);
                plot(x - x0, y + y0, f * alpha);
                plot(x - x0, y - y0, f * alpha);
            }

            if f < ALPHA_LIMIT { break; }

            y0 += 1;
        }

        if gaussian_dot_pdf(x0 as f64, width, hardness) < ALPHA_LIMIT { break; }

        x0 += 1;
    }
}