use std::ops::Bound::{Included, Unbounded};
use std::ops::RangeInclusive;
use na::{Point, Scalar};

use crate::{BBox, BoundPoints};

/// Builds a bounding box from a range of points
///
/// # Example
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

impl<N: Copy + Scalar, const D: usize> BoundPoints<N, D> for RangeInclusive<Point<N, D>> {
    #[inline]
    fn start_point(&self) -> Point<N, D> {
        *self.start()
    }

    #[inline]
    fn end_point(&self) -> Point<N, D> {
        *self.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod bound_points {
        use na::point;
        use super::*;

        #[test]
        fn test_start_point() {
            assert_eq!(
                (point![0, 0]..=point![5, 5]).start_point(),
                point![0, 0]
            );
        }

        #[test]
        fn test_end_point() {
            assert_eq!(
                (point![0, 0]..=point![5, 5]).end_point(),
                point![5, 5]
            );
        }
    }
}