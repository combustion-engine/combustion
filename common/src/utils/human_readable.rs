use std::cmp;

pub const UNITS: [&'static str; 9] = ["B", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
pub const SI_UNITS: [&'static str; 9] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB"];

pub fn humanize(num: f64) -> String {
    const DELIMITER: f64 = 1000.0;
    const DELIMITER_LOG: f64 = 3.0; //log10

    if num < 1_f64 && num > -1_f64 {
        format!("{} {}", num, "B")
    } else {
        let exponent = cmp::min((num.abs().log10() / DELIMITER_LOG).floor() as i32,
                                (UNITS.len() - 1) as i32);

        format!("{:.2} {}", num / DELIMITER.powi(exponent), UNITS[exponent as usize])
    }
}

pub fn humanize_si(num: f64) -> String {
    const DELIMITER: f64 = 1024.0;
    const DELIMITER_LOG: f64 = 10.0; //log2

    if num <= 1_f64 && num >= -1_f64 {
        format!("{} {}", num, "B")
    } else {
        let exponent = cmp::min((num.abs().log2() / DELIMITER_LOG).floor() as i32,
                                (SI_UNITS.len() - 1) as i32);

        format!("{:.2} {}", num / DELIMITER.powi(exponent), SI_UNITS[exponent as usize])
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_humanize() {
        assert_eq!(humanize(0.1), "0.1 B");
        assert_eq!(humanize(10.0), "10.00 B");
        assert_eq!(humanize(4_321.00), "4.32 kB");
        assert_eq!(humanize(4_321_000.00), "4.32 MB");
        assert_eq!(humanize(4_321_000_000.00), "4.32 GB");
        assert_eq!(humanize(4_321_000_000_000.00), "4.32 TB");

        assert_eq!(humanize(-4_321.00), "-4.32 kB");
        assert_eq!(humanize(-0.1), "-0.1 B");
    }

    #[test]
    fn test_humanize_si() {
        assert_eq!(humanize_si(0.1), "0.1 B");
        assert_eq!(humanize_si(10.0), "10.00 B");
        assert_eq!(humanize_si(4_321.00), "4.22 KiB");
        assert_eq!(humanize_si(4_321_000.00), "4.12 MiB");
        assert_eq!(humanize_si(4_321_000_000.00), "4.02 GiB");
        assert_eq!(humanize_si(4_321_000_000_000.00), "3.93 TiB");

        assert_eq!(humanize_si(-4_321.00), "-4.22 KiB");
        assert_eq!(humanize_si(-0.1), "-0.1 B");
    }
}