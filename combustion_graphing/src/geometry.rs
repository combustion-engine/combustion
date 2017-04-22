use common::num_utils::min_max;

pub enum Intersection {
    NoIntersection,
    Intersection(f64, f64),
    Collinear
}

impl Intersection {
    pub fn line_line(x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, x4: f64, y4: f64) -> Intersection {
        let d = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

        if d.abs() < 0.0001 {
            Intersection::Collinear
        } else {
            let a = x1 * y2 - y1 * x2;
            let b = x3 * y4 - y3 * x4;

            let x = (a * (x3 - x4) - (x1 - x2) * b) / d;
            let y = (a * (y3 - y4) - (y1 - y2) * b) / d;

            let (x0min, x0max) = min_max(x1, x2);
            let (x1min, x1max) = min_max(x3, x4);

            let (y0min, y0max) = min_max(y1, y2);
            let (y1min, y1max) = min_max(y3, y4);

            if x0min <= x && x <= x0max && x1min <= x && x <= x1max &&
                y0min <= y && y <= y0max && y1min <= y && y <= y1max {
                Intersection::Intersection(x, y)
            } else {
                Intersection::NoIntersection
            }
        }
    }
}