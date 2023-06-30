use na::{Point, Scalar};
use std::ops::Bound::{self, *};
use std::ops::{
    Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};

use super::BBox;
use crate::traits::{BBoxBounded, BoundingBox};

// Implementations
impl<N: Scalar, const D: usize> BoundingBox<N, D> for RangeFull {
    fn get_range(&self, _dim: usize) -> (Bound<&N>, Bound<&N>) {
        (Unbounded, Unbounded)
    }

    /// Always true
    fn holds(&self, _: &Point<N, D>) -> bool {
        true
    }
}

impl<N: Scalar, const D: usize> BoundingBox<N, D> for RangeFrom<Point<N, D>> {
    fn get_range(&self, d: usize) -> (Bound<&N>, Bound<&N>) {
        (Included(&self.start[d]), Unbounded)
    }
}

impl<N: Scalar, const D: usize> BoundingBox<N, D> for RangeTo<Point<N, D>> {
    fn get_range(&self, d: usize) -> (Bound<&N>, Bound<&N>) {
        (Unbounded, Excluded(&self.end[d]))
    }
}

impl<N: Scalar, const D: usize> BoundingBox<N, D> for RangeToInclusive<Point<N, D>> {
    fn get_range(&self, d: usize) -> (Bound<&N>, Bound<&N>) {
        (Unbounded, Included(&self.end[d]))
    }
}

impl<N: Scalar, const D: usize> BoundingBox<N, D> for Range<Point<N, D>> {
    fn get_range(&self, d: usize) -> (Bound<&N>, Bound<&N>) {
        (Included(&self.start[d]), Excluded(&self.end[d]))
    }
}

impl<N: Scalar, const D: usize> BoundingBox<N, D> for RangeInclusive<Point<N, D>> {
    fn get_range(&self, d: usize) -> (Bound<&N>, Bound<&N>) {
        (Included(&self.start()[d]), Included(&self.end()[d]))
    }
}

impl<N: Scalar, const D: usize> BoundingBox<N, D> for (Bound<Point<N, D>>, Bound<Point<N, D>>) {
    fn get_range(&self, d: usize) -> (Bound<&N>, Bound<&N>) {
        (
            match self.start_bound() {
                Included(pt) => Included(&pt[d]),
                Excluded(pt) => Excluded(&pt[d]),
                Unbounded => Unbounded,
            },
            match self.end_bound() {
                Included(pt) => Included(&pt[d]),
                Excluded(pt) => Excluded(&pt[d]),
                Unbounded => Unbounded,
            }
        )
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use na::{point, Point};

    #[test]
    fn range_full_box_contains() {
        assert!((..).holds(&point![1, 1]));
    }

    #[test]
    fn range_from_box_contains() {
        let range = Point::origin()..;

        assert!(range.holds(&point![1, 1]));
        assert!(range.holds(&Point::origin()));

        assert!(!range.holds(&point![1, -1]));
        assert!(!range.holds(&point![-1, 1]));
        assert!(!range.holds(&point![-1, -1]));
    }

    #[test]
    fn range_to_box_contains() {
        let range = ..Point::origin();

        assert!(range.holds(&point![-1, -1]));

        assert!(!range.holds(&Point::origin()));
        assert!(!range.holds(&point![-1, 1]));
        assert!(!range.holds(&point![1, -1]));
        assert!(!range.holds(&point![1, 1]));
    }

    #[test]
    fn range_box_contains() {
        let range = Point::origin()..point![5, 5];

        assert!(range.holds(&point![2, 2]));
        assert!(range.holds(&Point::origin()));

        assert!(!range.holds(&point![1, -1]));
        assert!(!range.holds(&point![-1, 1]));
        assert!(!range.holds(&point![-1, -1]));

        assert!(!range.holds(&point![1, 5]));
        assert!(!range.holds(&point![5, 1]));
        assert!(!range.holds(&point![5, 5]));
    }

    #[test]
    fn range_inclusive_box_contains() {
        let range = Point::origin()..=point![5, 5];

        assert!(range.holds(&point![2, 2]));
        assert!(range.holds(&Point::origin()));
        assert!(range.holds(&point![5, 5]));

        assert!(!range.holds(&point![1, -1]));
        assert!(!range.holds(&point![-1, 1]));
        assert!(!range.holds(&point![-1, -1]));

        assert!(!range.holds(&point![1, 6]));
        assert!(!range.holds(&point![6, 1]));
        assert!(!range.holds(&point![6, 6]));
    }

    #[test]
    fn range_to_inclusive_box_contains() {
        let range = ..=point![5, 5];

        assert!(range.holds(&point![-1, -1]));
        assert!(range.holds(&point![5, 5]));

        assert!(!range.holds(&point![1, 6]));
        assert!(!range.holds(&point![6, 1]));
        assert!(!range.holds(&point![6, 6]));
    }
}
