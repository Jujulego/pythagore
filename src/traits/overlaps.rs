use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};
use std::ops::Bound::{self, Excluded, Included, Unbounded};

/// Tests if ranges overlaps
pub trait Overlaps<Lhs = Self> {
    fn overlap(&self, lhs: &Lhs) -> bool;
}

// Implementations for Range
impl<T: PartialOrd> Overlaps for Range<T> {
    #[inline]
    fn overlap(&self, lhs: &Range<T>) -> bool {
        self.start < lhs.end && self.end > lhs.start
    }
}

impl<T: PartialOrd> Overlaps<RangeFrom<T>> for Range<T> {
    #[inline]
    fn overlap(&self, lhs: &RangeFrom<T>) -> bool {
        self.end > lhs.start
    }
}

impl<T: PartialOrd> Overlaps<RangeFull> for Range<T> {
    #[inline]
    fn overlap(&self, _: &RangeFull) -> bool {
        true
    }
}

impl<T: PartialOrd> Overlaps<RangeInclusive<T>> for Range<T> {
    #[inline]
    fn overlap(&self, lhs: &RangeInclusive<T>) -> bool {
        &self.start <= lhs.end() && &self.end >= lhs.start()
    }
}

impl<T: PartialOrd> Overlaps<RangeTo<T>> for Range<T> {
    #[inline]
    fn overlap(&self, lhs: &RangeTo<T>) -> bool {
        self.start < lhs.end
    }
}

impl<T: PartialOrd> Overlaps<RangeToInclusive<T>> for Range<T> {
    #[inline]
    fn overlap(&self, lhs: &RangeToInclusive<T>) -> bool {
        self.start <= lhs.end
    }
}

impl<T: PartialOrd> Overlaps<(Bound<T>, Bound<T>)> for Range<T> {
    fn overlap(&self, lhs: &(Bound<T>, Bound<T>)) -> bool {
        (match &lhs.0 {
            Excluded(lhs_start) |
            Included(lhs_start) => &self.end > lhs_start,
            Unbounded => true
        }) && (match &lhs.1 {
            Excluded(lhs_end) => &self.start < lhs_end,
            Included(lhs_end) => &self.start <= lhs_end,
            Unbounded => true
        })
    }
}

// Implementations for RangeFrom
impl<T: PartialOrd> Overlaps<Range<T>> for RangeFrom<T> {
    #[inline]
    fn overlap(&self, lhs: &Range<T>) -> bool {
        self.start < lhs.end
    }
}

impl<T: PartialOrd> Overlaps for RangeFrom<T> {
    #[inline]
    fn overlap(&self, _: &RangeFrom<T>) -> bool {
        true
    }
}

impl<T: PartialOrd> Overlaps<RangeFull> for RangeFrom<T> {
    #[inline]
    fn overlap(&self, _: &RangeFull) -> bool {
        true
    }
}

impl<T: PartialOrd> Overlaps<RangeInclusive<T>> for RangeFrom<T> {
    #[inline]
    fn overlap(&self, lhs: &RangeInclusive<T>) -> bool {
        &self.start <= lhs.end()
    }
}

impl<T: PartialOrd> Overlaps<RangeTo<T>> for RangeFrom<T> {
    #[inline]
    fn overlap(&self, lhs: &RangeTo<T>) -> bool {
        self.start < lhs.end
    }
}

impl<T: PartialOrd> Overlaps<RangeToInclusive<T>> for RangeFrom<T> {
    #[inline]
    fn overlap(&self, lhs: &RangeToInclusive<T>) -> bool {
        self.start <= lhs.end
    }
}

impl<T: PartialOrd> Overlaps<(Bound<T>, Bound<T>)> for RangeFrom<T> {
    #[inline]
    fn overlap(&self, lhs: &(Bound<T>, Bound<T>)) -> bool {
        match &lhs.1 {
            Excluded(lhs_end) => &self.start < lhs_end,
            Included(lhs_end) => &self.start <= lhs_end,
            Unbounded => true
        }
    }
}

// Implementation for RangeFull
impl Overlaps for RangeFull {
    #[inline]
    fn overlap(&self, _: &RangeFull) -> bool {
        true
    }
}

// Implementations for RangeInclusive
impl<T: PartialOrd> Overlaps<Range<T>> for RangeInclusive<T> {
    #[inline]
    fn overlap(&self, lhs: &Range<T>) -> bool {
        self.start() < &lhs.end && self.end() >= &lhs.start
    }
}

impl<T: PartialOrd> Overlaps<RangeFrom<T>> for RangeInclusive<T> {
    #[inline]
    fn overlap(&self, lhs: &RangeFrom<T>) -> bool {
        self.end() >= &lhs.start
    }
}

impl<T: PartialOrd> Overlaps<RangeFull> for RangeInclusive<T> {
    #[inline]
    fn overlap(&self, _: &RangeFull) -> bool {
        true
    }
}

impl<T: PartialOrd> Overlaps for RangeInclusive<T> {
    #[inline]
    fn overlap(&self, lhs: &RangeInclusive<T>) -> bool {
        self.start() <= lhs.end() && self.end() >= lhs.start()
    }
}

impl<T: PartialOrd> Overlaps<RangeTo<T>> for RangeInclusive<T> {
    #[inline]
    fn overlap(&self, lhs: &RangeTo<T>) -> bool {
        self.start() < &lhs.end
    }
}

impl<T: PartialOrd> Overlaps<RangeToInclusive<T>> for RangeInclusive<T> {
    #[inline]
    fn overlap(&self, lhs: &RangeToInclusive<T>) -> bool {
        self.start() <= &lhs.end
    }
}

impl<T: PartialOrd> Overlaps<(Bound<T>, Bound<T>)> for RangeInclusive<T> {
    fn overlap(&self, lhs: &(Bound<T>, Bound<T>)) -> bool {
        (match &lhs.0 {
            Excluded(lhs_start) => self.end() > lhs_start,
            Included(lhs_start) => self.end() >= lhs_start,
            Unbounded => true
        }) && (match &lhs.1 {
            Excluded(lhs_end) => self.start() < lhs_end,
            Included(lhs_end) => self.start() <= lhs_end,
            Unbounded => true
        })
    }
}

// Implementations for RangeTo
impl<T: PartialOrd> Overlaps<Range<T>> for RangeTo<T> {
    #[inline]
    fn overlap(&self, lhs: &Range<T>) -> bool {
        self.end > lhs.start
    }
}

impl<T: PartialOrd> Overlaps<RangeFrom<T>> for RangeTo<T> {
    #[inline]
    fn overlap(&self, lhs: &RangeFrom<T>) -> bool {
        self.end > lhs.start
    }
}

impl<T: PartialOrd> Overlaps<RangeFull> for RangeTo<T> {
    #[inline]
    fn overlap(&self, _: &RangeFull) -> bool {
        true
    }
}

impl<T: PartialOrd> Overlaps<RangeInclusive<T>> for RangeTo<T> {
    #[inline]
    fn overlap(&self, lhs: &RangeInclusive<T>) -> bool {
        &self.end >= lhs.start()
    }
}

impl<T: PartialOrd> Overlaps for RangeTo<T> {
    #[inline]
    fn overlap(&self, _: &RangeTo<T>) -> bool {
        true
    }
}

impl<T: PartialOrd> Overlaps<RangeToInclusive<T>> for RangeTo<T> {
    #[inline]
    fn overlap(&self, _: &RangeToInclusive<T>) -> bool {
        true
    }
}

impl<T: PartialOrd> Overlaps<(Bound<T>, Bound<T>)> for RangeTo<T> {
    #[inline]
    fn overlap(&self, lhs: &(Bound<T>, Bound<T>)) -> bool {
        match &lhs.0 {
            Excluded(lhs_start) |
            Included(lhs_start) => &self.end > lhs_start,
            Unbounded => true
        }
    }
}

// Implementations for RangeToInclusive
impl<T: PartialOrd> Overlaps<Range<T>> for RangeToInclusive<T> {
    #[inline]
    fn overlap(&self, lhs: &Range<T>) -> bool {
        self.end >= lhs.start
    }
}

impl<T: PartialOrd> Overlaps<RangeFrom<T>> for RangeToInclusive<T> {
    #[inline]
    fn overlap(&self, lhs: &RangeFrom<T>) -> bool {
        self.end >= lhs.start
    }
}

impl<T: PartialOrd> Overlaps<RangeFull> for RangeToInclusive<T> {
    #[inline]
    fn overlap(&self, _: &RangeFull) -> bool {
        true
    }
}

impl<T: PartialOrd> Overlaps<RangeInclusive<T>> for RangeToInclusive<T> {
    #[inline]
    fn overlap(&self, lhs: &RangeInclusive<T>) -> bool {
        &self.end >= lhs.start()
    }
}

impl<T: PartialOrd> Overlaps<RangeTo<T>> for RangeToInclusive<T> {
    #[inline]
    fn overlap(&self, _: &RangeTo<T>) -> bool {
        true
    }
}

impl<T: PartialOrd> Overlaps for RangeToInclusive<T> {
    #[inline]
    fn overlap(&self, _: &RangeToInclusive<T>) -> bool {
        true
    }
}

impl<T: PartialOrd> Overlaps<(Bound<T>, Bound<T>)> for RangeToInclusive<T> {
    #[inline]
    fn overlap(&self, lhs: &(Bound<T>, Bound<T>)) -> bool {
        match &lhs.0 {
            Excluded(lhs_start) => &self.end > lhs_start,
            Included(lhs_start) => &self.end >= lhs_start,
            Unbounded => true
        }
    }
}

// Implementations for Bound tuple
impl<T: PartialOrd> Overlaps<Range<T>> for (Bound<T>, Bound<T>) {
    #[inline]
    fn overlap(&self, lhs: &Range<T>) -> bool {
        lhs.overlap(self)
    }
}

impl<T: PartialOrd> Overlaps<RangeFrom<T>> for (Bound<T>, Bound<T>) {
    #[inline]
    fn overlap(&self, lhs: &RangeFrom<T>) -> bool {
        lhs.overlap(self)
    }
}

impl<T: PartialOrd> Overlaps<RangeFull> for (Bound<T>, Bound<T>) {
    #[inline]
    fn overlap(&self, _: &RangeFull) -> bool {
        true
    }
}

impl<T: PartialOrd> Overlaps<RangeInclusive<T>> for (Bound<T>, Bound<T>) {
    #[inline]
    fn overlap(&self, lhs: &RangeInclusive<T>) -> bool {
        lhs.overlap(self)
    }
}

impl<T: PartialOrd> Overlaps<RangeTo<T>> for (Bound<T>, Bound<T>) {
    #[inline]
    fn overlap(&self, lhs: &RangeTo<T>) -> bool {
        lhs.overlap(self)
    }
}

impl<T: PartialOrd> Overlaps<RangeToInclusive<T>> for (Bound<T>, Bound<T>) {
    #[inline]
    fn overlap(&self, lhs: &RangeToInclusive<T>) -> bool {
        lhs.overlap(self)
    }
}

impl<T: PartialOrd> Overlaps for (Bound<T>, Bound<T>) {
    fn overlap(&self, lhs: &(Bound<T>, Bound<T>)) -> bool {
        (match (&self.1, &lhs.0) {
            (Excluded(rhs_end), Excluded(lhs_start)) |
            (Included(rhs_end), Excluded(lhs_start)) |
            (Excluded(rhs_end), Included(lhs_start)) => rhs_end > lhs_start,
            (Included(rhs_end), Included(lhs_start)) => rhs_end >= lhs_start,
            (Unbounded, _) | (_, Unbounded) => true,
        }) && (match (&self.0, &lhs.1) {
            (Excluded(rhs_start), Excluded(lhs_end)) |
            (Included(rhs_start), Excluded(lhs_end)) |
            (Excluded(rhs_start), Included(lhs_end)) => rhs_start < lhs_end,
            (Included(rhs_start), Included(lhs_end)) => rhs_start <= lhs_end,
            (Unbounded, _) | (_, Unbounded) => true,
        })
    }
}
