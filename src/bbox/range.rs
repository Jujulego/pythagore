use na::{Point, Scalar};
use std::ops::Bound::{self, *};
use std::ops::{
    Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};

use super::BBox;
use crate::traits::BBoxBounded;

// Implementations
impl<N: Copy + Scalar, const D: usize> BBoxBounded<N, D> for RangeFull {
    fn bbox(&self) -> BBox<N, D> {
        BBox::from([(Unbounded, Unbounded); D])
    }
}

impl<N: Copy + Scalar, const D: usize> BBoxBounded<N, D> for RangeFrom<Point<N, D>> {
    fn bbox(&self) -> BBox<N, D> {
        let mut bounds = [(Unbounded, Unbounded); D];

        for (dim, pair) in bounds.iter_mut().enumerate() {
            pair.0 = Included(self.start[dim]);
        }

        BBox::from(bounds)
    }
}

impl<N: Copy + Scalar, const D: usize> BBoxBounded<N, D> for RangeTo<Point<N, D>> {
    fn bbox(&self) -> BBox<N, D> {
        let mut bounds = [(Unbounded, Unbounded); D];

        for (dim, pair) in bounds.iter_mut().enumerate() {
            pair.1 = Excluded(self.end[dim]);
        }

        BBox::from(bounds)
    }
}

impl<N: Copy + Scalar, const D: usize> BBoxBounded<N, D> for Range<Point<N, D>> {
    fn bbox(&self) -> BBox<N, D> {
        let mut bounds = [(Unbounded, Unbounded); D];

        for (dim, pair) in bounds.iter_mut().enumerate() {
            pair.0 = Included(self.start[dim]);
            pair.1 = Excluded(self.end[dim]);
        }

        BBox::from(bounds)
    }
}

impl<N: Copy + Scalar, const D: usize> BBoxBounded<N, D> for RangeInclusive<Point<N, D>> {
    fn bbox(&self) -> BBox<N, D> {
        let mut bounds = [(Unbounded, Unbounded); D];

        for (dim, pair) in bounds.iter_mut().enumerate() {
            pair.0 = Included(self.start()[dim]);
            pair.1 = Included(self.end()[dim]);
        }

        BBox::from(bounds)
    }
}

impl<N: Copy + Scalar, const D: usize> BBoxBounded<N, D> for RangeToInclusive<Point<N, D>> {
    fn bbox(&self) -> BBox<N, D> {
        let mut bounds = [(Unbounded, Unbounded); D];

        for (dim, pair) in bounds.iter_mut().enumerate() {
            pair.1 = Included(self.end[dim]);
        }

        BBox::from(bounds)
    }
}

impl<N: Copy + Scalar, const D: usize> BBoxBounded<N, D>
    for (Bound<Point<N, D>>, Bound<Point<N, D>>)
{
    fn bbox(&self) -> BBox<N, D> {
        let mut bounds = [(Unbounded, Unbounded); D];

        for (dim, pair) in bounds.iter_mut().enumerate() {
            pair.0 = match self.start_bound() {
                Included(pt) => Included(pt[dim]),
                Excluded(pt) => Excluded(pt[dim]),
                Unbounded => Unbounded,
            };

            pair.1 = match self.end_bound() {
                Included(pt) => Included(pt[dim]),
                Excluded(pt) => Excluded(pt[dim]),
                Unbounded => Unbounded,
            };
        }

        BBox::from(bounds)
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use na::{point, Point};

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
