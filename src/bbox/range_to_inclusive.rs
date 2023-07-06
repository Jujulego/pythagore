use std::ops::Bound::{Excluded, Included, Unbounded};
use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};
use na::{Point, Scalar};

use crate::{BBox, Intersection, PointBounds};
use crate::bbox::utils::min_point;

/// Builds a bounding box from a range of points
///
/// # Example
/// ```
/// use std::ops::Bound::{Included, Unbounded};
/// use nalgebra::point;
/// use pythagore::BBox;
///
/// assert_eq!(
///     BBox::from(..=point![3, 4]),
///     BBox::from([
///        (Unbounded, Included(3)),
///        (Unbounded, Included(4)),
///     ])
/// )
/// ```
impl<N: Copy + Scalar, const D: usize> From<RangeToInclusive<Point<N, D>>> for BBox<N, D> {
    fn from(value: RangeToInclusive<Point<N, D>>) -> Self {
        let mut ranges = [(Unbounded, Unbounded); D];

        for (idx, range) in ranges.iter_mut().enumerate() {
            range.1 = Included(unsafe { *value.end.get_unchecked(idx) });
        }

        BBox::from(ranges)
    }
}

impl<N: Copy + Scalar, const D: usize> PointBounds<N, D> for RangeToInclusive<Point<N, D>> {
    #[inline]
    fn start_point(&self) -> Option<Point<N, D>> {
        None
    }

    #[inline]
    fn end_point(&self) -> Option<Point<N, D>> {
        Some(self.end)
    }
}

impl<N: Copy + PartialOrd + Scalar, const D: usize> Intersection<BBox<N, D>> for RangeToInclusive<Point<N, D>> {
    type Output = BBox<N, D>;

    #[inline]
    fn intersection(&self, lhs: &BBox<N, D>) -> Self::Output {
        lhs.intersection(self)
    }
}

impl<N: Copy + Ord + Scalar, const D: usize> Intersection<Range<Point<N, D>>> for RangeToInclusive<Point<N, D>> {
    type Output = BBox<N, D>;

    fn intersection(&self, lhs: &Range<Point<N, D>>) -> Self::Output {
        let mut ranges = [(Unbounded, Unbounded); D];

        for (idx, range) in ranges.iter_mut().enumerate() {
            range.0 = Included(*unsafe { lhs.start.get_unchecked(idx) });

            let rex = unsafe { self.end.get_unchecked(idx) };
            let lex = unsafe { lhs.end.get_unchecked(idx) };

            if rex < lex {
                range.1 = Included(*rex);
            } else {
                range.1 = Excluded(*lex);
            }
        }

        BBox::from(ranges)
    }
}

impl<N: Copy + Default + Ord + Scalar, const D: usize> Intersection<RangeFrom<Point<N, D>>> for RangeToInclusive<Point<N, D>> {
    type Output = RangeInclusive<Point<N, D>>;

    #[inline]
    fn intersection(&self, lhs: &RangeFrom<Point<N, D>>) -> Self::Output {
        lhs.start..=self.end
    }
}

impl<N: Scalar, const D: usize> Intersection<RangeFull> for RangeToInclusive<Point<N, D>> {
    type Output = RangeToInclusive<Point<N, D>>;

    #[inline]
    fn intersection(&self, _: &RangeFull) -> Self::Output {
        self.clone()
    }
}

impl<N: Copy + Default + Ord + Scalar, const D: usize> Intersection<RangeInclusive<Point<N, D>>> for RangeToInclusive<Point<N, D>> {
    type Output = RangeInclusive<Point<N, D>>;

    #[inline]
    fn intersection(&self, lhs: &RangeInclusive<Point<N, D>>) -> Self::Output {
        *lhs.start()..=min_point(&self.end, lhs.end())
    }
}

impl<N: Copy + Default + Ord + Scalar, const D: usize> Intersection<RangeTo<Point<N, D>>> for RangeToInclusive<Point<N, D>> {
    type Output = BBox<N, D>;

    fn intersection(&self, lhs: &RangeTo<Point<N, D>>) -> Self::Output {
        let mut ranges = [(Unbounded, Unbounded); D];

        for (idx, range) in ranges.iter_mut().enumerate() {
            let rex = unsafe { self.end.get_unchecked(idx) };
            let lex = unsafe { lhs.end.get_unchecked(idx) };

            if rex < lex {
                range.1 = Included(*rex);
            } else {
                range.1 = Excluded(*lex);
            }
        }

        BBox::from(ranges)
    }
}

impl<N: Copy + Default + Ord + Scalar, const D: usize> Intersection for RangeToInclusive<Point<N, D>> {
    type Output = RangeToInclusive<Point<N, D>>;

    #[inline]
    fn intersection(&self, lhs: &RangeToInclusive<Point<N, D>>) -> Self::Output {
        ..=min_point(&self.end, &lhs.end)
    }
}

#[cfg(test)]
mod tests {
    use na::point;
    use super::*;

    #[test]
    fn test_intersection() {
        assert_eq!((..=point![10, 15]).intersection(&(point![5, 0]..point![15, 10])), BBox::from([
            (Included(5), Included(10)),
            (Included(0), Excluded(10)),
        ]));
        assert_eq!((..=point![10, 15]).intersection(&(point![5, 0]..)), point![5, 0]..=point![10, 15]);
        assert_eq!((..=point![10, 15]).intersection(&(..)), ..=point![10, 15]);
        assert_eq!((..=point![10, 15]).intersection(&(point![5, 0]..=point![15, 10])), point![5, 0]..=point![10, 10]);
        assert_eq!((..=point![10, 15]).intersection(&(..point![15, 10])), BBox::from([
            (Unbounded, Included(10)),
            (Unbounded, Excluded(10)),
        ]));
        assert_eq!((..=point![10, 15]).intersection(&(..=point![15, 10])), ..=point![10, 10]);
    }

    mod point_bounds {
        use na::point;
        use super::*;

        #[test]
        fn test_start_point() {
            assert_eq!(
                (..=point![5, 5]).start_point(),
                None
            );
        }

        #[test]
        fn test_end_point() {
            assert_eq!(
                (..=point![5, 5]).end_point(),
                Some(point![5, 5])
            );
        }
    }
}