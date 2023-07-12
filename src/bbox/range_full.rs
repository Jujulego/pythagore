use std::ops::Bound::Unbounded;
use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};
use na::{Point, Scalar};

use crate::{BBox, Intersection, PointBounds};
use crate::traits::DimensionBounds;

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

impl<N: Scalar, const D: usize> DimensionBounds<N, D> for RangeFull {
    type Output = RangeFull;

    #[inline]
    fn get_bounds(&self, _idx: usize) -> Self::Output {
        ..
    }

    #[inline]
    unsafe fn get_bounds_unchecked(&self, _idx: usize) -> Self::Output {
        ..
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

impl<N: Copy + Scalar, const D: usize> Intersection<BBox<N, D>> for RangeFull {
    type Output = BBox<N, D>;

    #[inline]
    fn intersection(&self, lhs: &BBox<N, D>) -> Self::Output {
        *lhs
    }
}

impl<N: Copy + Scalar, const D: usize> Intersection<Range<Point<N, D>>> for RangeFull {
    type Output = Range<Point<N, D>>;

    #[inline]
    fn intersection(&self, lhs: &Range<Point<N, D>>) -> Self::Output {
        lhs.clone()
    }
}

impl<N: Copy + Scalar, const D: usize> Intersection<RangeFrom<Point<N, D>>> for RangeFull {
    type Output = RangeFrom<Point<N, D>>;

    #[inline]
    fn intersection(&self, lhs: &RangeFrom<Point<N, D>>) -> Self::Output {
        lhs.clone()
    }
}

impl Intersection for RangeFull {
    type Output = RangeFull;

    #[inline]
    fn intersection(&self, _: &RangeFull) -> Self::Output {
        *self
    }
}

impl<N: Copy + Scalar, const D: usize> Intersection<RangeInclusive<Point<N, D>>> for RangeFull {
    type Output = RangeInclusive<Point<N, D>>;

    #[inline]
    fn intersection(&self, lhs: &RangeInclusive<Point<N, D>>) -> Self::Output {
        lhs.clone()
    }
}

impl<N: Copy + Scalar, const D: usize> Intersection<RangeTo<Point<N, D>>> for RangeFull {
    type Output = RangeTo<Point<N, D>>;

    #[inline]
    fn intersection(&self, lhs: &RangeTo<Point<N, D>>) -> Self::Output {
        *lhs
    }
}

impl<N: Copy + Scalar, const D: usize> Intersection<RangeToInclusive<Point<N, D>>> for RangeFull {
    type Output = RangeToInclusive<Point<N, D>>;

    #[inline]
    fn intersection(&self, lhs: &RangeToInclusive<Point<N, D>>) -> Self::Output {
        *lhs
    }
}

// Tests
#[cfg(test)]
mod tests {
    use na::point;
    use super::*;

    #[test]
    fn test_intersection() {
        assert_eq!((..).intersection(&(point![5, 0]..point![15, 10])), point![5, 0]..point![15, 10]);
        assert_eq!((..).intersection(&(point![5, 0]..)), point![5, 0]..);
        assert_eq!((..).intersection(&(..)), ..);
        assert_eq!((..).intersection(&(point![5, 0]..=point![15, 10])), point![5, 0]..=point![15, 10]);
        assert_eq!((..).intersection(&(..point![15, 10])), ..point![15, 10]);
        assert_eq!((..).intersection(&(..=point![15, 10])), ..=point![15, 10]);
    }

    mod dimension_bounds {
        use super::*;

        #[test]
        fn test_get_bounds() {
            assert_eq!(DimensionBounds::<i32, 2>::get_bounds(&(..), 0), ..);
            assert_eq!(DimensionBounds::<i32, 2>::get_bounds(&(..), 1), ..);
        }
    }

    mod point_bounds {
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