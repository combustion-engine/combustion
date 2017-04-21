//! Simple Bezier Curve container and evaluator

pub type BezierControls = Vec<(f64, f64)>;

pub struct BezierCurve {
    controls: BezierControls,
}

impl BezierCurve {
    pub fn new(controls: BezierControls) -> BezierCurve {
        BezierCurve { controls: controls }
    }

    pub fn controls(&self) -> &BezierControls { &self.controls }

    pub fn controls_mut(&mut self) -> &mut BezierControls { &mut self.controls }

    pub fn evaluate(&self, t: f64) -> (f64, f64) {
        let u = 1.0 - t;

        assert!(self.controls.len() > 0, "No control points defined");

        let (x, y) = (0.0, 0.0);

        match self.controls.len() - 1 {
            0 => { self.controls[0] }
            1 => {
                let (p0, p1) = (self.controls[0], self.controls[1]);

                let (x, y) = (u * p0.0 + t * p1.0,
                              u * p0.1 + t * p1.1);

                (x, y)
            }
            2 => {
                let tt = t * t;
                let uu = u * u;

                let (p0, p1, p2) = (self.controls[0], self.controls[1], self.controls[2]);

                // first term: [x, y] += (1-t)^2 * p0
                let (x, y) = (x + uu * p0.0,
                              y + uu * p0.1);

                // second term: [x, y] += 2 * (1-t) * t * p1
                let (x, y) = (x + 2.0 * u * t * p1.0,
                              y + 2.0 * u * t * p1.1);

                // third term: [x, y] += t^2 * p2
                let (x, y) = (x + tt * p2.0,
                              y + tt * p2.1);

                (x, y)
            }
            3 => {
                let tt = t * t;
                let uu = u * u;
                let uuu = uu * u;
                let ttt = tt * t;

                let (p0, p1, p2, p3) = (self.controls[0], self.controls[1], self.controls[2], self.controls[3]);

                // [x, y] = ((1-t)^3 * p0) + (3 * (1-t)^2 * t * p1) + (3 * (1-t) * t^2 * p2) + (3 * t^3 * p3)

                // first term: [x, y] += (1-t)^3 * p0
                let (x, y) = (uuu * p0.0,
                              uuu * p0.1);

                // second term: [x, y] += 3 * (1-t)^2 * t * p1
                let (x, y) = (x + 3.0 * uu * t * p1.0,
                              y + 3.0 * uu * t * p1.1);

                // third term: [x, y] += 3 * (1-t) * t^2 * p2
                let (x, y) = (x + 3.0 * u * tt * p2.0,
                              y + 3.0 * u * tt * p2.1);

                // fourth term: [x, y] += t^3 * p3
                let (x, y) = (x + ttt * p3.0,
                              y + ttt * p3.1);

                (x, y)
            }
            _ => unimplemented!()
        }
    }
}