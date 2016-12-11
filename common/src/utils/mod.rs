use num_traits::Num;
//use std::ops::{Deref, DerefMut};

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
pub fn clamp<T: PartialOrd>(v: T, a: T, b: T) -> T {
    max(a, min(b, v))
}

#[inline(always)]
pub fn round_multiple<T: Num + Copy>(num: T, multiple: T) -> T {
    ((num + multiple - T::one()) / multiple) * multiple
}

