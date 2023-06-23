mod range;
mod utils;

use na::{Point, Scalar};
use std::hash::{Hash, Hasher};
use std::ops::RangeBounds;
use std::ops::Bound::{self as Bound, *};

use crate::bbox::utils::*;
use crate::traits::BBoxBounded;

/// `BBox<N, D>` structure for D dimension bounding boxes
#[derive(Clone, Copy, Debug, Eq)]
pub struct BBox<N: Scalar, const D: usize> {
    bounds: [(Bound<N>, Bound<N>); D],
}

// Methods
impl<N: Copy + Scalar + PartialOrd, const D: usize> BBox<N, D> {
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
        let mut result = BBox::default();

        for dim in 0..D {
            result.bounds[dim] = (
                select_bound(self.bounds[dim].0, other.bounds[dim].0, |a, b| a >= b),
                select_bound(self.bounds[dim].1, other.bounds[dim].1, |a, b| a <= b),
            );
        }

        result
    }

    /// Returns a new bbox including the given point
    pub fn include(&self, pt: &Point<N, D>) -> BBox<N, D> {
        let mut result = BBox::default();

        for dim in 0..D {
            result.bounds[dim] = (
                include_value(self.bounds[dim].0, &pt[dim], |a, b| a < b),
                include_value(self.bounds[dim].1, &pt[dim], |a, b| a > b),
            );
        }

        result
    }
}

// Utils
impl<N: Copy + Scalar, const D: usize> BBoxBounded<N, D> for BBox<N, D>  {
    fn bbox(&self) -> BBox<N, D> {
        *self
    }
}

impl<N: Copy + Scalar, const D: usize> Default for BBox<N, D> {
    fn default() -> Self {
        BBox {
            bounds: [(Unbounded, Unbounded); D],
        }
    }
}

impl<N: Scalar + Hash, const D: usize> Hash for BBox<N, D> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bounds.hash(state);
    }
}

// Conversions
impl<N: Scalar, const D: usize> From<[(Bound<N>, Bound<N>); D]> for BBox<N, D> {
    fn from(bounds: [(Bound<N>, Bound<N>); D]) -> Self {
        BBox { bounds }
    }
}

// Operators
impl<N: Scalar, const D: usize> PartialEq for BBox<N, D> {
    fn eq(&self, other: &Self) -> bool {
        self.bounds == other.bounds
    }
}

// Tests
#[cfg(test)]
mod tests {
    use std::ops::Bound::{Excluded, Included, Unbounded};
    use na::point;
    use super::*;
    use crate::traits::BBoxBounded;

    #[test]
    fn bbox_is_empty() {
        let a: BBox<u32, 3> = [
            (Included(0), Included(5)),
            (Included(0), Included(5)),
            (Included(7), Included(5)),
        ].into();

        assert!(a.is_empty());
    }

    #[test]
    fn bbox_is_not_empty() {
        let a: BBox<u32, 3> = [
            (Included(0), Included(5)),
            (Included(0), Included(5)),
            (Included(0), Included(5)),
        ].into();

        assert!(!a.is_empty());
    }

    #[test]
    fn bbox_contains() {
        let a: BBox<i32, 2> = [
            (Included(0), Included(5)),
            (Included(0), Included(5)),
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
        let a: BBox<u32, 1> = [(Included(0), Included(5))].into();
        let b: BBox<u32, 1> = [(Included(2), Included(7))].into();

        assert_eq!(a.intersection(&b), [(Included(2), Included(5))].into());
    }

    #[test]
    fn bbox_intersection_contains() {
        let a: BBox<u32, 1> = [(Included(0), Included(7))].into();
        let b: BBox<u32, 1> = [(Included(2), Included(5))].into();

        assert_eq!(a.intersection(&b), b);
    }

    #[test]
    fn bbox_intersection_no_intersection() {
        let a: BBox<u32, 1> = [(Included(0), Included(2))].into();
        let b: BBox<u32, 1> = [(Included(5), Included(7))].into();

        assert_eq!(a.intersection(&b), [(Included(5), Included(2))].into());
    }

    #[test]
    fn bbox_intersection_some_included_some_excluded() {
        let a: BBox<u32, 1> = [(Included(0), Included(5))].into();
        let b: BBox<u32, 1> = [(Excluded(2), Excluded(7))].into();

        assert_eq!(a.intersection(&b), [(Excluded(2), Included(5))].into());
    }

    #[test]
    fn bbox_intersection_some_unbounded() {
        let a: BBox<u32, 1> = [(Included(0), Unbounded)].into();
        let b: BBox<u32, 1> = [(Unbounded, Excluded(7))].into();

        assert_eq!(a.intersection(&b), [(Included(0), Excluded(7))].into());
    }

    #[test]
    fn bbox_intersection_one_fully_unbounded() {
        let a: BBox<u32, 1> = [(Included(0), Included(5))].into();
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