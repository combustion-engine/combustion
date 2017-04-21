use std::ops::Neg;

use common::num_utils::ScaleExt;

use super::calculus::simpsons_rule;

use super::wavelength::{RED_WAVELENGTH_MIN, RED_WAVELENGTH_MAX};
use super::wavelength::{GREEN_WAVELENGTH_MIN, GREEN_WAVELENGTH_MAX};
use super::wavelength::{BLUE_WAVELENGTH_MIN, BLUE_WAVELENGTH_MAX};

use super::tables;

pub fn gaussian_weight_function(x: f64) -> f64 {
    (x - 0.5).powi(2).neg().exp()
}

pub fn gaussian_weighted_integral<F>(half_n: u32, a: f64, b: f64, f: F) -> f64 where F: Fn(f64) -> f64 {
    simpsons_rule(half_n, a, b, |x| { f(x) * gaussian_weight_function(x.scale(a, b, 0.0, 1.0)) }) /
        ((b - a) * simpsons_rule(half_n, 0.0, 1.0, gaussian_weight_function))
}

pub type RGBResponse = ((f64, f64), (f64, f64), (f64, f64));

pub fn weighted_wavelength_response(table: tables::IORTable, half_samples: u32) -> RGBResponse {
    let get_eta = |wavelength| {
        let (eta, _) = tables::get_ior(table, wavelength).unwrap();
        eta
    };

    let get_k = |wavelength| {
        let (_, k) = tables::get_ior(table, wavelength).unwrap();
        k
    };

    let red_eta = gaussian_weighted_integral(half_samples, RED_WAVELENGTH_MIN, RED_WAVELENGTH_MAX, &get_eta);
    let green_eta = gaussian_weighted_integral(half_samples, GREEN_WAVELENGTH_MIN, GREEN_WAVELENGTH_MAX, &get_eta);
    let blue_eta = gaussian_weighted_integral(half_samples, BLUE_WAVELENGTH_MIN, BLUE_WAVELENGTH_MAX, &get_eta);

    let red_k = gaussian_weighted_integral(half_samples, RED_WAVELENGTH_MIN, RED_WAVELENGTH_MAX, &get_k);
    let green_k = gaussian_weighted_integral(half_samples, GREEN_WAVELENGTH_MIN, GREEN_WAVELENGTH_MAX, &get_k);
    let blue_k = gaussian_weighted_integral(half_samples, BLUE_WAVELENGTH_MIN, BLUE_WAVELENGTH_MAX, &get_k);

    ((red_eta, red_k), (green_eta, green_k), (blue_eta, blue_k))
}