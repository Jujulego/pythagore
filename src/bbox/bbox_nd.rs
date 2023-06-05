use std::hash::{Hash, Hasher};
use std::ops::{Bound, RangeBounds};
use std::ops::Bound::*;
use num_traits::Num;
use crate::Point;
use crate::traits::BBoxBounded;

/// `BBox<N, D>` structure for D dimension bounding boxes
#[derive(Clone, Copy, Debug, Eq)]
pub struct BBox<'n, N: Num, const D: usize> {
    bounds: [(Bound<&'n N>, Bound<&'n N>); D],
}

// Utils
fn range_is_empty<'n, N: PartialOrd>(range: &'n (Bound<&'n N>, Bound<&'n N>)) -> bool {
    match range {
        (Included(l), Included(r)) => l > r,
        (Included(l), Excluded(r)) |
        (Excluded(l), Included(r)) |
        (Excluded(l), Excluded(r)) => l >= r,
        (Unbounded, _) => false,
        (_, Unbounded) => false,
    }
}

fn select_bound<'n, N, F>(lhs: &Bound<&'n N>, rhs: &Bound<&'n N>, selector: F) -> Bound<&'n N>
where F: FnOnce(&'n N, &'n N) -> bool
{
    match (lhs, rhs) {
        (Included(l), Included(r)) |
        (Included(l), Excluded(r)) |
        (Excluded(l), Included(r)) |
        (Excluded(l), Excluded(r)) => if selector(*l, *r) { *lhs } else { *rhs }
        (Unbounded, _) => *rhs,
        (_, Unbounded) => *lhs,
    }
}

fn include_value<'n, N: PartialEq, F>(bound: &Bound<&'n N>, x: &'n N, selector: F) -> Bound<&'n N>
where F: FnOnce(&'n N, &'n N) -> bool
{
    match bound {
        Unbounded => Unbounded,
        Excluded(b) => if selector(*b, x) { *bound } else { Included(x) },
        Included(b) => if *b == x || selector(*b, x) { *bound } else { Included(x) }
    }
}

// Methods
impl<N: Num + PartialOrd, const D: usize> BBox<'_, N, D> {
    /// Returns true if bbox is empty
    pub fn is_empty(&self) -> bool {
        self.bounds.iter().any(range_is_empty)
    }

    /// Returns true if bbox contains given point
    pub fn contains(&self, pt: &Point<N, D>) -> bool {
        self.bounds.iter()
            .zip(pt.iter())
            .all(|(bounds, x)| bounds.contains(x))
    }

    /// Returns intersection between bbox
    pub fn intersection(&self, other: &Self) -> Self {
        self.bounds.iter()
            .zip(other.bounds.iter())
            .map(|(l, r)| (
                select_bound(&l.0, &r.0, |a, b| a >= b),
                select_bound(&l.1, &r.1, |a, b| a <= b),
            ))
            .collect()
    }
}

impl<'n, N: Num + PartialOrd, const D: usize> BBox<'n, N, D> {
    /// Returns a new bbox including the given point
    pub fn include(&self, pt: &'n Point<N, D>) -> BBox<'n, N, D> {
        self.bounds.iter()
            .zip(pt.iter())
            .map(|(bounds, x)| (
                include_value(&bounds.0, x, |a, b| a < b),
                include_value(&bounds.1, x, |a, b| a > b),
            ))
            .collect()
    }
}

// Utils
impl<'n, N: Num, const D: usize> BBoxBounded<N, D> for BBox<'n, N, D>  {
    fn bbox(&self) -> BBox<'n, N, D> {
        BBox {
            bounds: self.bounds
        }
    }
}

impl<'n, N: Num, const D: usize> Default for BBox<'_, N, D> {
    fn default() -> Self {
        BBox {
            bounds: [(Unbounded, Unbounded); D],
        }
    }
}

impl<N: Num + Hash, const D: usize> Hash for BBox<'_, N, D> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bounds.hash(state);
    }
}

impl<'n, N: Num, const D: usize> From<[(Bound<&'n N>, Bound<&'n N>); D]> for BBox<'n, N, D> {
    fn from(bounds: [(Bound<&'n N>, Bound<&'n N>); D]) -> Self {
        BBox { bounds }
    }
}

impl<'n, N: Num, const D: usize> FromIterator<(Bound<&'n N>, Bound<&'n N>)> for BBox<'n, N, D> {
    fn from_iter<T: IntoIterator<Item = (Bound<&'n N>, Bound<&'n N>)>>(iter: T) -> Self {
        let mut bounds = [(Unbounded, Unbounded); D];

        for (idx, pair) in iter.into_iter().take(D).enumerate() {
            bounds[idx] = pair;
        }

        BBox { bounds }
    }
}

// Operators
impl<'n, N: Num, const D: usize> PartialEq for BBox<'n, N, D> {
    fn eq(&self, other: &Self) -> bool {
        self.bounds == other.bounds
    }
}

// Tests
#[cfg(test)]
mod tests {
    use std::ops::Bound::{Excluded, Included, Unbounded};
    use crate::bbox::bbox_nd::BBox;
    use crate::point;
    use crate::traits::BBoxBounded;

    #[test]
    fn bbox_is_empty() {
        let a: BBox<u32, 3> = [
            (Included(&0), Included(&5)),
            (Included(&0), Included(&5)),
            (Included(&7), Included(&5)),
        ].into();

        assert!(a.is_empty());
    }

    #[test]
    fn bbox_is_not_empty() {
        let a: BBox<u32, 3> = [
            (Included(&0), Included(&5)),
            (Included(&0), Included(&5)),
            (Included(&0), Included(&5)),
        ].into();

        assert!(!a.is_empty());
    }

    #[test]
    fn bbox_contains() {
        let a: BBox<i32, 3> = [
            (Included(&0), Included(&5)),
            (Included(&0), Included(&5)),
            (Unbounded, Unbounded),
        ].into();

        assert!(a.contains(&point![2, 2]));

        assert!(!a.contains(&point![-3, 2]));
        assert!(!a.contains(&point![-3, -3]));
        assert!(!a.contains(&point![2, -3]));
        assert!(!a.contains(&point![7, -3]));
        assert!(!a.contains(&point![7, 2]));
        assert!(!a.contains(&point![7, 7]));
        assert!(!a.contains(&point![2, 7]));
        assert!(!a.contains(&point![-3, 7]));
    }

    #[test]
    fn bbox_intersection_overlaps() {
        let a: BBox<u32, 1> = [(Included(&0), Included(&5))].into();
        let b: BBox<u32, 1> = [(Included(&2), Included(&7))].into();

        assert_eq!(a.intersection(&b), [(Included(&2), Included(&5))].into());
    }

    #[test]
    fn bbox_intersection_contains() {
        let a: BBox<u32, 1> = [(Included(&0), Included(&7))].into();
        let b: BBox<u32, 1> = [(Included(&2), Included(&5))].into();

        assert_eq!(a.intersection(&b), b);
    }

    #[test]
    fn bbox_intersection_no_intersection() {
        let a: BBox<u32, 1> = [(Included(&0), Included(&2))].into();
        let b: BBox<u32, 1> = [(Included(&5), Included(&7))].into();

        assert_eq!(a.intersection(&b), [(Included(&5), Included(&2))].into());
    }

    #[test]
    fn bbox_intersection_some_included_some_excluded() {
        let a: BBox<u32, 1> = [(Included(&0), Included(&5))].into();
        let b: BBox<u32, 1> = [(Excluded(&2), Excluded(&7))].into();

        assert_eq!(a.intersection(&b), [(Excluded(&2), Included(&5))].into());
    }

    #[test]
    fn bbox_intersection_some_unbounded() {
        let a: BBox<u32, 1> = [(Included(&0), Unbounded)].into();
        let b: BBox<u32, 1> = [(Unbounded, Excluded(&7))].into();

        assert_eq!(a.intersection(&b), [(Included(&0), Excluded(&7))].into());
    }

    #[test]
    fn bbox_intersection_one_fully_unbounded() {
        let a: BBox<u32, 1> = [(Included(&0), Included(&5))].into();
        let b: BBox<u32, 1> = [(Unbounded, Unbounded)].into();

        assert_eq!(a.intersection(&b), a);
    }

    #[test]
    fn bbox_intersection_no_bounds() {
        let a: BBox<u32, 1> = [(Unbounded, Unbounded)].into();
        let b: BBox<u32, 1> = [(Unbounded, Unbounded)].into();

        assert_eq!(a.intersection(&b), a);
    }

    #[test]
    fn bbox_include() {
        let range = point![2]..point![6];

        assert_eq!(range.bbox().include(&point![0]), (point![0]..point![6]).bbox());
        assert_eq!(range.bbox().include(&point![4]), (point![2]..point![6]).bbox());
        assert_eq!(range.bbox().include(&point![6]), (point![2]..=point![6]).bbox());
        assert_eq!(range.bbox().include(&point![8]), (point![2]..=point![8]).bbox());
        assert_eq!((..).bbox().include(&point![8]), (..).bbox());
    }
}