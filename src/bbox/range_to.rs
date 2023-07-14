use std::ops::Bound::{Excluded, Included, Unbounded};
use std::ops::{Bound, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};
use na::{Point, Scalar};

use crate::{BBox, Intersection, PointBounds};
use crate::bbox::utils::{min_bound, min_point};
use crate::traits::DimensionBounds;

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

impl<N: Copy + Scalar, const D: usize> DimensionBounds<N, D> for RangeTo<Point<N, D>> {
    type Output = RangeTo<N>;

    #[inline]
    unsafe fn get_bounds_unchecked(&self, idx: usize) -> Self::Output {
        ..*self.end.get_unchecked(idx)
    }
}

impl<N: Copy + Scalar, const D: usize> PointBounds<N, D> for RangeTo<Point<N, D>> {
    #[inline]
    fn start_point(&self) -> Option<Point<N, D>> {
        None
    }

    #[inline]
    fn end_point(&self) -> Option<Point<N, D>> {
        Some(self.end)
    }
}

impl<N: Copy + PartialOrd + Scalar, const D: usize> Intersection<BBox<N, D>> for RangeTo<Point<N, D>> {
    type Output = BBox<N, D>;

    #[inline]
    fn intersection(&self, rhs: &BBox<N, D>) -> Self::Output {
        rhs.intersection(self)
    }
}

impl<N: Copy + Default + Ord + Scalar, const D: usize> Intersection<Range<Point<N, D>>> for RangeTo<Point<N, D>> {
    type Output = Range<Point<N, D>>;

    #[inline]
    fn intersection(&self, rhs: &Range<Point<N, D>>) -> Self::Output {
        rhs.start..min_point(&self.end, &rhs.end)
    }
}

impl<N: Copy + Scalar, const D: usize> Intersection<RangeFrom<Point<N, D>>> for RangeTo<Point<N, D>> {
    type Output = Range<Point<N, D>>;

    #[inline]
    fn intersection(&self, rhs: &RangeFrom<Point<N, D>>) -> Self::Output {
        rhs.start..self.end
    }
}

impl<N: Scalar, const D: usize> Intersection<RangeFull> for RangeTo<Point<N, D>> {
    type Output = RangeTo<Point<N, D>>;

    #[inline]
    fn intersection(&self, _: &RangeFull) -> Self::Output {
        self.clone()
    }
}

impl<N: Copy + PartialOrd + Scalar, const D: usize> Intersection<RangeInclusive<Point<N, D>>> for RangeTo<Point<N, D>> {
    type Output = BBox<N, D>;

    fn intersection(&self, rhs: &RangeInclusive<Point<N, D>>) -> Self::Output {
        let mut ranges = [(Unbounded, Unbounded); D];

        for (idx, range) in ranges.iter_mut().enumerate() {
            range.0 = Included(*unsafe { rhs.start().get_unchecked(idx) });

            let lex = unsafe { self.end.get_unchecked(idx) };
            let rex = unsafe { rhs.end().get_unchecked(idx) };

            range.1 = if lex <= rex { Excluded(*lex) } else { Included(*rex) };
        }

        BBox::from(ranges)
    }
}

impl<N: Copy + Default + Ord + Scalar, const D: usize> Intersection for RangeTo<Point<N, D>> {
    type Output = RangeTo<Point<N, D>>;

    #[inline]
    fn intersection(&self, rhs: &RangeTo<Point<N, D>>) -> Self::Output {
        ..min_point(&self.end, &rhs.end)
    }
}

impl<N: Copy + PartialOrd + Scalar, const D: usize> Intersection<RangeToInclusive<Point<N, D>>> for RangeTo<Point<N, D>> {
    type Output = BBox<N, D>;

    fn intersection(&self, rhs: &RangeToInclusive<Point<N, D>>) -> Self::Output {
        let mut ranges = [(Unbounded, Unbounded); D];

        for (idx, range) in ranges.iter_mut().enumerate() {
            let rex = unsafe { self.end.get_unchecked(idx) };
            let lex = unsafe { rhs.end.get_unchecked(idx) };

            range.1 = if rex <= lex { Excluded(*rex) } else { Included(*lex) };
        }

        BBox::from(ranges)
    }
}

impl<N: Copy + PartialOrd + Scalar, const D: usize> Intersection<(Bound<Point<N, D>>, Bound<Point<N, D>>)> for RangeTo<Point<N, D>> {
    type Output = BBox<N, D>;

    fn intersection(&self, rhs: &(Bound<Point<N, D>>, Bound<Point<N, D>>)) -> Self::Output {
        let mut ranges = [(Unbounded, Unbounded); D];

        for (idx, range) in ranges.iter_mut().enumerate() {
            let rhs = unsafe { rhs.get_bounds_unchecked(idx) };

            range.0 = rhs.0;
            range.1 = min_bound(Excluded(unsafe { *self.end.get_unchecked(idx) }), rhs.1);
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
        assert_eq!((..point![10, 15]).intersection(&(point![5, 0]..point![15, 10])), point![5, 0]..point![10, 10]);
        assert_eq!((..point![10, 15]).intersection(&(point![5, 0]..)), point![5, 0]..point![10, 15]);
        assert_eq!((..point![10, 15]).intersection(&(..)), ..point![10, 15]);
        assert_eq!((..point![10, 15]).intersection(&(point![5, 0]..=point![15, 10])), BBox::from([
            (Included(5), Excluded(10)),
            (Included(0), Included(10)),
        ]));
        assert_eq!((..point![10, 15]).intersection(&(..point![15, 10])), ..point![10, 10]);
        assert_eq!((..point![10, 15]).intersection(&(..=point![15, 10])), BBox::from([
            (Unbounded, Excluded(10)),
            (Unbounded, Included(10)),
        ]));
    }

    mod dimension_bounds {
        use na::point;
        use super::*;

        #[test]
        fn test_get_bounds() {
            assert_eq!((..point![3, 4]).get_bounds(0), ..3);
            assert_eq!((..point![3, 4]).get_bounds(1), ..4);
        }
    }

    mod point_bounds {
        use na::point;
        use super::*;

        #[test]
        fn test_start_point() {
            assert_eq!(
                (..point![5, 5]).start_point(),
                None
            );
        }

        #[test]
        fn test_end_point() {
            assert_eq!(
                (..point![5, 5]).end_point(),
                Some(point![5, 5])
            );
        }
    }
}