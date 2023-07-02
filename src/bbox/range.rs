use std::ops::Bound::{Excluded, Included, Unbounded};
use std::ops::Range;
use na::{Point, Scalar};

use crate::BBox;

/// Builds a bounding box from a range of points
///
/// ## Example
/// ```
/// use std::ops::Bound::{Excluded, Included};
/// use nalgebra::point;
/// use pythagore::BBox;
///
/// assert_eq!(
///     BBox::from(point![1, 2]..point![3, 4]),
///     BBox::from([
///        (Included(1), Excluded(3)),
///        (Included(2), Excluded(4)),
///     ])
/// )
/// ```
impl<N: Copy + Scalar, const D: usize> From<Range<Point<N, D>>> for BBox<N, D> {
    fn from(value: Range<Point<N, D>>) -> Self {
        let mut ranges = [(Unbounded, Unbounded); D];

        for (idx, range) in ranges.iter_mut().enumerate() {
            range.0 = Included(unsafe { *value.start.get_unchecked(idx) });
            range.1 = Excluded(unsafe { *value.end.get_unchecked(idx) });
        }

        BBox::from(ranges)
    }
}