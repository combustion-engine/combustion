
pub fn simpsons_rule<F>(half_n: u32, a: f64, b: f64, f: F) -> f64 where F: Fn(f64) -> f64 {
    let n = half_n * 2;

    let h = (b - a) / n as f64;

    let mut s = f(a) + f(b);

    for i in (1..n).step_by(2) {
        s += 4.0 * f(a + (i as f64 * h));
    }

    for i in (2..n - 1).step_by(2) {
        s += 2.0 * f(a + (i as f64 * h));
    }

    let r = (s * h) / 3.0;

    r
}