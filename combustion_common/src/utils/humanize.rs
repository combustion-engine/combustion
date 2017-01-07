//! Humanize values

use std::cmp;

pub const SI_UNITS: [&'static str; 9] = ["B", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
pub const IEC_UNITS: [&'static str; 9] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB"];

/// Humanize with SI units
///
/// E.g.,
///
/// ```
/// use combustion_common::utils::humanize::humanize_si;
///
/// assert_eq!(humanize_si(0.1), "0.1 B");
/// assert_eq!(humanize_si(10.0), "10.00 B");
/// assert_eq!(humanize_si(4_321.00), "4.32 kB");
/// assert_eq!(humanize_si(4_321_000.00), "4.32 MB");
/// assert_eq!(humanize_si(4_321_000_000.00), "4.32 GB");
/// assert_eq!(humanize_si(4_321_000_000_000.00), "4.32 TB");
///
/// assert_eq!(humanize_si(-4_321.00), "-4.32 kB");
/// assert_eq!(humanize_si(-0.1), "-0.1 B");
/// ```
pub fn humanize_si(num: f64) -> String {
    const DELIMITER: f64 = 1000.0;
    const DELIMITER_LOG: f64 = 3.0; //log10

    if num < 1_f64 && num > -1_f64 {
        format!("{} {}", num, "B")
    } else {
        let exponent = cmp::min((num.abs().log10() / DELIMITER_LOG).floor() as i32,
                                (SI_UNITS.len() - 1) as i32);

        format!("{:.2} {}", num / DELIMITER.powi(exponent), SI_UNITS[exponent as usize])
    }
}

/// Humanize with IEC base-2 units
///
/// E.g.,
///
/// ```
/// use combustion_common::utils::humanize::humanize_iec;
///
/// assert_eq!(humanize_iec(0.1), "0.1 B");
/// assert_eq!(humanize_iec(10.0), "10.00 B");
/// assert_eq!(humanize_iec(4_321.00), "4.22 KiB");
/// assert_eq!(humanize_iec(4_321_000.00), "4.12 MiB");
/// assert_eq!(humanize_iec(4_321_000_000.00), "4.02 GiB");
/// assert_eq!(humanize_iec(4_321_000_000_000.00), "3.93 TiB");
///
/// assert_eq!(humanize_iec(-4_321.00), "-4.22 KiB");
/// assert_eq!(humanize_iec(-0.1), "-0.1 B");
/// ```
pub fn humanize_iec(num: f64) -> String {
    const DELIMITER: f64 = 1024.0;
    const DELIMITER_LOG: f64 = 10.0; //log2

    if num <= 1_f64 && num >= -1_f64 {
        format!("{} {}", num, "B")
    } else {
        let exponent = cmp::min((num.abs().log2() / DELIMITER_LOG).floor() as i32,
                                (IEC_UNITS.len() - 1) as i32);

        format!("{:.2} {}", num / DELIMITER.powi(exponent), IEC_UNITS[exponent as usize])
    }
}