use std::ops::Bound::Unbounded;
use std::ops::RangeFull;
use na::{Point, Scalar};
use num_traits::Bounded;

use crate::{BBox, BoundPoints};

/// Builds a bounding box from a range of points
///
/// # Example
/// ```
/// use std::ops::Bound::Unbounded;
/// use nalgebra::point;
/// use pythagore::BBox;
///
/// assert_eq!(
///     BBox::<i32, 2>::from(..),
///     BBox::from([
///        (Unbounded, Unbounded),
///        (Unbounded, Unbounded),
///     ])
/// )
/// ```
impl<N: Copy + Scalar, const D: usize> From<RangeFull> for BBox<N, D> {
    fn from(_value: RangeFull) -> Self {
        BBox::from([(Unbounded, Unbounded); D])
    }
}

impl<N: Bounded + Scalar, const D: usize> BoundPoints<N, D> for RangeFull {
    #[inline]
    fn start_point(&self) -> Point<N, D> {
        Point::min_value()
    }

    #[inline]
    fn end_point(&self) -> Point<N, D> {
        Point::max_value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod bound_points {
        use super::*;

        #[test]
        fn test_start_point() {
            assert_eq!(
                (..).start_point(),
                Point::<i32, 2>::min_value()
            );
        }

        #[test]
        fn test_end_point() {
            assert_eq!(
                (..).end_point(),
                Point::<i32, 2>::max_value()
            );
        }
    }
}