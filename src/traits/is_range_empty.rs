use std::ops::Bound::{self, *};
use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

/// Tests to known if a range does not contain anything
pub trait IsRangeEmpty {
    /// Returns true if range is empty.
    fn is_range_empty(&self) -> bool;
}

// Implementations
/// Always return false for RangeFull
impl IsRangeEmpty for RangeFull {
    #[inline]
    fn is_range_empty(&self) -> bool {
        false
    }
}

/// Always return false for RangeFrom
impl<N> IsRangeEmpty for RangeFrom<N> {
    #[inline]
    fn is_range_empty(&self) -> bool {
        false
    }
}

/// Always return false for RangeTo
impl<N> IsRangeEmpty for RangeTo<N> {
    #[inline]
    fn is_range_empty(&self) -> bool {
        false
    }
}

/// Always return false for RangeToInclusive
impl<N> IsRangeEmpty for RangeToInclusive<N> {
    #[inline]
    fn is_range_empty(&self) -> bool {
        false
    }
}

/// Return true for Range if start >= end
impl<N: PartialOrd> IsRangeEmpty for Range<N> {
    #[inline]
    fn is_range_empty(&self) -> bool {
        self.start >= self.end
    }
}

/// Returns true for RangeInclusive if start > end
impl<N: PartialOrd> IsRangeEmpty for RangeInclusive<N> {
    #[inline]
    fn is_range_empty(&self) -> bool {
        self.start() > self.end()
    }
}

/// Return true if this bound tuple does not "contain" anything
impl<N: PartialOrd> IsRangeEmpty for (Bound<N>, Bound<N>) {
    fn is_range_empty(&self) -> bool {
        match self {
            (Included(l), Included(r)) => l > r,
            (Included(l), Excluded(r)) |
            (Excluded(l), Included(r)) |
            (Excluded(l), Excluded(r)) => l >= r,
            _ => false,
        }
    }
}
