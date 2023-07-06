mod bound_tuple;
mod range;
mod range_from;
mod range_full;
mod range_inclusive;
mod range_to;
mod range_to_inclusive;
mod utils;

use std::cmp::{max, min};
use std::ops::{Bound, Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::slice::{Iter, IterMut};
use na::{ClosedAdd, ClosedSub, Point, Scalar, SVector};
use num_traits::{One, Zero};
use crate::{Holds, Intersection, IsRangeEmpty, PointBounds, Walkable};
use crate::bbox::utils::{max_bound, min_bound};

type BBoxElement<N> = (Bound<N>, Bound<N>);

/// Generic Axis Aligned Bounding Box
/// Supports all kinds of bounds, independently on each axis
#[derive(Clone, Copy, Debug, Eq)]
pub struct BBox<N: Scalar, const D: usize> {
    ranges: [BBoxElement<N>; D],
}

impl<N: Scalar, const D: usize> BBox<N, D> {
    /// Builds a bounding box from two unordered points
    ///
    /// # Example
    /// ```
    /// use std::ops::Bound::{Excluded, Included};
    /// use nalgebra::point;
    /// use pythagore::BBox;
    ///
    /// assert_eq!(
    ///     BBox::from_points(&point![1, 4], &point![3, 2]),
    ///     BBox::from([
    ///        (Included(1), Excluded(3)),
    ///        (Included(2), Excluded(4)),
    ///     ])
    /// );
    /// ```
    pub fn from_points(a: &Point<N, D>, b: &Point<N, D>) -> BBox<N, D>
    where
        N: Copy + Ord
    {
        let mut ranges = [(Unbounded, Unbounded); D];

        for (idx, range) in ranges.iter_mut().enumerate() {
            range.0 = Included(*min(unsafe { a.get_unchecked(idx) }, unsafe { b.get_unchecked(idx) }));
            range.1 = Excluded(*max(unsafe { a.get_unchecked(idx) }, unsafe { b.get_unchecked(idx) }));
        }

        BBox {
            ranges
        }
    }

    /// Builds a bounding box from a point and a vector
    ///
    /// # Example
    /// ```
    /// use std::ops::Bound::{Excluded, Included};
    /// use nalgebra::{point, vector};
    /// use pythagore::BBox;
    ///
    /// assert_eq!(
    ///     BBox::from_anchor_size(&point![1, 1], &vector![3, -2]),
    ///     BBox::from([
    ///        (Included(1), Excluded(4)),
    ///        (Included(-1), Excluded(1)),
    ///     ])
    /// );
    /// ```
    pub fn from_anchor_size(anchor: &Point<N, D>, size: &SVector<N, D>) -> BBox<N, D>
    where
        N: ClosedAdd + Copy + Ord
    {
        BBox::from_points(anchor, &(anchor + size))
    }

    /// Builds an including bounding box from two unordered points
    ///
    /// # Example
    /// ```
    /// use std::ops::Bound::Included;
    /// use nalgebra::point;
    /// use pythagore::BBox;
    ///
    /// assert_eq!(
    ///     BBox::from_points_included(&point![1, 4], &point![3, 2]),
    ///     BBox::from([
    ///        (Included(1), Included(3)),
    ///        (Included(2), Included(4)),
    ///     ])
    /// );
    /// ```
    pub fn from_points_included(a: &Point<N, D>, b: &Point<N, D>) -> BBox<N, D>
    where
        N: Copy + Ord
    {
        let mut ranges = [(Unbounded, Unbounded); D];

        for (idx, range) in ranges.iter_mut().enumerate() {
            range.0 = Included(*min(unsafe { a.get_unchecked(idx) }, unsafe { b.get_unchecked(idx) }));
            range.1 = Included(*max(unsafe { a.get_unchecked(idx) }, unsafe { b.get_unchecked(idx) }));
        }

        BBox {
            ranges
        }
    }

    /// Builds an including bounding box from a point and a vector
    ///
    /// # Example
    /// ```
    /// use std::ops::Bound::Included;
    /// use nalgebra::{point, vector};
    /// use pythagore::BBox;
    ///
    /// assert_eq!(
    ///     BBox::from_anchor_size_included(&point![1, 1], &vector![3, -2]),
    ///     BBox::from([
    ///        (Included(1), Included(4)),
    ///        (Included(-1), Included(1)),
    ///     ])
    /// );
    /// ```
    pub fn from_anchor_size_included(anchor: &Point<N, D>, size: &SVector<N, D>) -> BBox<N, D>
    where
        N: ClosedAdd + Copy + Ord
    {
        BBox::from_points_included(anchor, &(anchor + size))
    }

    /// Returns a reference to an internal range, without doing bounds checking.
    ///
    /// # Safety
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*
    /// even if the resulting reference is not used.
    ///
    /// # Example
    /// ```
    /// use std::ops::Bound::{Excluded, Included};
    /// use nalgebra::point;
    /// use pythagore::BBox;
    ///
    /// let bbox = BBox::from(point![1, 2]..point![3, 4]);
    ///
    /// unsafe {
    ///     assert_eq!(bbox.get_unchecked(0), &(Included(1), Excluded(3)));
    /// }
    /// ```
    #[inline]
    pub unsafe fn get_unchecked(&self, idx: usize) -> &BBoxElement<N> {
        self.ranges.get_unchecked(idx)
    }

    /// Returns a mutable reference to an internal range, without doing bounds checking.
    ///
    /// # Safety
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*
    /// even if the resulting reference is not used.
    ///
    /// # Example
    /// ```
    /// use std::ops::Bound::{Excluded, Included, Unbounded};
    /// use nalgebra::point;
    /// use pythagore::BBox;
    ///
    /// let mut bbox = BBox::from(point![1, 2]..point![3, 4]);
    ///
    /// unsafe {
    ///     *bbox.get_unchecked_mut(0) = (Unbounded, Excluded(0))
    /// }
    ///
    /// assert_eq!(
    ///     bbox,
    ///     BBox::from([
    ///        (Unbounded, Excluded(0)),
    ///        (Included(2), Excluded(4)),
    ///     ])
    /// );
    /// ```
    #[inline]
    pub unsafe fn get_unchecked_mut(&mut self, idx: usize) -> &mut BBoxElement<N> {
        self.ranges.get_unchecked_mut(idx)
    }

    /// Returns iterator over internal ranges
    #[inline]
    pub fn iter(&self) -> Iter<BBoxElement<N>> {
        self.ranges.iter()
    }

    /// Returns mutable iterator over internal ranges
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<BBoxElement<N>> {
        self.ranges.iter_mut()
    }
}

// Utils
/// Default is a fully unbounded bbox
///
/// # Example
/// ```
/// use std::ops::Bound::Unbounded;
/// use pythagore::BBox;
///
/// assert_eq!(
///     BBox::<i32, 2>::default(),
///     BBox::from([
///        (Unbounded, Unbounded),
///        (Unbounded, Unbounded),
///     ])
/// );
/// ```
impl<N: Copy + Scalar, const D: usize> Default for BBox<N, D> {
    fn default() -> Self {
        BBox {
            ranges: [(Unbounded, Unbounded); D]
        }
    }
}

/// Checks if bbox holds given point
///
/// # Example
/// ```
/// use nalgebra::point;
/// use pythagore::{BBox, Holds};
///
/// assert!(BBox::from(point![0, 0]..point![5, 5]).holds(&point![2, 2]));
/// ```
impl<N: Scalar + PartialOrd, const D: usize> Holds<Point<N, D>> for BBox<N, D> {
    fn holds(&self, object: &Point<N, D>) -> bool {
        self.ranges.iter().enumerate()
            .all(|(idx, range)| range.holds(unsafe { object.get_unchecked(idx) }))
    }
}

/// Returns true if bounding box cannot hold any point
///
/// # Example
/// ```
/// use nalgebra::point;
/// use pythagore::{BBox, IsRangeEmpty};
///
/// assert!(BBox::from(point![5, 5]..point![0, 0]).is_range_empty());
/// ```
impl<N: Scalar + PartialOrd, const D: usize> IsRangeEmpty for BBox<N, D> {
    fn is_range_empty(&self) -> bool {
        self.ranges.iter().any(|range| range.is_range_empty())
    }
}

impl<N: Copy + Scalar + Zero, const D: usize> PointBounds<N, D> for BBox<N, D> {
    fn start_point(&self) -> Option<Point<N, D>> {
        let mut point = Point::<N, D>::default();

        for (idx, range) in self.ranges.iter().enumerate() {
            if let Included(x) | Excluded(x) = range.0 {
                unsafe { *point.get_unchecked_mut(idx) = x };
            } else {
                return None
            }
        }

        Some(point)
    }

    fn end_point(&self) -> Option<Point<N, D>> {
        let mut point = Point::<N, D>::default();

        for (idx, range) in self.ranges.iter().enumerate() {
            if let Included(x) | Excluded(x) = range.1 {
                unsafe { *point.get_unchecked_mut(idx) = x };
            } else {
                return None
            }
        }

        Some(point)
    }
}

impl<N: ClosedAdd + ClosedSub + Copy + One + Scalar + Zero, const D: usize> Walkable<N, D> for BBox<N, D> {
    fn first_point(&self) -> Option<Point<N, D>> {
        let mut point = Point::<N, D>::default();

        for (idx, range) in self.ranges.iter().enumerate() {
            match range.0 {
                Included(x) => unsafe { *point.get_unchecked_mut(idx) = x },
                Excluded(x) => unsafe { *point.get_unchecked_mut(idx) = x + N::one() },
                Unbounded => return None,
            }
        }

        Some(point)
    }

    fn last_point(&self) -> Option<Point<N, D>> {
        let mut point = Point::<N, D>::default();

        for (idx, range) in self.ranges.iter().enumerate() {
            match range.1 {
                Included(x) => unsafe { *point.get_unchecked_mut(idx) = x },
                Excluded(x) => unsafe { *point.get_unchecked_mut(idx) = x - N::one() },
                Unbounded => return None,
            }
        }

        Some(point)
    }
}

impl<N: Copy + PartialOrd + Scalar, const D: usize> Intersection for BBox<N, D> {
    type Output = BBox<N, D>;

    fn intersection(&self, lhs: &Self) -> Self::Output {
        let mut ranges = [(Unbounded, Unbounded); D];

        for (idx, range) in ranges.iter_mut().enumerate() {
            let rhs = unsafe { self.get_unchecked(idx) };
            let lhs = unsafe { lhs.get_unchecked(idx) };

            range.0 = *max_bound(&rhs.0, &lhs.0);
            range.1 = *min_bound(&rhs.1, &lhs.1);
        }

        BBox::from(ranges)
    }
}

impl<N: Copy + PartialOrd + Scalar, const D: usize> Intersection<Range<Point<N, D>>> for BBox<N, D> {
    type Output = BBox<N, D>;

    fn intersection(&self, lhs: &Range<Point<N, D>>) -> Self::Output {
        let mut ranges = [(Unbounded, Unbounded); D];

        for (idx, range) in ranges.iter_mut().enumerate() {
            let rhs = unsafe { self.get_unchecked(idx) };

            range.0 = *max_bound(&rhs.0, &Included(unsafe { *lhs.start.get_unchecked(idx) }));
            range.1 = *min_bound(&rhs.1, &Excluded(unsafe { *lhs.end.get_unchecked(idx) }));
        }

        BBox::from(ranges)
    }
}

impl<N: Copy + PartialOrd + Scalar, const D: usize> Intersection<RangeFrom<Point<N, D>>> for BBox<N, D> {
    type Output = BBox<N, D>;

    fn intersection(&self, lhs: &RangeFrom<Point<N, D>>) -> Self::Output {
        let mut ranges = [(Unbounded, Unbounded); D];

        for (idx, range) in ranges.iter_mut().enumerate() {
            let rhs = unsafe { self.get_unchecked(idx) };

            range.0 = *max_bound(&rhs.0, &Included(unsafe { *lhs.start.get_unchecked(idx) }));
            range.1 = rhs.1;
        }

        BBox::from(ranges)
    }
}

impl<N: Copy + Scalar, const D: usize> Intersection<RangeFull> for BBox<N, D> {
    type Output = BBox<N, D>;

    #[inline]
    fn intersection(&self, _: &RangeFull) -> Self::Output {
        *self
    }
}

impl<N: Copy + PartialOrd + Scalar, const D: usize> Intersection<RangeInclusive<Point<N, D>>> for BBox<N, D> {
    type Output = BBox<N, D>;

    fn intersection(&self, lhs: &RangeInclusive<Point<N, D>>) -> Self::Output {
        let mut ranges = [(Unbounded, Unbounded); D];

        for (idx, range) in ranges.iter_mut().enumerate() {
            let rhs = unsafe { self.get_unchecked(idx) };

            range.0 = *max_bound(&rhs.0, &Included(unsafe { *lhs.start().get_unchecked(idx) }));
            range.1 = *min_bound(&rhs.1, &Included(unsafe { *lhs.end().get_unchecked(idx) }));
        }

        BBox::from(ranges)
    }
}

impl<N: Copy + PartialOrd + Scalar, const D: usize> Intersection<RangeTo<Point<N, D>>> for BBox<N, D> {
    type Output = BBox<N, D>;

    fn intersection(&self, lhs: &RangeTo<Point<N, D>>) -> Self::Output {
        let mut ranges = [(Unbounded, Unbounded); D];

        for (idx, range) in ranges.iter_mut().enumerate() {
            let rhs = unsafe { self.get_unchecked(idx) };

            range.0 = rhs.0;
            range.1 = *min_bound(&rhs.1, &Excluded(unsafe { *lhs.end.get_unchecked(idx) }));
        }

        BBox::from(ranges)
    }
}

impl<N: Copy + PartialOrd + Scalar, const D: usize> Intersection<RangeToInclusive<Point<N, D>>> for BBox<N, D> {
    type Output = BBox<N, D>;

    fn intersection(&self, lhs: &RangeToInclusive<Point<N, D>>) -> Self::Output {
        let mut ranges = [(Unbounded, Unbounded); D];

        for (idx, range) in ranges.iter_mut().enumerate() {
            let rhs = unsafe { self.get_unchecked(idx) };

            range.0 = rhs.0;
            range.1 = *min_bound(&rhs.1, &Included(unsafe { *lhs.end.get_unchecked(idx) }));
        }

        BBox::from(ranges)
    }
}

// Conversion
impl<N: Scalar, const D: usize> AsRef<[BBoxElement<N>; D]> for BBox<N, D> {
    #[inline]
    fn as_ref(&self) -> &[BBoxElement<N>; D] {
        &self.ranges
    }
}

impl<N: Scalar, const D: usize> AsMut<[BBoxElement<N>; D]> for BBox<N, D> {
    #[inline]
    fn as_mut(&mut self) -> &mut [BBoxElement<N>; D] {
        &mut self.ranges
    }
}

/// Builds a bounding box from a set of ranges
impl<N: Scalar, const D: usize> From<[BBoxElement<N>; D]> for BBox<N, D> {
    fn from(ranges: [BBoxElement<N>; D]) -> Self {
        BBox {
            ranges
        }
    }
}

// Operators
impl<N: Scalar, const D: usize> Index<usize> for BBox<N, D> {
    type Output = BBoxElement<N>;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.ranges[index]
    }
}

impl<N: Scalar, const D: usize> IndexMut<usize> for BBox<N, D> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.ranges[index]
    }
}

impl<N: Scalar, const D: usize> PartialEq for BBox<N, D> {
    fn eq(&self, other: &Self) -> bool {
        self.ranges == other.ranges
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    mod holds {
        use na::point;
        use super::*;

        #[test]
        fn test_all_point_coords_in_ranges() {
            assert!(BBox::from(point![0, 0]..point![5, 5]).holds(&point![2, 2]));
        }

        #[test]
        fn test_some_point_coords_lower_than_start() {
            assert!(!BBox::from(point![0, 0]..point![5, 5]).holds(&point![-2, 2]));
            assert!(!BBox::from(point![0, 0]..point![5, 5]).holds(&point![2, -2]));
        }

        #[test]
        fn test_some_point_coords_greater_than_end() {
            assert!(!BBox::from(point![0, 0]..point![5, 5]).holds(&point![7, 2]));
            assert!(!BBox::from(point![0, 0]..point![5, 5]).holds(&point![2, 7]));
        }
    }

    mod is_range_empty {
        use na::point;
        use super::*;

        #[test]
        fn test_all_start_coords_lower_than_end_coords() {
            assert!(!BBox::from(point![0, 0]..point![5, 5]).is_range_empty());
        }

        #[test]
        fn test_some_start_coords_greater_than_end_coords() {
            assert!(BBox::from(point![5, 0]..point![0, 5]).is_range_empty());
            assert!(BBox::from(point![0, 5]..point![5, 0]).is_range_empty());
        }

        #[test]
        fn test_some_start_coords_equals_end_coords() {
            assert!(BBox::from(point![0, 5]..point![5, 5]).is_range_empty());
            assert!(BBox::from(point![5, 0]..point![5, 5]).is_range_empty());

            assert!(!BBox::from(point![5, 0]..=point![5, 5]).is_range_empty());
            assert!(!BBox::from(point![0, 5]..=point![5, 5]).is_range_empty());
        }
    }

    mod point_bounds {
        use na::point;
        use super::*;

        #[test]
        fn test_start_point() {
            assert_eq!(
                BBox::from(point![0, 0]..point![5, 5]).start_point(),
                Some(point![0, 0])
            );

            assert_eq!(
                BBox::from(..point![5, 5]).start_point(),
                None
            );
        }

        #[test]
        fn test_end_point() {
            assert_eq!(
                BBox::from(point![0, 0]..point![5, 5]).end_point(),
                Some(point![5, 5])
            );

            assert_eq!(
                BBox::from(point![0, 0]..).end_point(),
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
                BBox::from(point![0, 0]..point![5, 5]).first_point(),
                Some(point![0, 0])
            );

            assert_eq!(
                BBox::from([(Included(0), Excluded(5)), (Excluded(0), Excluded(5))]).first_point(),
                Some(point![0, 1])
            );

            assert_eq!(
                BBox::from(..point![5, 5]).first_point(),
                None
            );
        }

        #[test]
        fn test_last_point() {
            assert_eq!(
                BBox::from(point![0, 0]..point![5, 5]).last_point(),
                Some(point![4, 4])
            );

            assert_eq!(
                BBox::from([(Included(0), Included(5)), (Included(0), Excluded(5))]).last_point(),
                Some(point![5, 4])
            );

            assert_eq!(
                BBox::from(point![0, 0]..).last_point(),
                None
            );
        }
    }
}