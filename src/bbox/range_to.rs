use std::ops::Bound::{Excluded, Unbounded};
use std::ops::RangeTo;
use na::{Point, Scalar};

use crate::BBox;

/// Builds a bounding box from a range of points
///
/// # Example
/// ```
/// use std::ops::Bound::{Excluded, Unbounded};
/// use nalgebra::point;
/// use pythagore::BBox;
///
/// assert_eq!(
///     BBox::from(..point![3, 4]),
///     BBox::from([
///        (Unbounded, Excluded(3)),
///        (Unbounded, Excluded(4)),
///     ])
/// )
/// ```
impl<N: Copy + Scalar, const D: usize> From<RangeTo<Point<N, D>>> for BBox<N, D> {
    fn from(value: RangeTo<Point<N, D>>) -> Self {
        let mut ranges = [(Unbounded, Unbounded); D];

        for (idx, range) in ranges.iter_mut().enumerate() {
            range.1 = Excluded(unsafe { *value.end.get_unchecked(idx) });
        }

        BBox::from(ranges)
    }
}