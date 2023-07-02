use std::ops::Bound::{self, Excluded, Included, Unbounded};
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
///     BBox::from((Excluded(point![1, 2]), Included(point![3, 4]))),
///     BBox::from([
///        (Excluded(1), Included(3)),
///        (Excluded(2), Included(4)),
///     ])
/// )
/// ```
impl<N: Copy + Scalar, const D: usize> From<(Bound<Point<N, D>>, Bound<Point<N, D>>)> for BBox<N, D> {
    fn from(value: (Bound<Point<N, D>>, Bound<Point<N, D>>)) -> Self {
        let mut ranges = [(Unbounded, Unbounded); D];

        for (idx, range) in ranges.iter_mut().enumerate() {
            range.0 = match value.0 {
                Included(x) => Included(x[idx]),
                Excluded(x) => Excluded(x[idx]),
                Unbounded => Unbounded,
            };

            range.1 = match value.1 {
                Included(x) => Included(x[idx]),
                Excluded(x) => Excluded(x[idx]),
                Unbounded => Unbounded,
            };
        }

        BBox::from(ranges)
    }
}