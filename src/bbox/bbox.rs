use std::ops::Bound::*;
use std::ops::{Bound, Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};
use num_traits::Num;
use crate::Point;

/// Implemented by range types to define bounding box using range syntax
///
/// ## Example
/// ```
/// use pythagore::{BBox, point, Point};
///
/// let bbox = Point::origin()..point!{ x: 5, y: 5 };
///
/// assert!(bbox.box_contains(&point!{ x: 2, y: 2 }));
/// ```
pub trait BBox<N: Num + PartialOrd, const D: usize>: RangeBounds<Point<N, D>> {
    fn box_contains(&self, item: &Point<N, D>) -> bool {
        (0..D - 1).all(|idx| (
            match self.start_bound() {
                Included(start) => Included(&start[idx]),
                Excluded(start) => Excluded(&start[idx]),
                Unbounded => Unbounded,
            },
            match self.end_bound() {
                Included(end) => Included(&end[idx]),
                Excluded(end) => Excluded(&end[idx]),
                Unbounded => Unbounded,
            },
        ).contains(&item[idx]))
    }
}

// Implementations
impl<N: Num + PartialOrd, const D: usize> BBox<N, D> for RangeFull {}

impl<N: Num + PartialOrd, const D: usize> BBox<N, D> for RangeFrom<Point<N, D>> {}

impl<N: Num + PartialOrd, const D: usize> BBox<N, D> for RangeTo<Point<N, D>> {}

impl<N: Num + PartialOrd, const D: usize> BBox<N, D> for Range<Point<N, D>> {}

impl<N: Num + PartialOrd, const D: usize> BBox<N, D> for RangeInclusive<Point<N, D>> {}

impl<N: Num + PartialOrd, const D: usize> BBox<N, D> for RangeToInclusive<Point<N, D>> {}

impl<N: Num + PartialOrd, const D: usize> BBox<N, D> for (Bound<Point<N, D>>, Bound<Point<N, D>>) {}

// Tests
#[cfg(test)]
mod tests {
    use crate::{point, Point};
    use super::*;

    #[test]
    fn range_full_box_contains() {
        assert!((..).box_contains(&point!{ x: 1, y: 1 }));
    }

    #[test]
    fn range_from_box_contains() {
        let range = Point::origin()..;

        assert!(range.box_contains(&point!{ x: 1, y: 1 }));
        assert!(range.box_contains(&Point::origin()));

        assert!(!range.box_contains(&point!{ x: 1, y: -1 }));
        assert!(!range.box_contains(&point!{ x: -1, y: 1 }));
        assert!(!range.box_contains(&point!{ x: -1, y: -1 }));
    }

    #[test]
    fn range_to_box_contains() {
        let range = ..Point::origin();

        assert!(range.box_contains(&point!{ x: -1, y: -1 }));

        assert!(!range.box_contains(&Point::origin()));
        assert!(!range.box_contains(&point!{ x: -1, y: 1 }));
        assert!(!range.box_contains(&point!{ x: 1, y: -1 }));
        assert!(!range.box_contains(&point!{ x: 1, y: 1 }));
    }

    #[test]
    fn range_box_contains() {
        let range = Point::origin()..point!{ x: 5, y: 5 };

        assert!(range.box_contains(&point!{ x: 2, y: 2 }));
        assert!(range.box_contains(&Point::origin()));

        assert!(!range.box_contains(&point!{ x: 1, y: -1 }));
        assert!(!range.box_contains(&point!{ x: -1, y: 1 }));
        assert!(!range.box_contains(&point!{ x: -1, y: -1 }));

        assert!(!range.box_contains(&point!{ x: 1, y: 5 }));
        assert!(!range.box_contains(&point!{ x: 5, y: 1 }));
        assert!(!range.box_contains(&point!{ x: 5, y: 5 }));
    }

    #[test]
    fn range_inclusive_box_contains() {
        let range = Point::origin()..=point!{ x: 5, y: 5 };

        assert!(range.box_contains(&point!{ x: 2, y: 2 }));
        assert!(range.box_contains(&Point::origin()));
        assert!(range.box_contains(&point!{ x: 5, y: 5 }));

        assert!(!range.box_contains(&point!{ x: 1, y: -1 }));
        assert!(!range.box_contains(&point!{ x: -1, y: 1 }));
        assert!(!range.box_contains(&point!{ x: -1, y: -1 }));

        assert!(!range.box_contains(&point!{ x: 1, y: 6 }));
        assert!(!range.box_contains(&point!{ x: 6, y: 1 }));
        assert!(!range.box_contains(&point!{ x: 6, y: 6 }));
    }

    #[test]
    fn range_to_inclusive_box_contains() {
        let range = ..=point!{ x: 5, y: 5 };

        assert!(range.box_contains(&point!{ x: -1, y: -1 }));
        assert!(range.box_contains(&point!{ x: 5, y: 5 }));

        assert!(!range.box_contains(&point!{ x: 1, y: 6 }));
        assert!(!range.box_contains(&point!{ x: 6, y: 1 }));
        assert!(!range.box_contains(&point!{ x: 6, y: 6 }));
    }
}