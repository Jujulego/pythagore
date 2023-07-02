use std::ops::Bound::{Included, Unbounded};
use std::ops::RangeFrom;
use na::{Point, Scalar};

use crate::BBox;

/// Builds a bounding box from a range of points
///
/// # Example
/// ```
/// use std::ops::Bound::{Excluded, Included, Unbounded};
/// use nalgebra::point;
/// use pythagore::BBox;
///
/// assert_eq!(
///     BBox::from(point![1, 2]..),
///     BBox::from([
///        (Included(1), Unbounded),
///        (Included(2), Unbounded),
///     ])
/// )
/// ```
impl<N: Copy + Scalar, const D: usize> From<RangeFrom<Point<N, D>>> for BBox<N, D> {
    fn from(value: RangeFrom<Point<N, D>>) -> Self {
        let mut ranges = [(Unbounded, Unbounded); D];

        for (idx, range) in ranges.iter_mut().enumerate() {
            range.0 = Included(unsafe { *value.start.get_unchecked(idx) });
        }

        BBox::from(ranges)
    }
}