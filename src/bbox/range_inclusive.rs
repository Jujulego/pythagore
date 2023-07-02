use std::ops::Bound::{Included, Unbounded};
use std::ops::RangeInclusive;
use na::{Point, Scalar};

use crate::BBox;

/// Builds a bounding box from a range of points
///
/// ## Example
/// ```
/// use std::ops::Bound::Included;
/// use nalgebra::point;
/// use pythagore::BBox;
///
/// assert_eq!(
///     BBox::from(point![1, 2]..=point![3, 4]),
///     BBox::from([
///        (Included(1), Included(3)),
///        (Included(2), Included(4)),
///     ])
/// )
/// ```
impl<N: Copy + Scalar, const D: usize> From<RangeInclusive<Point<N, D>>> for BBox<N, D> {
    fn from(value: RangeInclusive<Point<N, D>>) -> Self {
        let mut ranges = [(Unbounded, Unbounded); D];

        for (idx, range) in ranges.iter_mut().enumerate() {
            range.0 = Included(unsafe { *value.start().get_unchecked(idx) });
            range.1 = Included(unsafe { *value.end().get_unchecked(idx) });
        }

        BBox::from(ranges)
    }
}