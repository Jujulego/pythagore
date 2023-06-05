use std::ops::{Bound, RangeBounds};
use std::ops::Bound::*;
use num_traits::Num;
use crate::Point;
use crate::traits::BBoxBounded;

#[derive(Clone, Copy, Debug, Eq)]
pub struct BBox<'a, N: Num, const D: usize> {
    bounds: [(Bound<&'a N>, Bound<&'a N>); D],
}

// Utils
fn range_is_empty<'a, N: PartialOrd>(range: &'a (Bound<&'a N>, Bound<&'a N>)) -> bool {
    match range {
        (Included(l), Included(r)) => l > r,
        (Included(l), Excluded(r)) |
        (Excluded(l), Included(r)) |
        (Excluded(l), Excluded(r)) => l >= r,
        (Unbounded, _) => false,
        (_, Unbounded) => false,
    }
}

fn select_bound<'a, N, F>(lhs: &Bound<&'a N>, rhs: &Bound<&'a N>, selector: F) -> Bound<&'a N>
where F: FnOnce(&'a N, &'a N) -> bool
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

// Utils
impl<'a, N: Num, const D: usize> BBoxBounded<N, D> for BBox<'a, N, D>  {
    fn bbox(&self) -> BBox<'a, N, D> {
        BBox {
            bounds: self.bounds
        }
    }
}

impl<'a, N: Num, const D: usize> From<[(Bound<&'a N>, Bound<&'a N>); D]> for BBox<'a, N, D> {
    fn from(bounds: [(Bound<&'a N>, Bound<&'a N>); D]) -> Self {
        BBox { bounds }
    }
}

impl<'a, N: Num, const D: usize> FromIterator<(Bound<&'a N>, Bound<&'a N>)> for BBox<'a, N, D> {
    fn from_iter<T: IntoIterator<Item = (Bound<&'a N>, Bound<&'a N>)>>(iter: T) -> Self {
        let mut bounds = [(Unbounded, Unbounded); D];

        for (idx, pair) in iter.into_iter().take(D).enumerate() {
            bounds[idx] = pair;
        }

        BBox { bounds }
    }
}

// Operators
impl<'a, N: Num, const D: usize> PartialEq for BBox<'a, N, D> {
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
}