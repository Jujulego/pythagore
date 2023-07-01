use super::BBox;
use crate::bbox::BoundingBox;
use na::{Point, Scalar};
use num_traits::{Bounded, Zero};
use std::ops::Bound::{self, *};
use std::ops::{
    Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};

// Implementations
impl<N: Scalar, const D: usize> BoundingBox<N, D> for RangeFull {
    fn get_start(&self, _d: usize) -> Bound<N> {
        Unbounded
    }

    fn get_end(&self, _d: usize) -> Bound<N> {
        Unbounded
    }

    /// Always true
    fn holds(&self, _: &Point<N, D>) -> bool {
        true
    }

    /// Returns the other
    fn intersection(&self, other: &Self) -> BBox<N, D>
    where
        N: Copy,
    {
        BBox::from_bounding_box(other)
    }

    fn start_point(&self) -> Point<N, D>
    where
        N: Bounded,
    {
        Point::min_value()
    }

    fn end_point(&self) -> Point<N, D>
    where
        N: Bounded,
    {
        Point::max_value()
    }

    fn center_point(&self) -> Point<N, D>
    where
        N: Zero,
    {
        Point::origin()
    }
}

impl<N: Copy + Scalar, const D: usize> BoundingBox<N, D> for RangeFrom<Point<N, D>> {
    fn get_start(&self, d: usize) -> Bound<N> {
        Included(self.start[d])
    }

    fn get_end(&self, _d: usize) -> Bound<N> {
        Unbounded
    }

    fn holds(&self, pt: &Point<N, D>) -> bool where N: PartialOrd {
        self.contains(pt)
    }

    fn start_point(&self) -> Point<N, D> {
        self.start
    }

    fn end_point(&self) -> Point<N, D>
    where
        N: Bounded,
    {
        Point::max_value()
    }
}

impl<N: Copy + Scalar, const D: usize> BoundingBox<N, D> for RangeTo<Point<N, D>> {
    fn get_start(&self, _d: usize) -> Bound<N> {
        Unbounded
    }

    fn get_end(&self, d: usize) -> Bound<N> {
        Excluded(self.end[d])
    }

    fn holds(&self, pt: &Point<N, D>) -> bool where N: PartialOrd {
        self.contains(pt)
    }

    fn start_point(&self) -> Point<N, D>
    where
        N: Bounded,
    {
        Point::min_value()
    }

    fn end_point(&self) -> Point<N, D> {
        self.end
    }
}

impl<N: Copy + Scalar, const D: usize> BoundingBox<N, D> for RangeToInclusive<Point<N, D>> {
    fn get_start(&self, _d: usize) -> Bound<N> {
        Unbounded
    }

    fn get_end(&self, d: usize) -> Bound<N> {
        Included(self.end[d])
    }

    fn holds(&self, pt: &Point<N, D>) -> bool where N: PartialOrd {
        self.contains(pt)
    }

    fn start_point(&self) -> Point<N, D>
    where
        N: Bounded,
    {
        Point::min_value()
    }

    fn end_point(&self) -> Point<N, D> {
        self.end
    }
}

impl<N: Copy + Scalar, const D: usize> BoundingBox<N, D> for Range<Point<N, D>> {
    fn get_start(&self, d: usize) -> Bound<N> {
        Included(self.start[d])
    }

    fn get_end(&self, d: usize) -> Bound<N> {
        Excluded(self.end[d])
    }

    fn holds(&self, pt: &Point<N, D>) -> bool where N: PartialOrd {
        self.contains(pt)
    }

    fn start_point(&self) -> Point<N, D> {
        self.start
    }

    fn end_point(&self) -> Point<N, D> {
        self.end
    }
}

impl<N: Copy + Scalar, const D: usize> BoundingBox<N, D> for RangeInclusive<Point<N, D>> {
    fn get_start(&self, d: usize) -> Bound<N> {
        Included(self.start()[d])
    }

    fn get_end(&self, d: usize) -> Bound<N> {
        Excluded(self.end()[d])
    }

    fn holds(&self, pt: &Point<N, D>) -> bool where N: PartialOrd {
        self.contains(pt)
    }

    fn start_point(&self) -> Point<N, D> {
        *self.start()
    }

    fn end_point(&self) -> Point<N, D> {
        *self.end()
    }
}

impl<N: Copy + Scalar, const D: usize> BoundingBox<N, D> for (Bound<Point<N, D>>, Bound<Point<N, D>>) {
    fn get_start(&self, d: usize) -> Bound<N> {
        match self.start_bound() {
            Included(pt) => Included(pt[d]),
            Excluded(pt) => Excluded(pt[d]),
            Unbounded => Unbounded,
        }
    }

    fn get_end(&self, d: usize) -> Bound<N> {
        match self.end_bound() {
            Included(pt) => Included(pt[d]),
            Excluded(pt) => Excluded(pt[d]),
            Unbounded => Unbounded,
        }
    }

    fn holds(&self, pt: &Point<N, D>) -> bool where N: PartialOrd {
        self.contains(pt)
    }

    fn start_point(&self) -> Point<N, D>
    where
        N: Bounded,
    {
        match self.start_bound() {
            Included(pt) | Excluded(pt) => *pt,
            Unbounded => Point::min_value(),
        }
    }

    fn end_point(&self) -> Point<N, D>
    where
        N: Bounded,
    {
        match self.start_bound() {
            Included(pt) | Excluded(pt) => *pt,
            Unbounded => Point::max_value(),
        }
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use na::{point, Point};

    #[test]
    fn range_full_box_holds() {
        assert!((..).holds(&point![1, 1]));
    }

    #[test]
    fn range_from_box_holds() {
        let range = Point::origin()..;

        assert!(range.holds(&point![1, 1]));
        assert!(range.holds(&Point::origin()));

        assert!(!range.holds(&point![1, -1]));
        assert!(!range.holds(&point![-1, 1]));
        assert!(!range.holds(&point![-1, -1]));
    }

    #[test]
    fn range_to_box_holds() {
        let range = ..Point::origin();

        assert!(range.holds(&point![-1, -1]));

        assert!(!range.holds(&Point::origin()));
        assert!(!range.holds(&point![-1, 1]));
        assert!(!range.holds(&point![1, -1]));
        assert!(!range.holds(&point![1, 1]));
    }

    #[test]
    fn range_box_holds() {
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
    fn range_inclusive_box_holds() {
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
    fn range_to_inclusive_box_holds() {
        let range = ..=point![5, 5];

        assert!(range.holds(&point![-1, -1]));
        assert!(range.holds(&point![5, 5]));

        assert!(!range.holds(&point![1, 6]));
        assert!(!range.holds(&point![6, 1]));
        assert!(!range.holds(&point![6, 6]));
    }
}
