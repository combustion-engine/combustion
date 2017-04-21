
pub fn fresnel_schlick(cos_theta: f64, eta_t: f64, eta_i: f64) -> f64 {
    let f0 = ((eta_i - eta_t) / (eta_i + eta_t)).powi(2);

    f0 + (1.0 - f0) * (1.0 - cos_theta).powi(5)
}

pub fn fresnel(cos_theta: f64, eta_t: f64, k_t: f64, eta_i: f64) -> (f64, f64) {
    let eta_t2 = eta_t * eta_t;
    let k_t2 = k_t * k_t;
    let eta_i2 = eta_i * eta_i;

    let cos_theta2 = cos_theta * cos_theta;
    let sin_theta2 = 1.0 - cos_theta2;
    let sin_theta = sin_theta2.sqrt();
    let tan_theta = sin_theta / cos_theta;
    let tan_theta2 = tan_theta * tan_theta;

    let ab_muliplier = 1.0 / (2.0 * eta_i2);

    let t0 = eta_i2 * sin_theta2;

    let sqrt_part = ((eta_t2 - k_t2 - t0).powi(2) + (4.0 * eta_t2 * k_t2)).sqrt();

    let a2 = ab_muliplier * (sqrt_part + eta_t2 - k_t2 - t0);
    let b2 = ab_muliplier * (sqrt_part - eta_t2 + k_t2 + t0);

    let a = a2.sqrt();

    let a2plusb2 = a2 + b2;

    let t1 = 2.0 * a * cos_theta;
    let t2 = 2.0 * a * sin_theta * tan_theta;
    let t3 = sin_theta2 * tan_theta2;

    let rs = (a2plusb2 - t1 + cos_theta2) / (a2plusb2 + t1 + cos_theta2);

    let rp = rs * ((a2plusb2 - t2 + t3) / (a2plusb2 + t2 + t3));

    (rs, rp)
}