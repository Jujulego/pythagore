use std::ops::Bound::{Included, Unbounded};
use std::ops::RangeFrom;
use na::{Point, Scalar};

use crate::{BBox, PointBounds};

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

impl<N: Copy + Scalar, const D: usize> PointBounds<N, D> for RangeFrom<Point<N, D>> {
    #[inline]
    fn start_point(&self) -> Option<Point<N, D>> {
        Some(self.start)
    }

    #[inline]
    fn end_point(&self) -> Option<Point<N, D>> {
        None
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
                (point![0, 0]..).start_point(),
                Some(point![0, 0])
            );
        }

        #[test]
        fn test_end_point() {
            assert_eq!(
                (point![0, 0]..).end_point(),
                None
            );
        }
    }
}