//! Numeric utilities

use std::ops::{Add, Mul};

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
    if a < b { (a, b) } else { (b, a) }
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

/// Clamp a value to the given range
///
/// ```
/// use combustion_common::num_utils::clamp;
///
/// assert_eq!(clamp(15u32, 0, 5), 5);
/// ```
pub fn clamp<T>(value: T, min: T, max: T) -> T where T: PartialOrd {
    if value < min { min } else if value > max { max } else { value }
}

/// Adds a `clamp` function to the type
///
/// E.g.,
///
/// ```
/// use combustion_common::num_utils::ClampExt;
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
/// use combustion_common::num_utils::AlmostEqExt;
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

/// Linear interpolation for numeric types
///
/// ```
/// use combustion_common::num_utils::lerp;
///
/// assert_eq!(lerp(0.5f32, 0.0, 0.0, 1.0, 3.0), 1.5);
/// ```
pub fn lerp<T: Num + Copy>(x: T, x0: T, y0: T, x1: T, y1: T) -> T {
    y0 + (x - x0) * ((y1 - y0) / (x1 - x0))
}

/// Generic linear interpolation for any supported types.
///
/// This form can support non-numeric `T` types if they satisfy the clause conditions.
///
/// ```
/// use combustion_common::num_utils::lerp_generic as lerp;
/// use combustion_common::color::Color;
///
/// let gray = Color::new(0.5, 0.5, 0.5, 1.0);
///
/// assert_eq!(gray, lerp(Color::white(), Color::black(), 0.5));
/// ```
pub fn lerp_generic<T, W: Num + Copy>(v0: T, v1: T, t: W) -> <<T as Mul<W>>::Output as Add>::Output where T: Mul<W>,
                                                                                                          <T as Mul<W>>::Output: Add<<T as Mul<W>>::Output>,
                                                                                                          <T as Mul<W>>::Output: Add<W> {
    // Ordered like this to preserve RHS ops
    v0 * (W::one() - t) + v1 * t
}

/// Trait to add generic linear interpolation functionality to types directly.
///
/// ```
/// use combustion_common::num_utils::LerpExt;
///
/// assert_eq!(0.0f32.lerp(0.5, 3.0), 1.5);
/// ```
pub trait LerpExt where Self: Num + Copy {
    /// Linearly interpolate `self` with `other` based on the weight value `t`
    fn lerp(self, t: Self, other: Self) -> Self {
        (Self::one() - t) * self + t * other
    }
}

/// Trait to add generic linear interpolation to types
///
/// ```
/// use combustion_common::num_utils::LerpGenericExt;
/// use combustion_common::color::Color;
///
/// let gray = Color::new(0.5, 0.5, 0.5, 1.0);
///
/// assert_eq!(gray, Color::white().lerp(Color::black(), 0.5));
/// ```
pub trait LerpGenericExt: Sized {
    /// Linearly interpolate `self` with `other` based on the weight value `t`
    ///
    /// This is the generic form which can support non-numeric `Self` types if they satisfy the clause conditions.
    fn lerp<W: Num + Copy>(self, other: Self, t: W) -> <<Self as Mul<W>>::Output as Add>::Output where Self: Mul<W>,
                                                                                                       <Self as Mul<W>>::Output: Add<<Self as Mul<W>>::Output>,
                                                                                                       <Self as Mul<W>>::Output: Add<W> {
        self * (W::one() - t) + other * t
    }
}

impl<T> LerpExt for T where T: Num + Copy {}

/// Scales a value between the range `in_min` and `in_max` to the range of `out_min` to `out_max`
///
/// ```
/// use combustion_common::num_utils::scale;
///
/// assert_eq!(scale(0.5f32, 0.0, 1.0, 0.0, 2.0), 1.0);
/// ```
#[inline]
pub fn scale<T: Num + Copy>(x: T, in_min: T, in_max: T, out_min: T, out_max: T) -> T {
    (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

/// Add `scale` method to `Num + Copy` types to scale their value
///
/// ```
/// use combustion_common::num_utils::ScaleExt;
///
/// assert_eq!(0.5f32.scale(0.0, 1.0, 0.0, 2.0), 1.0);
/// ```
pub trait ScaleExt where Self: Num + Copy {
    /// Scales the value between the range `in_min` and `in_max` to the range of `out_min` to `out_max`
    #[inline]
    fn scale(self, in_min: Self, in_max: Self, out_min: Self, out_max: Self) -> Self {
        scale(self, in_min, in_max, out_min, out_max)
    }
}

impl<T> ScaleExt for T where T: Num + Copy {}