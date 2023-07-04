use std::ops::Bound::{Excluded, Included, Unbounded};
use std::ops::Range;
use na::{ClosedSub, Point, Scalar, SVector};
use num_traits::One;

use crate::{BBox, PointBounds, Walkable};

/// Builds a bounding box from a range of points
///
/// # Example
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

impl<N: Copy + Scalar, const D: usize> PointBounds<N, D> for Range<Point<N, D>> {
    #[inline]
    fn start_point(&self) -> Option<Point<N, D>> {
        Some(self.start)
    }

    #[inline]
    fn end_point(&self) -> Option<Point<N, D>> {
        Some(self.end)
    }
}

impl<N: ClosedSub + Copy + One + Scalar, const D: usize> Walkable<N, D> for Range<Point<N, D>> {
    #[inline]
    fn first_point(&self) -> Option<Point<N, D>> {
        Some(self.start)
    }

    #[inline]
    fn last_point(&self) -> Option<Point<N, D>> {
        Some(self.end - SVector::repeat(N::one()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod point_bounds {
        use na::point;
        use super::*;

        #[test]
        fn test_start_point() {
            assert_eq!(
                (point![0, 0]..point![5, 5]).start_point(),
                Some(point![0, 0])
            );
        }

        #[test]
        fn test_end_point() {
            assert_eq!(
                (point![0, 0]..point![5, 5]).end_point(),
                Some(point![5, 5])
            );
        }
    }

    mod walkable {
        use na::point;
        use super::*;

        #[test]
        fn test_first_point() {
            assert_eq!(
                (point![0, 0]..point![5, 5]).first_point(),
                Some(point![0, 0])
            );
        }

        #[test]
        fn test_last_point() {
            assert_eq!(
                (point![0, 0]..point![5, 5]).last_point(),
                Some(point![4, 4])
            );
        }
    }
}