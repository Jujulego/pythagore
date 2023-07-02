use std::ops::{Bound, Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

/// Tests if an object in holded by an other
pub trait Holds<I> {
    fn holds(&self, object: &I) -> bool;
}

// Implementations
impl<T: PartialOrd> Holds<T> for RangeFrom<T> {
    #[inline]
    fn holds(&self, object: &T) -> bool {
        self.contains(object)
    }
}

impl<T> Holds<T> for RangeFull {
    #[inline]
    fn holds(&self, _object: &T) -> bool {
        true
    }
}

impl<T: PartialOrd> Holds<T> for RangeTo<T> {
    #[inline]
    fn holds(&self, object: &T) -> bool {
        self.contains(object)
    }
}

impl<T: PartialOrd> Holds<T> for RangeToInclusive<T> {
    #[inline]
    fn holds(&self, object: &T) -> bool {
        self.contains(object)
    }
}

impl<T: PartialOrd> Holds<T> for Range<T> {
    #[inline]
    fn holds(&self, object: &T) -> bool {
        self.contains(object)
    }
}

impl<T: PartialOrd> Holds<T> for RangeInclusive<T> {
    #[inline]
    fn holds(&self, object: &T) -> bool {
        self.contains(object)
    }
}

impl<T: PartialOrd> Holds<T> for (Bound<T>, Bound<T>) {
    #[inline]
    fn holds(&self, object: &T) -> bool {
        self.contains(object)
    }
}
