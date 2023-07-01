mod range;
mod utils;

use na::{max, min, ClosedAdd, Point, SVector, Scalar};
use std::hash::{Hash, Hasher};
use std::ops::Bound::{self as Bound, *};
use std::ops::{Index, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

use crate::bbox::utils::*;
use crate::traits::{BoundingBox, IsRangeEmpty};

/// `BBox<N, D>` structure for D dimension axe aligned bounding boxes
#[derive(Clone, Copy, Debug, Eq)]
pub struct BBox<N: Scalar, const D: usize> {
    ranges: [(Bound<N>, Bound<N>); D],
}

// Methods
impl<N: Scalar, const D: usize> BBox<N, D> {
    /// Builds a new bbox from a BoundingBox object
    pub fn from_bounding_box<B: BoundingBox<N, D>>(bbox: &B) -> BBox<N, D>
    where
        N: Copy,
    {
        let mut ranges = [(Unbounded, Unbounded); D];

        for (dim, pair) in ranges.iter_mut().enumerate() {
            let range = bbox.get_range(dim);

            pair.0 = range.0.cloned();
            pair.1 = range.1.cloned();
        }

        BBox { ranges }
    }

    /// Builds a bbox from a point and a size
    /// Roughly the same as `(anchor..anchor + size).bbox()`
    pub fn from_anchor_size(anchor: &Point<N, D>, size: &SVector<N, D>) -> BBox<N, D>
    where
        N: ClosedAdd + Copy + Ord,
    {
        BBox::from_points(anchor, &(anchor + size))
    }

    /// Builds a bbox from two points
    /// Roughly the same as `(start..end).bbox()`
    pub fn from_points(start: &Point<N, D>, end: &Point<N, D>) -> BBox<N, D>
    where
        N: Copy + Ord,
    {
        let mut result = BBox::default();

        for (dim, pair) in result.ranges.iter_mut().enumerate() {
            pair.0 = Included(min(start[dim], end[dim]));
            pair.1 = Excluded(max(start[dim], end[dim]));
        }

        result
    }

    /// Builds an including bbox from a point and a size
    /// Roughly the same as `(anchor..=anchor + size).bbox()`
    pub fn from_anchor_size_including(anchor: &Point<N, D>, size: &SVector<N, D>) -> BBox<N, D>
    where
        N: ClosedAdd + Copy + Ord,
    {
        BBox::from_points_including(anchor, &(anchor + size))
    }

    /// Builds a bbox from two points
    /// Roughly the same as `(start..end).bbox()`
    pub fn from_points_including(start: &Point<N, D>, end: &Point<N, D>) -> BBox<N, D>
    where
        N: Copy + Ord,
    {
        let mut result = BBox::default();

        for (dim, pair) in result.ranges.iter_mut().enumerate() {
            pair.0 = Included(min(start[dim], end[dim]));
            pair.1 = Included(max(start[dim], end[dim]));
        }

        result
    }

    /// Returns a new bbox including the given point
    pub fn include(&self, pt: &Point<N, D>) -> BBox<N, D>
    where
        N: Copy + PartialOrd,
    {
        let mut result = BBox::default();

        for (dim, pair) in result.ranges.iter_mut().enumerate() {
            pair.0 = include_value(&self.ranges[dim].0, &pt[dim], |a, b| a < b);
            pair.1 = include_value(&self.ranges[dim].1, &pt[dim], |a, b| a > b);
        }

        result
    }
}

// Utils
impl<N: Scalar, const D: usize> BoundingBox<N, D> for BBox<N, D> {
    fn get_range(&self, d: usize) -> (Bound<&N>, Bound<&N>) {
        (self.ranges[d].0.as_ref(), self.ranges[0].1.as_ref())
    }
}

impl<N: Copy + Scalar, const D: usize> Default for BBox<N, D> {
    fn default() -> Self {
        BBox {
            ranges: [(Unbounded, Unbounded); D],
        }
    }
}

impl<N: Scalar + Hash, const D: usize> Hash for BBox<N, D> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ranges.hash(state);
    }
}

impl<N: Scalar + PartialOrd, const D: usize> IsRangeEmpty for BBox<N, D> {
    fn is_range_empty(&self) -> bool {
        self.ranges.iter().any(|r| r.is_range_empty())
    }
}

// Conversions
impl<N: Scalar, const D: usize> From<[(Bound<N>, Bound<N>); D]> for BBox<N, D> {
    fn from(bounds: [(Bound<N>, Bound<N>); D]) -> Self {
        BBox { ranges: bounds }
    }
}

impl<N: Copy + Scalar, const D: usize> From<RangeFull> for BBox<N, D> {
    fn from(_: RangeFull) -> Self {
        BBox {
            ranges: [(Unbounded, Unbounded); D]
        }
    }
}

impl<N: Copy + Scalar, const D: usize> From<RangeFrom<Point<N, D>>> for BBox<N, D> {
    fn from(range: RangeFrom<Point<N, D>>) -> Self {
        let mut bounds = [(Unbounded, Unbounded); D];

        for (dim, pair) in bounds.iter_mut().enumerate() {
            pair.0 = Included(range.start[dim]);
        }

        BBox { ranges: bounds }
    }
}

impl<N: Copy + Scalar, const D: usize> From<RangeTo<Point<N, D>>> for BBox<N, D> {
    fn from(range: RangeTo<Point<N, D>>) -> Self {
        let mut bounds = [(Unbounded, Unbounded); D];

        for (dim, pair) in bounds.iter_mut().enumerate() {
            pair.1 = Excluded(range.end[dim]);
        }

        BBox { ranges: bounds }
    }
}

impl<N: Copy + Scalar, const D: usize> From<RangeToInclusive<Point<N, D>>> for BBox<N, D> {
    fn from(range: RangeToInclusive<Point<N, D>>) -> Self {
        let mut bounds = [(Unbounded, Unbounded); D];

        for (dim, pair) in bounds.iter_mut().enumerate() {
            pair.1 = Included(range.end[dim]);
        }

        BBox { ranges: bounds }
    }
}

impl<N: Copy + Scalar, const D: usize> From<Range<Point<N, D>>> for BBox<N, D> {
    fn from(range: Range<Point<N, D>>) -> Self {
        let mut bounds = [(Unbounded, Unbounded); D];

        for (dim, pair) in bounds.iter_mut().enumerate() {
            pair.0 = Included(range.start[dim]);
            pair.1 = Excluded(range.end[dim]);
        }

        BBox { ranges: bounds }
    }
}

impl<N: Copy + Scalar, const D: usize> From<RangeInclusive<Point<N, D>>> for BBox<N, D> {
    fn from(range: RangeInclusive<Point<N, D>>) -> Self {
        let mut bounds = [(Unbounded, Unbounded); D];

        for (dim, pair) in bounds.iter_mut().enumerate() {
            pair.0 = Included(range.start()[dim]);
            pair.1 = Included(range.end()[dim]);
        }

        BBox { ranges: bounds }
    }
}

// Operators
impl<N: Scalar, B: BoundingBox<N, D>, const D: usize> PartialEq<B> for BBox<N, D> {
    fn eq(&self, other: &B) -> bool {
        self.ranges.iter().enumerate()
            .all(|(d, range)| {
                let oth = other.get_range(d);
                range.0.as_ref() == oth.0 && range.1.as_ref() == oth.1
            })
    }
}

impl<N: Scalar, const D: usize> Index<usize> for BBox<N, D> {
    type Output = (Bound<N>, Bound<N>);

    fn index(&self, index: usize) -> &Self::Output {
        &self.ranges[index]
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use na::{point, vector};

    #[test]
    fn bbox_from_anchor_size() {
        assert_eq!(
            BBox::from_anchor_size(&point![1, 1], &vector![2, 3]),
            point![1, 1]..point![3, 4]
        );

        // With messy coords
        assert_eq!(
            BBox::from_anchor_size(&point![1, 4], &vector![2, -3]),
            point![1, 1]..point![3, 4]
        );
    }

    #[test]
    fn bbox_from_points() {
        assert_eq!(
            BBox::from_points(&point![1, 1], &point![2, 3]),
            point![1, 1]..point![2, 3]
        );

        // With messy coords
        assert_eq!(
            BBox::from_points(&point![1, 3], &point![2, 1]),
            point![1, 1]..point![2, 3]
        );
    }

    #[test]
    fn bbox_from_anchor_size_including() {
        assert_eq!(
            BBox::from_anchor_size_including(&point![1, 1], &vector![2, 3]),
            point![1, 1]..=point![3, 4]
        );

        // With messy coords
        assert_eq!(
            BBox::from_anchor_size_including(&point![1, 4], &vector![2, -3]),
           point![1, 1]..=point![3, 4]
        );
    }

    #[test]
    fn bbox_from_points_including() {
        assert_eq!(
            BBox::from_points_including(&point![1, 1], &point![2, 3]),
            point![1, 1]..=point![2, 3]
        );

        // With messy coords
        assert_eq!(
            BBox::from_points_including(&point![1, 3], &point![2, 1]),
            point![1, 1]..=point![2, 3]
        );
    }

    #[test]
    fn bbox_is_empty() {
        let a: BBox<u32, 3> = [
            (Included(0), Included(5)),
            (Included(0), Included(5)),
            (Included(7), Included(5)),
        ]
        .into();

        assert!(a.is_range_empty());
    }

    #[test]
    fn bbox_is_not_empty() {
        let a: BBox<u32, 3> = [
            (Included(0), Included(5)),
            (Included(0), Included(5)),
            (Included(0), Included(5)),
        ]
        .into();

        assert!(!a.is_range_empty());
    }

    #[test]
    fn bbox_contains() {
        let a: BBox<i32, 2> = [(Included(0), Included(5)), (Included(0), Included(5))].into();

        assert!(a.holds(&point![2, 2]));

        assert!(!a.holds(&point![-3, 2]));
        assert!(!a.holds(&point![-3, -3]));
        assert!(!a.holds(&point![2, -3]));
        assert!(!a.holds(&point![7, -3]));
        assert!(!a.holds(&point![7, 2]));
        assert!(!a.holds(&point![7, 7]));
        assert!(!a.holds(&point![2, 7]));
        assert!(!a.holds(&point![-3, 7]));
    }

    #[test]
    fn bbox_intersection_overlaps() {
        let a: BBox<u32, 1> = [(Included(0), Included(5))].into();
        let b: BBox<u32, 1> = [(Included(2), Included(7))].into();

        assert_eq!(a.intersection(&b), BBox::from([(Included(2), Included(5))]));
    }

    #[test]
    fn bbox_intersection_contains() {
        let a: BBox<u32, 1> = [(Included(0), Included(7))].into();
        let b: BBox<u32, 1> = [(Included(2), Included(5))].into();

        assert_eq!(a.intersection(&b), b);
    }

    #[test]
    fn bbox_intersection_no_intersection() {
        let a: BBox<u32, 1> = [(Included(0), Included(2))].into();
        let b: BBox<u32, 1> = [(Included(5), Included(7))].into();

        assert_eq!(a.intersection(&b), BBox::from([(Included(5), Included(2))]));
    }

    #[test]
    fn bbox_intersection_some_included_some_excluded() {
        let a: BBox<u32, 1> = [(Included(0), Included(5))].into();
        let b: BBox<u32, 1> = [(Excluded(2), Excluded(7))].into();

        assert_eq!(a.intersection(&b), BBox::from([(Excluded(2), Included(5))]));
    }

    #[test]
    fn bbox_intersection_some_unbounded() {
        let a: BBox<u32, 1> = [(Included(0), Unbounded)].into();
        let b: BBox<u32, 1> = [(Unbounded, Excluded(7))].into();

        assert_eq!(a.intersection(&b), BBox::from([(Included(0), Excluded(7))]));
    }

    #[test]
    fn bbox_intersection_one_fully_unbounded() {
        let a: BBox<u32, 1> = [(Included(0), Included(5))].into();
        let b: BBox<u32, 1> = [(Unbounded, Unbounded)].into();

        assert_eq!(a.intersection(&b), a);
    }

    #[test]
    fn bbox_intersection_no_bounds() {
        let a: BBox<u32, 1> = [(Unbounded, Unbounded)].into();
        let b: BBox<u32, 1> = [(Unbounded, Unbounded)].into();

        assert_eq!(a.intersection(&b), a);
    }

    #[test]
    fn bbox_include() {
        let range = BBox::from(point![2]..point![6]);

        assert_eq!(
            range.include(&point![0]),
            BBox::from(point![0]..point![6])
        );
        assert_eq!(
            range.include(&point![4]),
            BBox::from(point![2]..point![6])
        );
        assert_eq!(
            range.include(&point![6]),
            BBox::from(point![2]..=point![6])
        );
        assert_eq!(
            range.include(&point![8]),
            BBox::from(point![2]..=point![8])
        );
        assert_eq!(BBox::from(..).include(&point![8]), BBox::from(..));
    }

    #[test]
    fn bbox_start_point() {
        assert_eq!(BBox::from(..).start_point(), point![i32::MIN, i32::MIN]);
        assert_eq!(BBox::from(point![0, 0]..).start_point(), point![0, 0]);
        assert_eq!(
            BBox::from(point![0, 0]..point![5, 5]).start_point(),
            point![0, 0]
        );
        assert_eq!(
            BBox::from(point![0, 0]..=point![5, 5]).start_point(),
            point![0, 0]
        );
        assert_eq!(
            BBox::from(..point![5, 5]).start_point(),
            point![i32::MIN, i32::MIN]
        );
        assert_eq!(
            BBox::from(..=point![5, 5]).start_point(),
            point![i32::MIN, i32::MIN]
        );
    }

    #[test]
    fn bbox_end_point() {
        assert_eq!(BBox::from(..).end_point(), point![i32::MAX, i32::MAX]);
        assert_eq!(
            BBox::from(point![0, 0]..).end_point(),
            point![i32::MAX, i32::MAX]
        );
        assert_eq!(
            BBox::from(point![0, 0]..point![5, 5]).end_point(),
            point![5, 5]
        );
        assert_eq!(
            BBox::from(point![0, 0]..=point![5, 5]).end_point(),
            point![5, 5]
        );
        assert_eq!(BBox::from(..point![5, 5]).end_point(), point![5, 5]);
        assert_eq!(BBox::from(..=point![5, 5]).end_point(), point![5, 5]);
    }

    #[test]
    fn bbox_center_point() {
        assert_eq!(
            BBox::from(point![0.0, 0.0]..point![6.0, 6.0]).center_point(),
            point![3.0, 3.0]
        );
    }

    #[test]
    fn bbox_size() {
        assert_eq!(
            BBox::from(point![0.0, 0.0]..point![6.0, 6.0]).size(),
            vector![6.0, 6.0]
        );
    }
}
