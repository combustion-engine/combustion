//! Probability Distribution Functions

pub fn gaussian_dot_pdf(x: f64, width: f64, hardness: f64) -> f64 {
    ::std::f64::consts::E.powf(-((x / (width * 0.5)).powf(2.0f64.powf(hardness))))
}