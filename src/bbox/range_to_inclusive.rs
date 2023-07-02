use std::ops::Bound::{Included, Unbounded};
use std::ops::RangeToInclusive;
use na::{Point, Scalar};

use crate::BBox;

/// Builds a bounding box from a range of points
///
/// ## Example
/// ```
/// use std::ops::Bound::{Included, Unbounded};
/// use nalgebra::point;
/// use pythagore::BBox;
///
/// assert_eq!(
///     BBox::from(..=point![3, 4]),
///     BBox::from([
///        (Unbounded, Included(3)),
///        (Unbounded, Included(4)),
///     ])
/// )
/// ```
impl<N: Copy + Scalar, const D: usize> From<RangeToInclusive<Point<N, D>>> for BBox<N, D> {
    fn from(value: RangeToInclusive<Point<N, D>>) -> Self {
        let mut ranges = [(Unbounded, Unbounded); D];

        for (idx, range) in ranges.iter_mut().enumerate() {
            range.1 = Included(value.end[idx]);
        }

        BBox::from(ranges)
    }
}