use std::ops::Bound::*;
use std::ops::{Bound, Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};
use num_traits::Num;
use crate::traits::ScalarNum;

/// Implemented by range types to define bounding box using range syntax
///
/// ## Example
/// ```
/// use pythagore::{point, Point};
/// use pythagore::traits::BoxBounds;
///
/// let bbox = Point::origin()..point!{ x: 5, y: 5 };
///
/// assert!(bbox.box_contains(&point!{ x: 2, y: 2 }));
/// ```
pub trait BoxBounds<N, T>: RangeBounds<T>
where
    N: Num + PartialOrd,
    T: ScalarNum<N, Output = N>
{
    fn contains_element(&self, elem: &N, idx: usize) -> bool {
        let range = (
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
        );

        range.contains(&elem)
    }

    fn box_contains(&self, item: &T) -> bool {
        item.iter().enumerate()
            .all(|(idx, elem)| self.contains_element(elem, idx))
    }
}

impl<N: Num + PartialOrd, T: ScalarNum<N, Output = N>> BoxBounds<N, T> for RangeFull {}
impl<N: Num + PartialOrd, T: ScalarNum<N, Output = N>> BoxBounds<N, T> for RangeFrom<T> {}
impl<N: Num + PartialOrd, T: ScalarNum<N, Output = N>> BoxBounds<N, T> for RangeTo<T> {}
impl<N: Num + PartialOrd, T: ScalarNum<N, Output = N>> BoxBounds<N, T> for Range<T> {}
impl<N: Num + PartialOrd, T: ScalarNum<N, Output = N>> BoxBounds<N, T> for RangeInclusive<T> {}
impl<N: Num + PartialOrd, T: ScalarNum<N, Output = N>> BoxBounds<N, T> for RangeToInclusive<T> {}
impl<N: Num + PartialOrd, T: ScalarNum<N, Output = N>> BoxBounds<N, T> for (Bound<T>, Bound<T>) {}
