mod bound_tuple;
mod range;
mod range_from;
mod range_inclusive;
mod range_to;
mod range_to_inclusive;

use std::ops::{Bound, Index, IndexMut};
use std::ops::Bound::Unbounded;
use std::slice::{Iter, IterMut};
use na::Scalar;

type BBoxElement<N> = (Bound<N>, Bound<N>);

/// Generic Axis Aligned Bounding Box
/// Supports all kinds of bounds, independently on each axis
#[derive(Debug, Eq)]
pub struct BBox<N: Scalar, const D: usize> {
    ranges: [BBoxElement<N>; D],
}

impl<N: Scalar, const D: usize> BBox<N, D> {
    /// Returns a reference to an internal range, without doing bounds checking.
    ///
    /// # Safety
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*
    /// even if the resulting reference is not used.
    #[inline]
    pub unsafe fn get_unchecked(&self, idx: usize) -> &BBoxElement<N> {
        self.ranges.get_unchecked(idx)
    }

    /// Returns a mutable reference to an internal range, without doing bounds checking.
    ///
    /// # Safety
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*
    /// even if the resulting reference is not used.
    #[inline]
    pub unsafe fn get_unchecked_mut(&mut self, idx: usize) -> &BBoxElement<N> {
        self.ranges.get_unchecked_mut(idx)
    }

    /// Returns iterator over internal ranges
    #[inline]
    pub fn iter(&self) -> Iter<BBoxElement<N>> {
        self.ranges.iter()
    }

    /// Returns mutable iterator over internal ranges
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<BBoxElement<N>> {
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
impl<N: Scalar, const D: usize> From<[BBoxElement<N>; D]> for BBox<N, D> {
    fn from(ranges: [BBoxElement<N>; D]) -> Self {
        BBox {
            ranges
        }
    }
}

// Operators
impl<N: Scalar, const D: usize> Index<usize> for BBox<N, D> {
    type Output = BBoxElement<N>;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.ranges[index]
    }
}

impl<N: Scalar, const D: usize> IndexMut<usize> for BBox<N, D> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.ranges[index]
    }
}

impl<N: Scalar, const D: usize> PartialEq for BBox<N, D> {
    fn eq(&self, other: &Self) -> bool {
        self.ranges == other.ranges
    }
}
