use std::ops::Bound::{self, Excluded, Included, Unbounded};
use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};
use na::{ClosedAdd, ClosedSub, Point, Scalar, SVector};
use num_traits::One;

use crate::{BBox, Intersection, PointBounds, Walkable};
use crate::bbox::utils::{max_bound, min_bound};
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
///     BBox::from((Excluded(point![1, 2]), Included(point![3, 4]))),
///     BBox::from([
///        (Excluded(1), Included(3)),
///        (Excluded(2), Included(4)),
///     ])
/// )
/// ```
impl<N: Copy + Scalar, const D: usize> From<(Bound<Point<N, D>>, Bound<Point<N, D>>)> for BBox<N, D> {
    fn from(value: (Bound<Point<N, D>>, Bound<Point<N, D>>)) -> Self {
        let mut ranges = [(Unbounded, Unbounded); D];

        for (idx, range) in ranges.iter_mut().enumerate() {
            *range = unsafe { value.get_bounds_unchecked(idx) };
        }

        BBox::from(ranges)
    }
}

#[cfg(not(feature = "bound_map"))]
impl<N: Copy + Scalar, const D: usize> DimensionBounds<N, D> for (Bound<Point<N, D>>, Bound<Point<N, D>>) {
    type Output = (Bound<N>, Bound<N>);

    #[inline]
    unsafe fn get_bounds_unchecked(&self, idx: usize) -> Self::Output {
        (
            match &self.0 {
                Included(x) => Included(*x.get_unchecked(idx)),
                Excluded(x) => Excluded(*x.get_unchecked(idx)),
                Unbounded => Unbounded,
            },
            match &self.1 {
                Included(x) => Included(*x.get_unchecked(idx)),
                Excluded(x) => Excluded(*x.get_unchecked(idx)),
                Unbounded => Unbounded,
            }
        )
    }
}

#[cfg(feature = "bound_map")]
impl<N: Scalar, const D: usize> DimensionBounds<N, D> for (Bound<Point<N, D>>, Bound<Point<N, D>>) {
    type Output = (Bound<N>, Bound<N>);

    #[inline]
    unsafe fn get_bounds_unchecked(&self, idx: usize) -> Self::Output {
        (self.0.map(|x| *x.get_unchecked(idx)), self.1.map(|x| *x.get_unchecked(idx)))
    }
}

impl<N: Copy + Scalar, const D: usize> PointBounds<N, D> for (Bound<Point<N, D>>, Bound<Point<N, D>>) {
    fn start_point(&self) -> Option<Point<N, D>> {
        if let Included(pt) | Excluded(pt) = self.0 {
            Some(pt)
        } else {
            None
        }
    }

    fn end_point(&self) -> Option<Point<N, D>> {
        if let Included(pt) | Excluded(pt) = self.1 {
            Some(pt)
        } else {
            None
        }
    }
}

impl<N: ClosedAdd + ClosedSub + Copy + One + Scalar, const D: usize> Walkable<N, D> for (Bound<Point<N, D>>, Bound<Point<N, D>>) {
    fn first_point(&self) -> Option<Point<N, D>> {
        match self.0 {
            Included(pt) => Some(pt),
            Excluded(pt) => Some(pt + SVector::repeat(N::one())),
            Unbounded => None
        }
    }

    fn last_point(&self) -> Option<Point<N, D>> {
        match self.1 {
            Included(pt) => Some(pt),
            Excluded(pt) => Some(pt - SVector::repeat(N::one())),
            Unbounded => None
        }
    }
}

impl<N: Copy + Ord + Scalar, const D: usize> Intersection<Range<Point<N, D>>> for (Bound<Point<N, D>>, Bound<Point<N, D>>) {
    type Output = BBox<N, D>;

    fn intersection(&self, lhs: &Range<Point<N, D>>) -> Self::Output {
        lhs.intersection(self)
    }
}

impl<N: Copy + Ord + Scalar, const D: usize> Intersection<RangeFrom<Point<N, D>>> for (Bound<Point<N, D>>, Bound<Point<N, D>>) {
    type Output = BBox<N, D>;

    fn intersection(&self, lhs: &RangeFrom<Point<N, D>>) -> Self::Output {
        lhs.intersection(self)
    }
}

impl<N: Copy + Ord + Scalar, const D: usize> Intersection<RangeFull> for (Bound<Point<N, D>>, Bound<Point<N, D>>) {
    type Output = (Bound<Point<N, D>>, Bound<Point<N, D>>);

    fn intersection(&self, _: &RangeFull) -> Self::Output {
        *self
    }
}

impl<N: Copy + Ord + Scalar, const D: usize> Intersection<RangeInclusive<Point<N, D>>> for (Bound<Point<N, D>>, Bound<Point<N, D>>) {
    type Output = BBox<N, D>;

    fn intersection(&self, lhs: &RangeInclusive<Point<N, D>>) -> Self::Output {
        lhs.intersection(self)
    }
}

impl<N: Copy + Ord + Scalar, const D: usize> Intersection<RangeTo<Point<N, D>>> for (Bound<Point<N, D>>, Bound<Point<N, D>>) {
    type Output = BBox<N, D>;

    fn intersection(&self, lhs: &RangeTo<Point<N, D>>) -> Self::Output {
        lhs.intersection(self)
    }
}

impl<N: Copy + Ord + Scalar, const D: usize> Intersection<RangeToInclusive<Point<N, D>>> for (Bound<Point<N, D>>, Bound<Point<N, D>>) {
    type Output = BBox<N, D>;

    fn intersection(&self, lhs: &RangeToInclusive<Point<N, D>>) -> Self::Output {
        lhs.intersection(self)
    }
}

impl<N: Copy + Ord + Scalar, const D: usize> Intersection<(Bound<Point<N, D>>, Bound<Point<N, D>>)> for (Bound<Point<N, D>>, Bound<Point<N, D>>) {
    type Output = BBox<N, D>;

    fn intersection(&self, rhs: &(Bound<Point<N, D>>, Bound<Point<N, D>>)) -> Self::Output {
        let mut ranges = [(Unbounded, Unbounded); D];

        for (idx, range) in ranges.iter_mut().enumerate() {
            let lhs = unsafe { self.get_bounds_unchecked(idx) };
            let rhs = unsafe { rhs.get_bounds_unchecked(idx) };

            range.0 = min_bound(lhs.0, rhs.0);
            range.1 = max_bound(lhs.1, rhs.1);
        }

        BBox::from(ranges)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod dimension_bounds {
        use na::point;
        use super::*;

        #[test]
        fn test_get_bounds() {
            assert_eq!(
                (Excluded(point![1, 2]), Excluded(point![3, 4])).get_bounds(0),
                (Excluded(1), Excluded(3)),
            );
            assert_eq!(
                (Excluded(point![1, 2]), Excluded(point![3, 4])).get_bounds(1),
                (Excluded(2), Excluded(4)),
            );
        }
    }

    mod point_bounds {
        use na::point;
        use super::*;

        #[test]
        fn test_start_point() {
            assert_eq!(
                (Included(point![0, 0]), Excluded(point![5, 5])).start_point(),
                Some(point![0, 0])
            );

            assert_eq!(
                (Unbounded, Excluded(point![5, 5])).start_point(),
                None
            );
        }

        #[test]
        fn test_end_point() {
            assert_eq!(
                (Included(point![0, 0]), Excluded(point![5, 5])).end_point(),
                Some(point![5, 5])
            );

            assert_eq!(
                (Included(point![0, 0]), Unbounded).end_point(),
                None
            );
        }
    }

    mod walkable {
        use na::point;
        use super::*;

        #[test]
        fn test_first_point() {
            assert_eq!(
                (Included(point![0, 0]), Excluded(point![5, 5])).first_point(),
                Some(point![0, 0])
            );

            assert_eq!(
                (Excluded(point![0, 0]), Excluded(point![5, 5])).first_point(),
                Some(point![1, 1])
            );

            assert_eq!(
                (Unbounded, Excluded(point![5, 5])).first_point(),
                None
            );
        }

        #[test]
        fn test_last_point() {
            assert_eq!(
                (Included(point![0, 0]), Included(point![5, 5])).last_point(),
                Some(point![5, 5])
            );

            assert_eq!(
                (Included(point![0, 0]), Excluded(point![5, 5])).last_point(),
                Some(point![4, 4])
            );

            assert_eq!(
                (Included(point![0, 0]), Unbounded).last_point(),
                None
            );
        }
    }
}