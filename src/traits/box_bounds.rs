use std::ops::Bound::*;
use std::ops::{Bound, Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};
use num_traits::Num;
use crate::Scalar;
use crate::traits::{Dimension, BoxableScalar};

/// Implemented by range types to define bounding box using range syntax
///
/// ## Example
/// ```
/// use pythagore::{point, Point, scalar};
/// use pythagore::traits::BoxBounds;
///
/// let bbox = Point::origin()..point!{ x: 5, y: 5 };
///
/// assert!(bbox.box_contains(&point!{ x: 2, y: 2 }));
/// ```
pub trait BoxBounds<N, T, const D: usize>: RangeBounds<T>
where
    N: Num + PartialOrd,
    T: BoxableScalar<N, Output = N> + Dimension<D>
{
    fn box_contains<U>(&self, item: &U) -> bool
    where
        U: BoxableScalar<N, Output = N> + Dimension<D>
    {
        (0..D).all(|idx| (
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
impl<N, const D: usize> BoxBounds<N, Scalar<N, D>, D> for RangeFull
where
    N: Copy + Num + PartialOrd,
{}

impl<N, T, const D: usize> BoxBounds<N, T, D> for RangeFrom<T>
where
    N: Num + PartialOrd,
    T: BoxableScalar<N, Output = N> + Dimension<D>
{}

impl<N, T, const D: usize> BoxBounds<N, T, D> for RangeTo<T>
where
    N: Num + PartialOrd,
    T: BoxableScalar<N, Output = N> + Dimension<D>
{}

impl<N, T, const D: usize> BoxBounds<N, T, D> for Range<T>
where
    N: Num + PartialOrd,
    T: BoxableScalar<N, Output = N> + Dimension<D>
{}

impl<N, T, const D: usize> BoxBounds<N, T, D> for RangeInclusive<T>
where
    N: Num + PartialOrd,
    T: BoxableScalar<N, Output = N> + Dimension<D>
{}

impl<N, T, const D: usize> BoxBounds<N, T, D> for RangeToInclusive<T>
where
    N: Num + PartialOrd,
    T: BoxableScalar<N, Output = N> + Dimension<D>
{}

impl<N, T, const D: usize> BoxBounds<N, T, D> for (Bound<T>, Bound<T>)
where
    N: Num + PartialOrd,
    T: BoxableScalar<N, Output = N> + Dimension<D>
{}

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