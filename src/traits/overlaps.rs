use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};
use std::ops::Bound::{self, Excluded, Included, Unbounded};

/// Tests if ranges overlaps
pub trait Overlaps<Rhs = Self> {
    fn overlaps(&self, rhs: &Rhs) -> bool;
}

// Implementations for Range
impl<T: PartialOrd> Overlaps for Range<T> {
    #[inline]
    fn overlaps(&self, rhs: &Range<T>) -> bool {
        self.start < rhs.end && self.end > rhs.start
    }
}

impl<T: PartialOrd> Overlaps<RangeFrom<T>> for Range<T> {
    #[inline]
    fn overlaps(&self, rhs: &RangeFrom<T>) -> bool {
        self.end > rhs.start
    }
}

impl<T> Overlaps<RangeFull> for Range<T> {
    #[inline]
    fn overlaps(&self, _: &RangeFull) -> bool {
        true
    }
}

impl<T: PartialOrd> Overlaps<RangeInclusive<T>> for Range<T> {
    #[inline]
    fn overlaps(&self, rhs: &RangeInclusive<T>) -> bool {
        &self.start <= rhs.end() && &self.end >= rhs.start()
    }
}

impl<T: PartialOrd> Overlaps<RangeTo<T>> for Range<T> {
    #[inline]
    fn overlaps(&self, rhs: &RangeTo<T>) -> bool {
        self.start < rhs.end
    }
}

impl<T: PartialOrd> Overlaps<RangeToInclusive<T>> for Range<T> {
    #[inline]
    fn overlaps(&self, rhs: &RangeToInclusive<T>) -> bool {
        self.start <= rhs.end
    }
}

impl<T: PartialOrd> Overlaps<(Bound<T>, Bound<T>)> for Range<T> {
    fn overlaps(&self, rhs: &(Bound<T>, Bound<T>)) -> bool {
        (match &rhs.0 {
            Excluded(lhs_start) |
            Included(lhs_start) => &self.end > lhs_start,
            Unbounded => true
        }) && (match &rhs.1 {
            Excluded(lhs_end) => &self.start < lhs_end,
            Included(lhs_end) => &self.start <= lhs_end,
            Unbounded => true
        })
    }
}

// Implementations for RangeFrom
impl<T: PartialOrd> Overlaps<Range<T>> for RangeFrom<T> {
    #[inline]
    fn overlaps(&self, rhs: &Range<T>) -> bool {
        self.start < rhs.end
    }
}

impl<T> Overlaps for RangeFrom<T> {
    #[inline]
    fn overlaps(&self, _: &RangeFrom<T>) -> bool {
        true
    }
}

impl<T> Overlaps<RangeFull> for RangeFrom<T> {
    #[inline]
    fn overlaps(&self, _: &RangeFull) -> bool {
        true
    }
}

impl<T: PartialOrd> Overlaps<RangeInclusive<T>> for RangeFrom<T> {
    #[inline]
    fn overlaps(&self, rhs: &RangeInclusive<T>) -> bool {
        &self.start <= rhs.end()
    }
}

impl<T: PartialOrd> Overlaps<RangeTo<T>> for RangeFrom<T> {
    #[inline]
    fn overlaps(&self, rhs: &RangeTo<T>) -> bool {
        self.start < rhs.end
    }
}

impl<T: PartialOrd> Overlaps<RangeToInclusive<T>> for RangeFrom<T> {
    #[inline]
    fn overlaps(&self, rhs: &RangeToInclusive<T>) -> bool {
        self.start <= rhs.end
    }
}

impl<T: PartialOrd> Overlaps<(Bound<T>, Bound<T>)> for RangeFrom<T> {
    #[inline]
    fn overlaps(&self, rhs: &(Bound<T>, Bound<T>)) -> bool {
        match &rhs.1 {
            Excluded(lhs_end) => &self.start < lhs_end,
            Included(lhs_end) => &self.start <= lhs_end,
            Unbounded => true
        }
    }
}

// Implementation for RangeFull
impl<T> Overlaps<Range<T>> for RangeFull {
    #[inline]
    fn overlaps(&self, _: &Range<T>) -> bool {
        true
    }
}

impl<T> Overlaps<RangeFrom<T>> for RangeFull {
    #[inline]
    fn overlaps(&self, _: &RangeFrom<T>) -> bool {
        true
    }
}

impl Overlaps for RangeFull {
    #[inline]
    fn overlaps(&self, _: &RangeFull) -> bool {
        true
    }
}

impl<T> Overlaps<RangeInclusive<T>> for RangeFull {
    #[inline]
    fn overlaps(&self, _: &RangeInclusive<T>) -> bool {
        true
    }
}

impl<T> Overlaps<RangeTo<T>> for RangeFull {
    #[inline]
    fn overlaps(&self, _: &RangeTo<T>) -> bool {
        true
    }
}

impl<T> Overlaps<RangeToInclusive<T>> for RangeFull {
    #[inline]
    fn overlaps(&self, _: &RangeToInclusive<T>) -> bool {
        true
    }
}

impl<T> Overlaps<(Bound<T>, Bound<T>)> for RangeFull {
    #[inline]
    fn overlaps(&self, _: &(Bound<T>, Bound<T>)) -> bool {
        true
    }
}

// Implementations for RangeInclusive
impl<T: PartialOrd> Overlaps<Range<T>> for RangeInclusive<T> {
    #[inline]
    fn overlaps(&self, rhs: &Range<T>) -> bool {
        self.start() < &rhs.end && self.end() >= &rhs.start
    }
}

impl<T: PartialOrd> Overlaps<RangeFrom<T>> for RangeInclusive<T> {
    #[inline]
    fn overlaps(&self, rhs: &RangeFrom<T>) -> bool {
        self.end() >= &rhs.start
    }
}

impl<T> Overlaps<RangeFull> for RangeInclusive<T> {
    #[inline]
    fn overlaps(&self, _: &RangeFull) -> bool {
        true
    }
}

impl<T: PartialOrd> Overlaps for RangeInclusive<T> {
    #[inline]
    fn overlaps(&self, rhs: &RangeInclusive<T>) -> bool {
        self.start() <= rhs.end() && self.end() >= rhs.start()
    }
}

impl<T: PartialOrd> Overlaps<RangeTo<T>> for RangeInclusive<T> {
    #[inline]
    fn overlaps(&self, rhs: &RangeTo<T>) -> bool {
        self.start() < &rhs.end
    }
}

impl<T: PartialOrd> Overlaps<RangeToInclusive<T>> for RangeInclusive<T> {
    #[inline]
    fn overlaps(&self, rhs: &RangeToInclusive<T>) -> bool {
        self.start() <= &rhs.end
    }
}

impl<T: PartialOrd> Overlaps<(Bound<T>, Bound<T>)> for RangeInclusive<T> {
    fn overlaps(&self, rhs: &(Bound<T>, Bound<T>)) -> bool {
        (match &rhs.0 {
            Excluded(lhs_start) => self.end() > lhs_start,
            Included(lhs_start) => self.end() >= lhs_start,
            Unbounded => true
        }) && (match &rhs.1 {
            Excluded(lhs_end) => self.start() < lhs_end,
            Included(lhs_end) => self.start() <= lhs_end,
            Unbounded => true
        })
    }
}

// Implementations for RangeTo
impl<T: PartialOrd> Overlaps<Range<T>> for RangeTo<T> {
    #[inline]
    fn overlaps(&self, rhs: &Range<T>) -> bool {
        self.end > rhs.start
    }
}

impl<T: PartialOrd> Overlaps<RangeFrom<T>> for RangeTo<T> {
    #[inline]
    fn overlaps(&self, rhs: &RangeFrom<T>) -> bool {
        self.end > rhs.start
    }
}

impl<T> Overlaps<RangeFull> for RangeTo<T> {
    #[inline]
    fn overlaps(&self, _: &RangeFull) -> bool {
        true
    }
}

impl<T: PartialOrd> Overlaps<RangeInclusive<T>> for RangeTo<T> {
    #[inline]
    fn overlaps(&self, rhs: &RangeInclusive<T>) -> bool {
        &self.end >= rhs.start()
    }
}

impl<T> Overlaps for RangeTo<T> {
    #[inline]
    fn overlaps(&self, _: &RangeTo<T>) -> bool {
        true
    }
}

impl<T> Overlaps<RangeToInclusive<T>> for RangeTo<T> {
    #[inline]
    fn overlaps(&self, _: &RangeToInclusive<T>) -> bool {
        true
    }
}

impl<T: PartialOrd> Overlaps<(Bound<T>, Bound<T>)> for RangeTo<T> {
    #[inline]
    fn overlaps(&self, rhs: &(Bound<T>, Bound<T>)) -> bool {
        match &rhs.0 {
            Excluded(lhs_start) |
            Included(lhs_start) => &self.end > lhs_start,
            Unbounded => true
        }
    }
}

// Implementations for RangeToInclusive
impl<T: PartialOrd> Overlaps<Range<T>> for RangeToInclusive<T> {
    #[inline]
    fn overlaps(&self, rhs: &Range<T>) -> bool {
        self.end >= rhs.start
    }
}

impl<T: PartialOrd> Overlaps<RangeFrom<T>> for RangeToInclusive<T> {
    #[inline]
    fn overlaps(&self, rhs: &RangeFrom<T>) -> bool {
        self.end >= rhs.start
    }
}

impl<T> Overlaps<RangeFull> for RangeToInclusive<T> {
    #[inline]
    fn overlaps(&self, _: &RangeFull) -> bool {
        true
    }
}

impl<T: PartialOrd> Overlaps<RangeInclusive<T>> for RangeToInclusive<T> {
    #[inline]
    fn overlaps(&self, rhs: &RangeInclusive<T>) -> bool {
        &self.end >= rhs.start()
    }
}

impl<T> Overlaps<RangeTo<T>> for RangeToInclusive<T> {
    #[inline]
    fn overlaps(&self, _: &RangeTo<T>) -> bool {
        true
    }
}

impl<T> Overlaps for RangeToInclusive<T> {
    #[inline]
    fn overlaps(&self, _: &RangeToInclusive<T>) -> bool {
        true
    }
}

impl<T: PartialOrd> Overlaps<(Bound<T>, Bound<T>)> for RangeToInclusive<T> {
    #[inline]
    fn overlaps(&self, rhs: &(Bound<T>, Bound<T>)) -> bool {
        match &rhs.0 {
            Excluded(lhs_start) => &self.end > lhs_start,
            Included(lhs_start) => &self.end >= lhs_start,
            Unbounded => true
        }
    }
}

// Implementations for Bound tuple
impl<T: PartialOrd> Overlaps<Range<T>> for (Bound<T>, Bound<T>) {
    #[inline]
    fn overlaps(&self, rhs: &Range<T>) -> bool {
        rhs.overlaps(self)
    }
}

impl<T: PartialOrd> Overlaps<RangeFrom<T>> for (Bound<T>, Bound<T>) {
    #[inline]
    fn overlaps(&self, rhs: &RangeFrom<T>) -> bool {
        rhs.overlaps(self)
    }
}

impl<T> Overlaps<RangeFull> for (Bound<T>, Bound<T>) {
    #[inline]
    fn overlaps(&self, _: &RangeFull) -> bool {
        true
    }
}

impl<T: PartialOrd> Overlaps<RangeInclusive<T>> for (Bound<T>, Bound<T>) {
    #[inline]
    fn overlaps(&self, rhs: &RangeInclusive<T>) -> bool {
        rhs.overlaps(self)
    }
}

impl<T: PartialOrd> Overlaps<RangeTo<T>> for (Bound<T>, Bound<T>) {
    #[inline]
    fn overlaps(&self, rhs: &RangeTo<T>) -> bool {
        rhs.overlaps(self)
    }
}

impl<T: PartialOrd> Overlaps<RangeToInclusive<T>> for (Bound<T>, Bound<T>) {
    #[inline]
    fn overlaps(&self, rhs: &RangeToInclusive<T>) -> bool {
        rhs.overlaps(self)
    }
}

impl<T: PartialOrd> Overlaps for (Bound<T>, Bound<T>) {
    fn overlaps(&self, rhs: &(Bound<T>, Bound<T>)) -> bool {
        (match (&self.1, &rhs.0) {
            (Excluded(rhs_end), Excluded(lhs_start)) |
            (Included(rhs_end), Excluded(lhs_start)) |
            (Excluded(rhs_end), Included(lhs_start)) => rhs_end > lhs_start,
            (Included(rhs_end), Included(lhs_start)) => rhs_end >= lhs_start,
            (Unbounded, _) | (_, Unbounded) => true,
        }) && (match (&self.0, &rhs.1) {
            (Excluded(rhs_start), Excluded(lhs_end)) |
            (Included(rhs_start), Excluded(lhs_end)) |
            (Excluded(rhs_start), Included(lhs_end)) => rhs_start < lhs_end,
            (Included(rhs_start), Included(lhs_end)) => rhs_start <= lhs_end,
            (Unbounded, _) | (_, Unbounded) => true,
        })
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    mod range {
        use super::*;

        #[test]
        fn test_overlaps_range() {
            assert!(!(0..4).overlaps(&(-3..-1)));
            assert!( (0..4).overlaps(&(-1.. 1)));
            assert!( (0..4).overlaps(&( 1.. 3)));
            assert!( (0..4).overlaps(&( 3.. 5)));
            assert!(!(0..4).overlaps(&( 5.. 7)));
        }

        #[test]
        fn test_overlaps_range_from() {
            assert!( (0..4).overlaps(&(-3..)));
            assert!( (0..4).overlaps(&( 2..)));
            assert!(!(0..4).overlaps(&( 5..)));
        }

        #[test]
        fn test_overlaps_range_full() {
            assert!( (0..4).overlaps(&(..)));
        }

        #[test]
        fn test_overlaps_range_inclusive() {
            assert!(!(0..4).overlaps(&(-3..=-1)));
            assert!( (0..4).overlaps(&(-3..= 0)));
            assert!( (0..4).overlaps(&(-1..= 1)));
            assert!( (0..4).overlaps(&( 1..= 3)));
            assert!( (0..4).overlaps(&( 3..= 5)));
            assert!(!(0..4).overlaps(&( 5..= 7)));
        }

        #[test]
        fn test_overlaps_range_to() {
            assert!(!(0..4).overlaps(&(..-3)));
            assert!( (0..4).overlaps(&(.. 2)));
            assert!( (0..4).overlaps(&(.. 5)));
        }

        #[test]
        fn test_overlaps_range_to_inclusive() {
            assert!(!(0..4).overlaps(&(..=-3)));
            assert!( (0..4).overlaps(&(..= 0)));
            assert!( (0..4).overlaps(&(..= 2)));
            assert!( (0..4).overlaps(&(..= 5)));
        }

        #[test]
        fn test_overlaps_bound_tuple() {
            assert!(!(0..4).overlaps(&(Included(-3), Included(-1))));
            assert!(!(0..4).overlaps(&(Included(-3), Excluded( 0))));
            assert!( (0..4).overlaps(&(Included(-3), Included( 0))));
            assert!( (0..4).overlaps(&(Included(-1), Included( 1))));
            assert!( (0..4).overlaps(&(Included( 1), Included( 3))));
            assert!( (0..4).overlaps(&(Included( 3), Included( 5))));
            assert!(!(0..4).overlaps(&(Included( 5), Included( 7))));
        }
    }

    mod range_from {
        use super::*;

        #[test]
        fn test_overlaps_range() {
            assert!(!(0..).overlaps(&(-3..-1)));
            assert!( (0..).overlaps(&(-1.. 1)));
            assert!( (0..).overlaps(&( 3.. 5)));
        }

        #[test]
        fn test_overlaps_range_from() {
            assert!( (0..).overlaps(&(-3..)));
            assert!( (0..).overlaps(&( 2..)));
        }

        #[test]
        fn test_overlaps_range_full() {
            assert!( (0..).overlaps(&(..)));
        }

        #[test]
        fn test_overlaps_range_inclusive() {
            assert!(!(0..).overlaps(&(-3..=-1)));
            assert!( (0..).overlaps(&(-3..= 0)));
            assert!( (0..).overlaps(&(-1..= 1)));
            assert!( (0..).overlaps(&( 3..= 5)));
        }

        #[test]
        fn test_overlaps_range_to() {
            assert!(!(0..).overlaps(&(..-3)));
            assert!( (0..).overlaps(&(.. 2)));
        }

        #[test]
        fn test_overlaps_range_to_inclusive() {
            assert!(!(0..).overlaps(&(..=-3)));
            assert!( (0..).overlaps(&(..= 0)));
            assert!( (0..).overlaps(&(..= 2)));
        }

        #[test]
        fn test_overlaps_bound_tuple() {
            assert!(!(0..).overlaps(&(Included(-3), Included(-1))));
            assert!(!(0..).overlaps(&(Included(-3), Excluded( 0))));
            assert!( (0..).overlaps(&(Included(-3), Included( 0))));
            assert!( (0..).overlaps(&(Included(-1), Included( 1))));
            assert!( (0..).overlaps(&(Included( 3), Included( 5))));
        }
    }

    mod range_full {
        use super::*;

        #[test]
        fn test_overlaps_range() {
            assert!((..).overlaps(&(1..3)));
        }

        #[test]
        fn test_overlaps_range_from() {
            assert!((..).overlaps(&(2..)));
        }

        #[test]
        fn test_overlaps_range_full() {
            assert!((..).overlaps(&(..)));
        }

        #[test]
        fn test_overlaps_range_inclusive() {
            assert!((..).overlaps(&(1..=3)));
        }

        #[test]
        fn test_overlaps_range_to() {
            assert!((..).overlaps(&(..2)));
        }

        #[test]
        fn test_overlaps_range_to_inclusive() {
            assert!((..).overlaps(&(..=2)));
        }

        #[test]
        fn test_overlaps_bound_tuple() {
            assert!((..).overlaps(&(Included(1), Included(3))));
        }
    }

    mod range_inclusive {
        use super::*;

        #[test]
        fn test_overlaps_range() {
            assert!(!(0..=4).overlaps(&(-3..-1)));
            assert!( (0..=4).overlaps(&(-1.. 1)));
            assert!( (0..=4).overlaps(&( 1.. 3)));
            assert!( (0..=4).overlaps(&( 3.. 5)));
            assert!( (0..=4).overlaps(&( 4.. 7)));
            assert!(!(0..=4).overlaps(&( 5.. 7)));
        }

        #[test]
        fn test_overlaps_range_from() {
            assert!( (0..=4).overlaps(&(-3..)));
            assert!( (0..=4).overlaps(&( 2..)));
            assert!( (0..=4).overlaps(&( 4..)));
            assert!(!(0..=4).overlaps(&( 5..)));
        }

        #[test]
        fn test_overlaps_range_full() {
            assert!( (0..=4).overlaps(&(..)));
        }

        #[test]
        fn test_overlaps_range_inclusive() {
            assert!(!(0..=4).overlaps(&(-3..=-1)));
            assert!( (0..=4).overlaps(&(-3..= 0)));
            assert!( (0..=4).overlaps(&(-1..= 1)));
            assert!( (0..=4).overlaps(&( 1..= 3)));
            assert!( (0..=4).overlaps(&( 3..= 5)));
            assert!( (0..=4).overlaps(&( 4..= 5)));
            assert!(!(0..=4).overlaps(&( 5..= 7)));
        }

        #[test]
        fn test_overlaps_range_to() {
            assert!(!(0..=4).overlaps(&(..-3)));
            assert!( (0..=4).overlaps(&(.. 2)));
            assert!( (0..=4).overlaps(&(.. 5)));
        }

        #[test]
        fn test_overlaps_range_to_inclusive() {
            assert!(!(0..=4).overlaps(&(..=-3)));
            assert!( (0..=4).overlaps(&(..= 0)));
            assert!( (0..=4).overlaps(&(..= 2)));
            assert!( (0..=4).overlaps(&(..= 5)));
        }

        #[test]
        fn test_overlaps_bound_tuple() {
            assert!(!(0..=4).overlaps(&(Included(-3), Included(-1))));
            assert!(!(0..=4).overlaps(&(Included(-3), Excluded( 0))));
            assert!( (0..=4).overlaps(&(Included(-3), Included( 0))));
            assert!( (0..=4).overlaps(&(Included(-1), Included( 1))));
            assert!( (0..=4).overlaps(&(Included( 1), Included( 3))));
            assert!( (0..=4).overlaps(&(Included( 3), Included( 5))));
            assert!( (0..=4).overlaps(&(Included( 4), Included( 5))));
            assert!(!(0..=4).overlaps(&(Excluded( 4), Included( 5))));
            assert!(!(0..=4).overlaps(&(Included( 5), Included( 7))));
        }
    }

    mod range_to {
        use super::*;

        #[test]
        fn test_overlaps_range() {
            assert!( (..4).overlaps(&( 1.. 3)));
            assert!( (..4).overlaps(&( 3.. 5)));
            assert!(!(..4).overlaps(&( 5.. 7)));
        }

        #[test]
        fn test_overlaps_range_from() {
            assert!( (..4).overlaps(&( 2..)));
            assert!(!(..4).overlaps(&( 5..)));
        }

        #[test]
        fn test_overlaps_range_full() {
            assert!( (..4).overlaps(&(..)));
        }

        #[test]
        fn test_overlaps_range_inclusive() {
            assert!( (..4).overlaps(&( 1..= 3)));
            assert!( (..4).overlaps(&( 3..= 5)));
            assert!(!(..4).overlaps(&( 5..= 7)));
        }

        #[test]
        fn test_overlaps_range_to() {
            assert!( (..4).overlaps(&(.. 2)));
            assert!( (..4).overlaps(&(.. 5)));
        }

        #[test]
        fn test_overlaps_range_to_inclusive() {
            assert!( (..4).overlaps(&(..= 2)));
            assert!( (..4).overlaps(&(..= 5)));
        }

        #[test]
        fn test_overlaps_bound_tuple() {
            assert!( (..4).overlaps(&(Included( 1), Included( 3))));
            assert!( (..4).overlaps(&(Included( 3), Included( 5))));
            assert!(!(..4).overlaps(&(Included( 5), Included( 7))));
        }
    }

    mod range_to_inclusive {
        use super::*;

        #[test]
        fn test_overlaps_range() {
            assert!( (..=4).overlaps(&( 1.. 3)));
            assert!( (..=4).overlaps(&( 3.. 5)));
            assert!( (..=4).overlaps(&( 4.. 5)));
            assert!(!(..=4).overlaps(&( 5.. 7)));
        }

        #[test]
        fn test_overlaps_range_from() {
            assert!( (..=4).overlaps(&( 2..)));
            assert!( (..=4).overlaps(&( 4..)));
            assert!(!(..=4).overlaps(&( 5..)));
        }

        #[test]
        fn test_overlaps_range_full() {
            assert!( (..=4).overlaps(&(..)));
        }

        #[test]
        fn test_overlaps_range_inclusive() {
            assert!( (..=4).overlaps(&( 1..= 3)));
            assert!( (..=4).overlaps(&( 3..= 5)));
            assert!( (..=4).overlaps(&( 4..= 5)));
            assert!(!(..=4).overlaps(&( 5..= 7)));
        }

        #[test]
        fn test_overlaps_range_to() {
            assert!( (..=4).overlaps(&(.. 2)));
            assert!( (..=4).overlaps(&(.. 5)));
        }

        #[test]
        fn test_overlaps_range_to_inclusive() {
            assert!( (..=4).overlaps(&(..= 2)));
            assert!( (..=4).overlaps(&(..= 5)));
        }

        #[test]
        fn test_overlaps_bound_tuple() {
            assert!( (..=4).overlaps(&(Included( 1), Included( 3))));
            assert!( (..=4).overlaps(&(Included( 3), Included( 5))));
            assert!( (..=4).overlaps(&(Included( 4), Included( 5))));
            assert!(!(..=4).overlaps(&(Excluded( 4), Included( 5))));
            assert!(!(..=4).overlaps(&(Included( 5), Included( 7))));
        }
    }
}