use std::cmp::max;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::ops::{Bound, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};
use na::{ClosedSub, Point, Scalar, SVector};
use num_traits::One;

use crate::{BBox, Intersection, PointBounds, Walkable};
use crate::bbox::utils::{max_bound, max_point, min_bound, min_point};
use crate::traits::DimensionBounds;

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

impl<N: Copy + Scalar, const D: usize> DimensionBounds<N, D> for Range<Point<N, D>> {
    type Output = Range<N>;

    #[inline]
    unsafe fn get_bounds_unchecked(&self, idx: usize) -> Self::Output {
        *self.start.get_unchecked(idx)..*self.end.get_unchecked(idx)
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

impl<N: Copy + PartialOrd + Scalar, const D: usize> Intersection<BBox<N, D>> for Range<Point<N, D>> {
    type Output = BBox<N, D>;

    #[inline]
    fn intersection(&self, lhs: &BBox<N, D>) -> Self::Output {
        lhs.intersection(self)
    }
}

impl<N: Copy + Default + Ord + Scalar, const D: usize> Intersection for Range<Point<N, D>> {
    type Output = Range<Point<N, D>>;

    #[inline]
    fn intersection(&self, rhs: &Range<Point<N, D>>) -> Self::Output {
        max_point(&self.start, &rhs.start)..min_point(&self.end, &rhs.end)
    }
}

impl<N: Copy + Default + Ord + Scalar, const D: usize> Intersection<RangeFrom<Point<N, D>>> for Range<Point<N, D>> {
    type Output = Range<Point<N, D>>;

    #[inline]
    fn intersection(&self, rhs: &RangeFrom<Point<N, D>>) -> Self::Output {
        max_point(&self.start, &rhs.start)..self.end
    }
}

impl<N: Scalar, const D: usize> Intersection<RangeFull> for Range<Point<N, D>> {
    type Output = Range<Point<N, D>>;

    #[inline]
    fn intersection(&self, _: &RangeFull) -> Self::Output {
        self.clone()
    }
}

impl<N: Copy + Ord + Scalar, const D: usize> Intersection<RangeInclusive<Point<N, D>>> for Range<Point<N, D>> {
    type Output = BBox<N, D>;

    fn intersection(&self, rhs: &RangeInclusive<Point<N, D>>) -> Self::Output {
        let mut ranges = [(Unbounded, Unbounded); D];

        for (idx, range) in ranges.iter_mut().enumerate() {
            let lsx = unsafe { self.start.get_unchecked(idx) };
            let rsx = unsafe { rhs.start().get_unchecked(idx) };

            range.0 = Included(*max(lsx, rsx));

            let lex = unsafe { self.end.get_unchecked(idx) };
            let rex = unsafe { rhs.end().get_unchecked(idx) };

            range.1 = if lex <= rex { Excluded(*lex) } else { Included(*rex) }
        }

        BBox::from(ranges)
    }
}

impl<N: Copy + Default + Ord + Scalar, const D: usize> Intersection<RangeTo<Point<N, D>>> for Range<Point<N, D>> {
    type Output = Range<Point<N, D>>;

    #[inline]
    fn intersection(&self, rhs: &RangeTo<Point<N, D>>) -> Self::Output {
        self.start..min_point(&self.end, &rhs.end)
    }
}

impl<N: Copy + Ord + Scalar, const D: usize> Intersection<RangeToInclusive<Point<N, D>>> for Range<Point<N, D>> {
    type Output = BBox<N, D>;

    fn intersection(&self, rhs: &RangeToInclusive<Point<N, D>>) -> Self::Output {
        let mut ranges = [(Unbounded, Unbounded); D];

        for (idx, range) in ranges.iter_mut().enumerate() {
            range.0 = Included(*unsafe { self.start.get_unchecked(idx) });

            let lex = unsafe { self.end.get_unchecked(idx) };
            let rex = unsafe { rhs.end.get_unchecked(idx) };

            range.1 = if lex <= rex { Excluded(*lex) } else { Included(*rex) }
        }

        BBox::from(ranges)
    }
}

impl<N: Copy + Ord + Scalar, const D: usize> Intersection<(Bound<Point<N, D>>, Bound<Point<N, D>>)> for Range<Point<N, D>> {
    type Output = BBox<N, D>;

    fn intersection(&self, rhs: &(Bound<Point<N, D>>, Bound<Point<N, D>>)) -> Self::Output {
        let mut ranges = [(Unbounded, Unbounded); D];

        for (idx, range) in ranges.iter_mut().enumerate() {
            let rhs = unsafe { rhs.get_bounds_unchecked(idx) };

            range.0 = max_bound(Included(unsafe { *self.start.get_unchecked(idx) }), rhs.0);
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
        assert_eq!((point![0, 5]..point![10, 15]).intersection(&(point![5, 0]..point![15, 10])), point![5, 5]..point![10, 10]);
        assert_eq!((point![0, 5]..point![10, 15]).intersection(&(point![5, 0]..)), point![5, 5]..point![10, 15]);
        assert_eq!((point![0, 5]..point![10, 15]).intersection(&(..)), point![0, 5]..point![10, 15]);
        assert_eq!((point![0, 5]..point![10, 15]).intersection(&(point![5, 0]..=point![15, 10])), BBox::from([
            (Included(5), Excluded(10)),
            (Included(5), Included(10)),
        ]));
        assert_eq!((point![0, 5]..point![10, 15]).intersection(&(..point![15, 10])), point![0, 5]..point![10, 10]);
        assert_eq!((point![0, 5]..point![10, 15]).intersection(&(..=point![15, 10])), BBox::from([
            (Included(0), Excluded(10)),
            (Included(5), Included(10)),
        ]));
        assert_eq!((point![0, 5]..point![10, 15]).intersection(&(Excluded(point![5, 0]), Included(point![15, 10]))), BBox::from([
            (Excluded(5), Excluded(10)),
            (Included(5), Included(10)),
        ]));
    }

    mod dimension_bounds {
        use na::point;
        use super::*;

        #[test]
        fn test_get_bounds() {
            assert_eq!((point![1, 2]..point![3, 4]).get_bounds(0), 1..3);
            assert_eq!((point![1, 2]..point![3, 4]).get_bounds(1), 2..4);
        }
    }

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