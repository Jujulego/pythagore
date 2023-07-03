use std::ops::Bound::Unbounded;
use std::ops::RangeFull;
use na::{Point, Scalar};

use crate::{BBox, PointBounds};

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

impl<N: Scalar, const D: usize> PointBounds<N, D> for RangeFull {
    #[inline]
    fn start_point(&self) -> Option<Point<N, D>> {
        None
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
        use super::*;

        #[test]
        fn test_start_point() {
            assert_eq!(
                (..).start_point(),
                None::<Point<i32, 2>>
            );
        }

        #[test]
        fn test_end_point() {
            assert_eq!(
                (..).end_point(),
                None::<Point<i32, 2>>
            );
        }
    }
}