use std::cmp::max;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::ops::{Bound, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};
use na::{Point, Scalar};

use crate::{BBox, Intersection, PointBounds, Walkable};
use crate::bbox::utils::{max_bound, max_point, min_bound, min_point};
use crate::traits::DimensionBounds;

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

impl<N: Copy + Scalar, const D: usize> DimensionBounds<N, D> for RangeInclusive<Point<N, D>> {
    type Output = RangeInclusive<N>;

    #[inline]
    unsafe fn get_bounds_unchecked(&self, idx: usize) -> Self::Output {
        *self.start().get_unchecked(idx)..=*self.end().get_unchecked(idx)
    }
}

impl<N: Copy + Scalar, const D: usize> PointBounds<N, D> for RangeInclusive<Point<N, D>> {
    #[inline]
    fn start_point(&self) -> Option<Point<N, D>> {
        Some(*self.start())
    }

    #[inline]
    fn end_point(&self) -> Option<Point<N, D>> {
        Some(*self.end())
    }
}

impl<N: Copy + Scalar, const D: usize> Walkable<N, D> for RangeInclusive<Point<N, D>> {
    #[inline]
    fn first_point(&self) -> Option<Point<N, D>> {
        Some(*self.start())
    }

    #[inline]
    fn last_point(&self) -> Option<Point<N, D>> {
        Some(*self.end())
    }
}

impl<N: Copy + PartialOrd + Scalar, const D: usize> Intersection<BBox<N, D>> for RangeInclusive<Point<N, D>> {
    type Output = BBox<N, D>;

    #[inline]
    fn intersection(&self, lhs: &BBox<N, D>) -> Self::Output {
        lhs.intersection(self)
    }
}

impl<N: Copy + Ord + Scalar, const D: usize> Intersection<Range<Point<N, D>>> for RangeInclusive<Point<N, D>> {
    type Output = BBox<N, D>;

    fn intersection(&self, lhs: &Range<Point<N, D>>) -> Self::Output {
        let mut ranges = [(Unbounded, Unbounded); D];

        for (idx, range) in ranges.iter_mut().enumerate() {
            let rsx = unsafe { self.start().get_unchecked(idx) };
            let lsx = unsafe { lhs.start.get_unchecked(idx) };

            range.0 = Included(*max(rsx, lsx));

            let rex = unsafe { self.end().get_unchecked(idx) };
            let lex = unsafe { lhs.end.get_unchecked(idx) };

            range.1 = if rex < lex { Included(*rex) } else { Excluded(*lex) };
        }

        BBox::from(ranges)
    }
}

impl<N: Copy + Default + Ord + Scalar, const D: usize> Intersection<RangeFrom<Point<N, D>>> for RangeInclusive<Point<N, D>> {
    type Output = RangeInclusive<Point<N, D>>;

    #[inline]
    fn intersection(&self, lhs: &RangeFrom<Point<N, D>>) -> Self::Output {
        max_point(self.start(), &lhs.start)..=*self.end()
    }
}

impl<N: Scalar, const D: usize> Intersection<RangeFull> for RangeInclusive<Point<N, D>> {
    type Output = RangeInclusive<Point<N, D>>;

    #[inline]
    fn intersection(&self, _: &RangeFull) -> Self::Output {
        self.clone()
    }
}

impl<N: Copy + Default + Ord + Scalar, const D: usize> Intersection for RangeInclusive<Point<N, D>> {
    type Output = RangeInclusive<Point<N, D>>;

    #[inline]
    fn intersection(&self, lhs: &RangeInclusive<Point<N, D>>) -> Self::Output {
        max_point(self.start(), lhs.start())..=min_point(self.end(), lhs.end())
    }
}

impl<N: Copy + Default + Ord + Scalar, const D: usize> Intersection<RangeTo<Point<N, D>>> for RangeInclusive<Point<N, D>> {
    type Output = BBox<N, D>;

    fn intersection(&self, lhs: &RangeTo<Point<N, D>>) -> Self::Output {
        let mut ranges = [(Unbounded, Unbounded); D];

        for (idx, range) in ranges.iter_mut().enumerate() {
            range.0 = Included(*unsafe { self.start().get_unchecked(idx) });

            let rex = unsafe { self.end().get_unchecked(idx) };
            let lex = unsafe { lhs.end.get_unchecked(idx) };

            range.1 = if rex < lex { Included(*rex) } else { Excluded(*lex) };
        }

        BBox::from(ranges)
    }
}

impl<N: Copy + Default + Ord + Scalar, const D: usize> Intersection<RangeToInclusive<Point<N, D>>> for RangeInclusive<Point<N, D>> {
    type Output = RangeInclusive<Point<N, D>>;

    #[inline]
    fn intersection(&self, lhs: &RangeToInclusive<Point<N, D>>) -> Self::Output {
        *self.start()..=min_point(self.end(), &lhs.end)
    }
}

impl<N: Copy + Ord + Scalar, const D: usize> Intersection<(Bound<Point<N, D>>, Bound<Point<N, D>>)> for RangeInclusive<Point<N, D>> {
    type Output = BBox<N, D>;

    fn intersection(&self, lhs: &(Bound<Point<N, D>>, Bound<Point<N, D>>)) -> Self::Output {
        let mut ranges = [(Unbounded, Unbounded); D];

        for (idx, range) in ranges.iter_mut().enumerate() {
            let (start, end) = unsafe { lhs.get_bounds_unchecked(idx) };

            range.0 = min_bound(start, Included(unsafe { *self.start().get_unchecked(idx) }));
            range.1 = max_bound(end, Included(unsafe { *self.end().get_unchecked(idx) }));
        }

        BBox::from(ranges)
    }
}

#[cfg(test)]
mod tests {
    use na::point;
    use super::*;

    #[test]
    fn test_intersection() {
        assert_eq!((point![0, 5]..=point![10, 15]).intersection(&(point![5, 0]..point![15, 10])), BBox::from([
            (Included(5), Included(10)),
            (Included(5), Excluded(10)),
        ]));
        assert_eq!((point![0, 5]..=point![10, 15]).intersection(&(point![5, 0]..)), point![5, 5]..=point![10, 15]);
        assert_eq!((point![0, 5]..=point![10, 15]).intersection(&(..)), point![0, 5]..=point![10, 15]);
        assert_eq!((point![0, 5]..=point![10, 15]).intersection(&(point![5, 0]..=point![15, 10])), point![5, 5]..=point![10, 10]);
        assert_eq!((point![0, 5]..=point![10, 15]).intersection(&(..point![15, 10])), BBox::from([
            (Included(0), Included(10)),
            (Included(5), Excluded(10)),
        ]));
        assert_eq!((point![0, 5]..=point![10, 15]).intersection(&(..=point![15, 10])), point![0, 5]..=point![10, 10]);
    }

    mod dimension_bounds {
        use na::point;
        use super::*;

        #[test]
        fn test_get_bounds() {
            assert_eq!((point![1, 2]..=point![3, 4]).get_bounds(0), 1..=3);
            assert_eq!((point![1, 2]..=point![3, 4]).get_bounds(1), 2..=4);
        }
    }

    mod point_bounds {
        use na::point;
        use super::*;

        #[test]
        fn test_start_point() {
            assert_eq!(
                (point![0, 0]..=point![5, 5]).start_point(),
                Some(point![0, 0])
            );
        }

        #[test]
        fn test_end_point() {
            assert_eq!(
                (point![0, 0]..=point![5, 5]).end_point(),
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
                (point![0, 0]..=point![5, 5]).first_point(),
                Some(point![0, 0])
            );
        }

        #[test]
        fn test_last_point() {
            assert_eq!(
                (point![0, 0]..=point![5, 5]).last_point(),
                Some(point![5, 5])
            );
        }
    }
}