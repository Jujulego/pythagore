use std::ops::Bound::*;
use std::ops::{Bound, Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

use na::{Point, Scalar};
use crate::bbox::bbox_nd::BBox;
use crate::traits::BBoxBounded;

// Implementations
impl<N: Scalar, const D: usize> BBoxBounded<N, D> for RangeFull {
    fn bbox(&self) -> BBox<'_, N, D> {
        BBox::from([(Unbounded, Unbounded); D])
    }
}

impl<N: Scalar, const D: usize> BBoxBounded<N, D> for RangeFrom<Point<N, D>> {
    fn bbox(&self) -> BBox<'_, N, D> {
        self.start.iter()
            .map(|start| (Included(start), Unbounded))
            .collect()
    }
}

impl<N: Scalar, const D: usize> BBoxBounded<N, D> for RangeTo<Point<N, D>> {
    fn bbox(&self) -> BBox<'_, N, D> {
        self.end.iter()
            .map(|end| (Unbounded, Excluded(end)))
            .collect()
    }
}

impl<N: Scalar, const D: usize> BBoxBounded<N, D> for Range<Point<N, D>> {
    fn bbox(&self) -> BBox<'_, N, D> {
        self.start.iter()
            .zip(self.end.iter())
            .map(|(start, end)| (Included(start), Excluded(end)))
            .collect()
    }
}

impl<N: Scalar, const D: usize> BBoxBounded<N, D> for RangeInclusive<Point<N, D>> {
    fn bbox(&self) -> BBox<'_, N, D> {
        self.start().iter()
            .zip(self.end().iter())
            .map(|(start, end)| (Included(start), Included(end)))
            .collect()
    }
}

impl<N: Scalar, const D: usize> BBoxBounded<N, D> for RangeToInclusive<Point<N, D>> {
    fn bbox(&self) -> BBox<'_, N, D> {
        self.end.iter()
            .map(|end| (Unbounded, Included(end)))
            .collect()
    }
}

impl<N: Scalar, const D: usize> BBoxBounded<N, D> for (Bound<Point<N, D>>, Bound<Point<N, D>>) {
    fn bbox(&self) -> BBox<'_, N, D> {
        let mut bounds = [(Unbounded, Unbounded); D];

        for d in 0..D {
            bounds[d].0 = match self.start_bound() {
                Included(pt) => Included(&pt[d]),
                Excluded(pt) => Excluded(&pt[d]),
                Unbounded => Unbounded,
            };

            bounds[d].1 = match self.end_bound() {
                Included(pt) => Included(&pt[d]),
                Excluded(pt) => Excluded(&pt[d]),
                Unbounded => Unbounded,
            };
        }

        BBox::from(bounds)
    }
}

// Tests
#[cfg(test)]
mod tests {
    use na::{point, Point};
    use super::*;

    #[test]
    fn range_full_box_contains() {
        assert!((..).bbox().contains(&point![1, 1]));
    }

    #[test]
    fn range_from_box_contains() {
        let range = Point::origin()..;

        assert!(range.bbox().contains(&point![1, 1]));
        assert!(range.bbox().contains(&Point::origin()));

        assert!(!range.bbox().contains(&point![1, -1]));
        assert!(!range.bbox().contains(&point![-1, 1]));
        assert!(!range.bbox().contains(&point![-1, -1]));
    }

    #[test]
    fn range_to_box_contains() {
        let range = ..Point::origin();

        assert!(range.bbox().contains(&point![-1, -1]));

        assert!(!range.bbox().contains(&Point::origin()));
        assert!(!range.bbox().contains(&point![-1, 1]));
        assert!(!range.bbox().contains(&point![1, -1]));
        assert!(!range.bbox().contains(&point![1, 1]));
    }

    #[test]
    fn range_box_contains() {
        let range = Point::origin()..point![5, 5];

        assert!(range.bbox().contains(&point![2, 2]));
        assert!(range.bbox().contains(&Point::origin()));

        assert!(!range.bbox().contains(&point![1, -1]));
        assert!(!range.bbox().contains(&point![-1, 1]));
        assert!(!range.bbox().contains(&point![-1, -1]));

        assert!(!range.bbox().contains(&point![1, 5]));
        assert!(!range.bbox().contains(&point![5, 1]));
        assert!(!range.bbox().contains(&point![5, 5]));
    }

    #[test]
    fn range_inclusive_box_contains() {
        let range = Point::origin()..=point![5, 5];

        assert!(range.bbox().contains(&point![2, 2]));
        assert!(range.bbox().contains(&Point::origin()));
        assert!(range.bbox().contains(&point![5, 5]));

        assert!(!range.bbox().contains(&point![1, -1]));
        assert!(!range.bbox().contains(&point![-1, 1]));
        assert!(!range.bbox().contains(&point![-1, -1]));

        assert!(!range.bbox().contains(&point![1, 6]));
        assert!(!range.bbox().contains(&point![6, 1]));
        assert!(!range.bbox().contains(&point![6, 6]));
    }

    #[test]
    fn range_to_inclusive_box_contains() {
        let range = ..=point![5, 5];

        assert!(range.bbox().contains(&point![-1, -1]));
        assert!(range.bbox().contains(&point![5, 5]));

        assert!(!range.bbox().contains(&point![1, 6]));
        assert!(!range.bbox().contains(&point![6, 1]));
        assert!(!range.bbox().contains(&point![6, 6]));
    }
}