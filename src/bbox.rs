mod bound_tuple;
mod range;
mod range_from;
mod range_inclusive;
mod range_to;
mod range_to_inclusive;

use std::ops::Bound;
use std::ops::Bound::Unbounded;
use std::slice::{Iter, IterMut};
use na::Scalar;

/// Generic Axis Aligned Bounding Box
/// Supports all kinds of bounds, independently on each axis
#[derive(Debug, Eq)]
pub struct BBox<N: Scalar, const D: usize> {
    ranges: [(Bound<N>, Bound<N>); D],
}

impl<N: Scalar, const D: usize> BBox<N, D> {
    /// Returns iterator over internal ranges
    pub fn iter(&self) -> Iter<(Bound<N>, Bound<N>)> {
        self.ranges.iter()
    }

    /// Returns mutable iterator over internal ranges
    pub fn iter_mut(&mut self) -> IterMut<(Bound<N>, Bound<N>)> {
        self.ranges.iter_mut()
    }
}

// Utils
/// Default is a fully unbounded bbox
///
/// ## Example
/// ```
/// use std::ops::Bound::Unbounded;
/// use pythagore::BBox;
///
/// assert_eq!(
///     BBox::<i32, 2>::default(),
///     BBox::from([
///        (Unbounded, Unbounded),
///        (Unbounded, Unbounded),
///     ])
/// )
/// ```
impl<N: Copy + Scalar, const D: usize> Default for BBox<N, D> {
    fn default() -> Self {
        BBox {
            ranges: [(Unbounded, Unbounded); D]
        }
    }
}

// Conversion
/// Builds a bounding box from a set of ranges
impl<N: Scalar, const D: usize> From<[(Bound<N>, Bound<N>); D]> for BBox<N, D> {
    fn from(value: [(Bound<N>, Bound<N>); D]) -> Self {
        BBox {
            ranges: value
        }
    }
}

// Operators
impl<N: Scalar, const D: usize> PartialEq for BBox<N, D> {
    fn eq(&self, other: &Self) -> bool {
        self.ranges == other.ranges
    }
}
