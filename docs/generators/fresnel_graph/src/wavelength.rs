pub const RED_WAVELENGTH_MIN: f64 = 0.620;
pub const GREEN_WAVELENGTH_MIN: f64 = 0.495;
pub const BLUE_WAVELENGTH_MIN: f64 = 0.450;

pub const RED_WAVELENGTH_MAX: f64 = 0.740;
pub const GREEN_WAVELENGTH_MAX: f64 = 0.570;
pub const BLUE_WAVELENGTH_MAX: f64 = 0.495;

pub const RED_WAVELENGTH: f64 = (RED_WAVELENGTH_MIN + RED_WAVELENGTH_MAX) / 2.0;
pub const GREEN_WAVELENGTH: f64 = (GREEN_WAVELENGTH_MIN + GREEN_WAVELENGTH_MAX) / 2.0;
pub const BLUE_WAVELENGTH: f64 = (BLUE_WAVELENGTH_MIN + BLUE_WAVELENGTH_MAX) / 2.0;

pub fn w_to_color(w: f64) -> (f64, f64, f64) {
    let r;
    let g;
    let b;

    if (w >= 0.380) && (w < 0.440) {
        r = -(w - 0.440) / (0.440 - 0.380);
        g = 0.0;
        b = 1.0;
    } else if (w >= 0.440) && (w < 0.490) {
        r = 0.0;
        g = (w - 0.440) / (0.490 - 0.440);
        b = 1.0;
    } else if (w >= 0.490) && (w < 0.510) {
        r = 0.0;
        g = 1.0;
        b = -(w - 0.510) / (0.510 - 0.490);
    } else if (w >= 0.510) && (w < 0.580) {
        r = (w - 0.510) / (0.580 - 0.510);
        g = 1.0;
        b = 0.0;
    } else if (w >= 0.580) && (w < 0.645) {
        r = 1.0;
        g = -(w - 0.645) / (0.645 - 0.580);
        b = 0.0;
    } else if (w >= 0.645) && (w < 0.781) {
        r = 1.0;
        g = 0.0;
        b = 0.0;
    } else {
        r = 0.0;
        g = 0.0;
        b = 0.0;
    }

    let factor = if (w >= 0.380) && (w < 0.420) {
        0.3 + 0.7 * (w - 0.380) / (0.420 - 0.380)
    } else if (w >= 0.420) && (w < 0.701) {
        1.0
    } else if (w >= 0.701) && (w < 0.781) {
        0.3 + 0.7 * (0.780 - w) / (0.780 - 0.700)
    } else {
        0.0
    };

    (r * factor, g * factor, b * factor)
}