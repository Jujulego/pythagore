use std::ops::Bound::{Included, Unbounded};
use std::ops::{Bound, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};
use na::{Point, Scalar};

use crate::{BBox, Intersection, PointBounds};
use crate::bbox::utils::{max_point, min_bound};
use crate::traits::DimensionBounds;

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

impl<N: Copy + Scalar, const D: usize> DimensionBounds<N, D> for RangeFrom<Point<N, D>> {
    type Output = RangeFrom<N>;

    #[inline]
    unsafe fn get_bounds_unchecked(&self, idx: usize) -> Self::Output {
        *self.start.get_unchecked(idx)..
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

impl<N: Copy + PartialOrd + Scalar, const D: usize> Intersection<BBox<N, D>> for RangeFrom<Point<N, D>> {
    type Output = BBox<N, D>;

    #[inline]
    fn intersection(&self, lhs: &BBox<N, D>) -> Self::Output {
        lhs.intersection(self)
    }
}

impl<N: Copy + Default + Ord + Scalar, const D: usize> Intersection<Range<Point<N, D>>> for RangeFrom<Point<N, D>> {
    type Output = Range<Point<N, D>>;

    #[inline]
    fn intersection(&self, lhs: &Range<Point<N, D>>) -> Self::Output {
        max_point(&self.start, &lhs.start)..lhs.end
    }
}

impl<N: Copy + Default + Ord + Scalar, const D: usize> Intersection for RangeFrom<Point<N, D>> {
    type Output = RangeFrom<Point<N, D>>;

    #[inline]
    fn intersection(&self, lhs: &RangeFrom<Point<N, D>>) -> Self::Output {
        max_point(&self.start, &lhs.start)..
    }
}

impl<N: Scalar, const D: usize> Intersection<RangeFull> for RangeFrom<Point<N, D>> {
    type Output = RangeFrom<Point<N, D>>;

    #[inline]
    fn intersection(&self, _: &RangeFull) -> Self::Output {
        self.clone()
    }
}

impl<N: Copy + Default + Ord + Scalar, const D: usize> Intersection<RangeInclusive<Point<N, D>>> for RangeFrom<Point<N, D>> {
    type Output = RangeInclusive<Point<N, D>>;

    #[inline]
    fn intersection(&self, lhs: &RangeInclusive<Point<N, D>>) -> Self::Output {
        max_point(&self.start, lhs.start())..=*lhs.end()
    }
}

impl<N: Copy + Default + Ord + Scalar, const D: usize> Intersection<RangeTo<Point<N, D>>> for RangeFrom<Point<N, D>> {
    type Output = Range<Point<N, D>>;

    #[inline]
    fn intersection(&self, lhs: &RangeTo<Point<N, D>>) -> Self::Output {
        self.start..lhs.end
    }
}

impl<N: Copy + Default + Ord + Scalar, const D: usize> Intersection<RangeToInclusive<Point<N, D>>> for RangeFrom<Point<N, D>> {
    type Output = RangeInclusive<Point<N, D>>;

    #[inline]
    fn intersection(&self, lhs: &RangeToInclusive<Point<N, D>>) -> Self::Output {
        self.start..=lhs.end
    }
}

impl<N: Copy + Ord + Scalar, const D: usize> Intersection<(Bound<Point<N, D>>, Bound<Point<N, D>>)> for RangeFrom<Point<N, D>> {
    type Output = BBox<N, D>;

    fn intersection(&self, lhs: &(Bound<Point<N, D>>, Bound<Point<N, D>>)) -> Self::Output {
        let mut ranges = [(Unbounded, Unbounded); D];

        for (idx, range) in ranges.iter_mut().enumerate() {
            let lhs = unsafe { lhs.get_bounds_unchecked(idx) };

            range.0 = min_bound(Included(unsafe { *self.start.get_unchecked(idx) }), lhs.0);
            range.1 = lhs.1;
        }

        BBox::from(ranges)
    }
}

// Tests
#[cfg(test)]
mod tests {
    use na::point;
    use super::*;

    #[test]
    fn test_intersection() {
        assert_eq!((point![0, 5]..).intersection(&(point![5, 0]..point![15, 10])), point![5, 5]..point![15, 10]);
        assert_eq!((point![0, 5]..).intersection(&(point![5, 0]..)), point![5, 5]..);
        assert_eq!((point![0, 5]..).intersection(&(..)), point![0, 5]..);
        assert_eq!((point![0, 5]..).intersection(&(point![5, 0]..=point![15, 10])), point![5, 5]..=point![15, 10]);
        assert_eq!((point![0, 5]..).intersection(&(..point![15, 10])), point![0, 5]..point![15, 10]);
        assert_eq!((point![0, 5]..).intersection(&(..=point![15, 10])), point![0, 5]..=point![15, 10]);
    }

    mod dimension_bounds {
        use na::point;
        use super::*;

        #[test]
        fn test_get_bounds() {
            assert_eq!((point![1, 2]..).get_bounds(0), 1..);
            assert_eq!((point![1, 2]..).get_bounds(1), 2..);
        }
    }

    mod point_bounds {
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