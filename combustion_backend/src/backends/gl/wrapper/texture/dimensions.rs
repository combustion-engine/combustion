//! Texture Dimension utilities

use super::super::bindings::types::*;
use super::super::bindings::*;

/// Trait for generic dimensions
pub trait GLDimensions: Sized {
    /// Number of dimensions
    fn dimensions() -> usize;
    /// OpenGL Texture Wrap enum value for the given dimension
    fn texture_wrap(&self) -> GLenum;
    /// "Iterate" over all dimensions and apply the given functor
    fn iterate<F, T, E>(f: F) -> Result<(), E> where F: FnMut(Self) -> Result<T, E>;
}

/// Single dimension
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GLOneDimension {
    One,
}

impl GLDimensions for GLOneDimension {
    #[inline(always)]
    fn dimensions() -> usize { 1 }

    #[inline]
    fn texture_wrap(&self) -> GLenum {
        match *self {
            GLOneDimension::One => TEXTURE_WRAP_S
        }
    }

    fn iterate<F, T, E>(mut f: F) -> Result<(), E> where F: FnMut(Self) -> Result<T, E> {
        f(GLOneDimension::One)?;
        Ok(())
    }
}

/// Two dimensions
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GLTwoDimensions {
    One,
    Two
}

impl GLDimensions for GLTwoDimensions {
    #[inline(always)]
    fn dimensions() -> usize { 2 }

    #[inline]
    fn texture_wrap(&self) -> GLenum {
        match *self {
            GLTwoDimensions::One => TEXTURE_WRAP_S,
            GLTwoDimensions::Two => TEXTURE_WRAP_T,
        }
    }

    fn iterate<F, T, E>(mut f: F) -> Result<(), E> where F: FnMut(Self) -> Result<T, E> {
        f(GLTwoDimensions::One)?;
        f(GLTwoDimensions::Two)?;
        Ok(())
    }
}

/// Three dimensions
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GLThreeDimensions {
    One,
    Two,
    Three
}

impl GLDimensions for GLThreeDimensions {
    #[inline(always)]
    fn dimensions() -> usize { 3 }

    #[inline]
    fn texture_wrap(&self) -> GLenum {
        match *self {
            GLThreeDimensions::One => TEXTURE_WRAP_S,
            GLThreeDimensions::Two => TEXTURE_WRAP_T,
            GLThreeDimensions::Three => TEXTURE_WRAP_R,
        }
    }

    fn iterate<F, T, E>(mut f: F) -> Result<(), E> where F: FnMut(Self) -> Result<T, E> {
        f(GLThreeDimensions::One)?;
        f(GLThreeDimensions::Two)?;
        f(GLThreeDimensions::Three)?;
        Ok(())
    }
}
