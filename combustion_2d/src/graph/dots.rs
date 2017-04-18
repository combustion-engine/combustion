use ::stat::gaussian_dot_pdf;

pub fn gaussian_pdf_dot<F>(x: u32, y: u32, width: f64, hardness: f64, strenth: f64, mut plot: F) where F: FnMut(u32, u32, f64) {
    let x0: i64 = x as i64;
    let y0: i64 = y as i64;

    let mut x: i64 = 0;
    let mut y: i64 = 0;

    let mut do_plot = |x, y, i| {
        if x >= 0 && y >= 0 {
            plot(x as u32, y as u32, i);
        }
    };

    loop {
        loop {
            let d = (x as f64).hypot(y as f64);

            let f = gaussian_dot_pdf(d, width, hardness);

            if f < 0.01 { break; }

            do_plot(x0 + x, y0 + y, strenth);
            do_plot(x0 + x, y0 - y, strenth);
            do_plot(x0 - x, y0 + y, strenth);
            do_plot(x0 - x, y0 - y, strenth);

            y += 1;
        }

        x += 1;
    }
}