use num_traits::Num;

pub mod stopwatch;
pub mod human_readable;

#[inline(always)]
pub fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b { a } else { b }
}

#[inline(always)]
pub fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

#[inline(always)]
pub fn round_multiple<T: Num + Copy>(num: T, multiple: T) -> T {
    ((num + multiple - T::one()) / multiple) * multiple
}

pub trait ClampExt {
    fn clamp(&self, min: Self, max: Self) -> Self;
}

macro_rules! impl_clamp {
    ($t:ident) => {
        impl ClampExt for $t {
            #[inline]
            fn clamp(&self, min: Self, max: Self) -> Self {
                if *self < min { min } else if *self > max { max } else { *self }
            }
        }
    }
}

impl_clamp!(usize);
impl_clamp!(isize);

impl_clamp!(u8);
impl_clamp!(i8);

impl_clamp!(u16);
impl_clamp!(i16);

impl_clamp!(u32);
impl_clamp!(i32);

impl_clamp!(u64);
impl_clamp!(i64);

impl_clamp!(f32);
impl_clamp!(f64);

/// Extension that provides approximate equality comparison for floating point numbers
pub trait AlmostEqExt {
    /// Tests if two numbers are almost equal within a degree of accuracy
    ///
    /// E.g.:
    ///
    /// ```ignore
    /// assert!(5.12345f32.almost_eq(5.12, 0.1));
    /// assert!(0.00000001f32.almost_eq(0.0, 0.0000001));
    /// assert!(0.99999999f32.almost_eq(1.0, 0.0000001));
    /// ```
    fn almost_eq(&self, b: Self, accuracy: Self) -> bool;

    /// Variation of `almost_eq` that doesn't check for infinite or NaN values.
    fn almost_eq_fast(&self, b: Self, accuracy: Self) -> bool;
}

macro_rules! impl_almost_eq_ext {
    ($t:ident) => {
        impl AlmostEqExt for $t {
            fn almost_eq(&self, b: $t, accuracy: $t) -> bool {
                if self.is_infinite() || b.is_infinite() {
                    return *self == b;
                } else if self.is_nan() && b.is_nan() {
                    return false;
                } else {
                    (*self - b).abs() < accuracy
                }
            }

            #[inline(always)]
            fn almost_eq_fast(&self, b: $t, accuracy: $t) -> bool {
                (*self - b).abs() < accuracy
            }
        }
    }
}

impl_almost_eq_ext!(f32);
impl_almost_eq_ext!(f64);

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_clamp() {
        assert_eq!(15u32.clamp(0, 5), 5);
        assert!(3.14f32.clamp(0.0, 1.0) < 2.0);
        assert!(0.4f32.clamp(1.5, 3.0) > 1.0)
    }

    #[test]
    fn test_almost_eq() {
        assert!(5.12345f32.almost_eq(5.12, 0.1));
        assert!(0.00000001f32.almost_eq(0.0, 0.0000001));
        assert!(0.99999999f32.almost_eq(1.0, 0.0000001));
    }

    #[test]
    #[should_panic]
    fn test_almost_eq2() {
        assert!((0.1.almost_eq(4.0, 0.1)));
    }
}