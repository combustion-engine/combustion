//! Numeric utilities

use num_traits::{Num, Float};

/// Generic min function for any `PartialOrd`
///
/// ```
/// use combustion_common::num_utils::min;
///
/// assert_eq!(min(1, 2), 1);
/// ```
#[inline(always)]
pub fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b { a } else { b }
}

/// Generic max function for any `PartialOrd`
///
/// ```
/// use combustion_common::num_utils::max;
///
/// assert_eq!(max(1, 2), 2);
/// ```
#[inline(always)]
pub fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a >= b { a } else { b }
}

/// Generic min-max function for any `PartialOrd`
///
/// ```
/// use combustion_common::num_utils::min_max;
///
/// assert_eq!(min_max(1, 2), (1, 2));
/// ```
#[inline(always)]
pub fn min_max<T: PartialOrd>(a: T, b: T) -> (T, T) {
    if a >= b { (b, a) } else { (a, b) }
}

/// Round a number to a certain multiple
///
/// E.g.,
///
/// ```
/// use combustion_common::num_utils::round_multiple;
///
/// assert_eq!(round_multiple(43, 5), 45)
/// ```
#[inline(always)]
pub fn round_multiple<T: Num + Copy>(num: T, multiple: T) -> T {
    ((num + multiple - T::one()) / multiple) * multiple
}

/// Adds a `clamp` function to the type
///
/// E.g.,
///
/// ```
/// use combustion_common::num_utils::*;
///
/// assert_eq!(15u32.clamp(0, 5), 5);
/// assert!(3.14f32.clamp(0.0, 1.0) < 2.0);
/// assert!(0.4f32.clamp(1.5, 3.0) > 1.0)
/// ```
pub trait ClampExt {
    /// Clamps the value to `min` and `max` bounds.
    fn clamp(self, min: Self, max: Self) -> Self;
}

impl<T> ClampExt for T where T: PartialOrd {
    fn clamp(self, min: T, max: T) -> T {
        if self < min { min } else if self > max { max } else { self }
    }
}

/// Extension that provides approximate equality comparison for floating point numbers
///
/// E.g.,
///
/// ```
/// use combustion_common::num_utils::*;
///
/// assert!(5.12345f32.almost_eq(5.12, 0.1));
/// assert!(0.00000001f32.almost_eq(0.0, 0.0000001));
/// assert!(0.99999999f32.almost_eq(1.0, 0.0000001));
/// assert!(!(0.1.almost_eq(4.0, 0.1)));
/// ```
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

impl<T> AlmostEqExt for T where T: Float {
    fn almost_eq(&self, b: T, accuracy: T) -> bool {
        if self.is_infinite() || b.is_infinite() {
            *self == b
        } else if self.is_nan() && b.is_nan() {
            false
        } else {
            (*self - b).abs() < accuracy
        }
    }

    #[inline(always)]
    fn almost_eq_fast(&self, b: T, accuracy: T) -> bool {
        (*self - b).abs() < accuracy
    }
}