use std::ops::Bound::{Excluded, Unbounded};
use std::ops::RangeTo;
use na::{Point, Scalar};
use num_traits::Bounded;

use crate::{BBox, BoundPoints};

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

impl<N: Bounded + Copy + Scalar, const D: usize> BoundPoints<N, D> for RangeTo<Point<N, D>> {
    #[inline]
    fn start_point(&self) -> Point<N, D> {
        Point::min_value()
    }

    #[inline]
    fn end_point(&self) -> Point<N, D> {
        self.end
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
                (..point![5, 5]).start_point(),
                Point::min_value()
            );
        }

        #[test]
        fn test_end_point() {
            assert_eq!(
                (..point![5, 5]).end_point(),
                point![5, 5]
            );
        }
    }
}