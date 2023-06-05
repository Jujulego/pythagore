use std::ops::Bound::*;
use std::ops::{Bound, Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};
use num_traits::Num;

use crate::bbox::bbox_nd::BBox;
use crate::Point;
use crate::traits::BBoxBounded;

// Implementations
impl<N: Num, const D: usize> BBoxBounded<N, D> for RangeFull {
    fn bbox(&self) -> BBox<'_, N, D> {
        BBox::from([(Unbounded, Unbounded); D])
    }
}

impl<N: Num, const D: usize> BBoxBounded<N, D> for RangeFrom<Point<N, D>> {
    fn bbox(&self) -> BBox<'_, N, D> {
        self.start.iter()
            .map(|start| (Included(start), Unbounded))
            .collect()
    }
}

impl<N: Num, const D: usize> BBoxBounded<N, D> for RangeTo<Point<N, D>> {
    fn bbox(&self) -> BBox<'_, N, D> {
        self.end.iter()
            .map(|end| (Unbounded, Excluded(end)))
            .collect()
    }
}

impl<N: Num, const D: usize> BBoxBounded<N, D> for Range<Point<N, D>> {
    fn bbox(&self) -> BBox<'_, N, D> {
        self.start.iter()
            .zip(self.end.iter())
            .map(|(start, end)| (Included(start), Excluded(end)))
            .collect()
    }
}

impl<N: Num, const D: usize> BBoxBounded<N, D> for RangeInclusive<Point<N, D>> {
    fn bbox(&self) -> BBox<'_, N, D> {
        self.start().iter()
            .zip(self.end().iter())
            .map(|(start, end)| (Included(start), Included(end)))
            .collect()
    }
}

impl<N: Num, const D: usize> BBoxBounded<N, D> for RangeToInclusive<Point<N, D>> {
    fn bbox(&self) -> BBox<'_, N, D> {
        self.end.iter()
            .map(|end| (Unbounded, Included(end)))
            .collect()
    }
}

impl<N: Num, const D: usize> BBoxBounded<N, D> for (Bound<Point<N, D>>, Bound<Point<N, D>>) {
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
    use crate::{point, Point};
    use super::*;

    #[test]
    fn range_full_box_contains() {
        assert!((..).bbox().contains(&point!{ x: 1, y: 1 }));
    }

    #[test]
    fn range_from_box_contains() {
        let range = Point::origin()..;

        assert!(range.bbox().contains(&point!{ x: 1, y: 1 }));
        assert!(range.bbox().contains(&Point::origin()));

        assert!(!range.bbox().contains(&point!{ x: 1, y: -1 }));
        assert!(!range.bbox().contains(&point!{ x: -1, y: 1 }));
        assert!(!range.bbox().contains(&point!{ x: -1, y: -1 }));
    }

    #[test]
    fn range_to_box_contains() {
        let range = ..Point::origin();

        assert!(range.bbox().contains(&point!{ x: -1, y: -1 }));

        assert!(!range.bbox().contains(&Point::origin()));
        assert!(!range.bbox().contains(&point!{ x: -1, y: 1 }));
        assert!(!range.bbox().contains(&point!{ x: 1, y: -1 }));
        assert!(!range.bbox().contains(&point!{ x: 1, y: 1 }));
    }

    #[test]
    fn range_box_contains() {
        let range = Point::origin()..point!{ x: 5, y: 5 };

        assert!(range.bbox().contains(&point!{ x: 2, y: 2 }));
        assert!(range.bbox().contains(&Point::origin()));

        assert!(!range.bbox().contains(&point!{ x: 1, y: -1 }));
        assert!(!range.bbox().contains(&point!{ x: -1, y: 1 }));
        assert!(!range.bbox().contains(&point!{ x: -1, y: -1 }));

        assert!(!range.bbox().contains(&point!{ x: 1, y: 5 }));
        assert!(!range.bbox().contains(&point!{ x: 5, y: 1 }));
        assert!(!range.bbox().contains(&point!{ x: 5, y: 5 }));
    }

    #[test]
    fn range_inclusive_box_contains() {
        let range = Point::origin()..=point!{ x: 5, y: 5 };

        assert!(range.bbox().contains(&point!{ x: 2, y: 2 }));
        assert!(range.bbox().contains(&Point::origin()));
        assert!(range.bbox().contains(&point!{ x: 5, y: 5 }));

        assert!(!range.bbox().contains(&point!{ x: 1, y: -1 }));
        assert!(!range.bbox().contains(&point!{ x: -1, y: 1 }));
        assert!(!range.bbox().contains(&point!{ x: -1, y: -1 }));

        assert!(!range.bbox().contains(&point!{ x: 1, y: 6 }));
        assert!(!range.bbox().contains(&point!{ x: 6, y: 1 }));
        assert!(!range.bbox().contains(&point!{ x: 6, y: 6 }));
    }

    #[test]
    fn range_to_inclusive_box_contains() {
        let range = ..=point!{ x: 5, y: 5 };

        assert!(range.bbox().contains(&point!{ x: -1, y: -1 }));
        assert!(range.bbox().contains(&point!{ x: 5, y: 5 }));

        assert!(!range.bbox().contains(&point!{ x: 1, y: 6 }));
        assert!(!range.bbox().contains(&point!{ x: 6, y: 1 }));
        assert!(!range.bbox().contains(&point!{ x: 6, y: 6 }));
    }
}