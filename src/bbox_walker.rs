mod iter;

use std::ops::AddAssign;
use na::{Point, Scalar};
use num_traits::One;
use crate::bbox_walker::iter::Iter;

/// Generates points inside a bbox, in xy order.
#[derive(Clone, Copy, Debug)]
pub struct BBoxWalker<N: Scalar, const D: usize> {
    first: Point<N, D>,
    last: Point<N, D>,
}

impl<N: Scalar, const D: usize> BBoxWalker<N, D> {
    /// Builds a BBox Walker, moving inside a bbox going from first to last included.
    /// Uses a default step size of 1
    pub fn new(first: Point<N, D>, last: Point<N, D>) -> BBoxWalker<N, D> {
        BBoxWalker {
            first,
            last
        }
    }

    /// First available point
    pub fn first(&self) -> &Point<N, D> {
        &self.first
    }

    /// Last available point
    pub fn last(&self) -> &Point<N, D> {
        &self.last
    }

    /// Returns iterator on walked points
    #[inline]
    pub fn iter(&self) -> Iter<'_, N, D> {
        Iter::new(self)
    }

    /// Computes next point, if exists from "from" point.
    pub fn next(&self, from: &Point<N, D>) -> Option<Point<N, D>>
    where
        N: AddAssign + Copy + One + Ord
    {
        if from == &self.last || unsafe { from.get_unchecked(0) > self.last.get_unchecked(0) } {
            return None;
        }

        let mut next = self.first;
        let mut addable: Option<usize> = None;

        for (idx, v) in from.iter().enumerate() {
            if v < unsafe { self.first.get_unchecked(idx) } {
                break;
            } else if v < unsafe { self.last.get_unchecked(idx) } {
                unsafe { *next.get_unchecked_mut(idx) = *v };
                addable = Some(idx);

                if idx == D - 1 {
                    unsafe { *next.get_unchecked_mut(idx) += N::one() };
                }
            } else {
                if let Some(back) = addable {
                    unsafe { *next.get_unchecked_mut(back) += N::one() };

                    return Some(next);
                } else {
                    unsafe { *next.get_unchecked_mut(idx) = *self.last.get_unchecked(idx) };
                }
            }
        }

        Some(next)
    }
}

// Utils
impl<'a, N: AddAssign + Copy + One + Ord + Scalar, const D: usize> IntoIterator for &'a BBoxWalker<N, D> {
    type Item = Point<N, D>;
    type IntoIter = Iter<'a, N, D>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// Tests
#[cfg(test)]
mod tests {
    use na::point;
    use super::*;

    #[test]
    fn test_next_on_whole_range() {
        let walker = BBoxWalker::new(point![0, 0], point![2, 2]);

        assert_eq!(walker.next(&point![0, 0]), Some(point![0, 1]));
        assert_eq!(walker.next(&point![0, 1]), Some(point![0, 2]));
        assert_eq!(walker.next(&point![0, 2]), Some(point![1, 0]));
        assert_eq!(walker.next(&point![1, 0]), Some(point![1, 1]));
        assert_eq!(walker.next(&point![1, 1]), Some(point![1, 2]));
        assert_eq!(walker.next(&point![1, 2]), Some(point![2, 0]));
        assert_eq!(walker.next(&point![2, 0]), Some(point![2, 1]));
        assert_eq!(walker.next(&point![2, 1]), Some(point![2, 2]));
        assert_eq!(walker.next(&point![2, 2]), None);
    }

    #[test]
    fn test_iterator() {
        let walker = BBoxWalker::new(point![0, 0], point![2, 2]);
        let mut iter = walker.iter();

        assert_eq!(iter.next(), Some(point![0, 0]));
        assert_eq!(iter.next(), Some(point![0, 1]));
        assert_eq!(iter.next(), Some(point![0, 2]));
        assert_eq!(iter.next(), Some(point![1, 0]));
        assert_eq!(iter.next(), Some(point![1, 1]));
        assert_eq!(iter.next(), Some(point![1, 2]));
        assert_eq!(iter.next(), Some(point![2, 0]));
        assert_eq!(iter.next(), Some(point![2, 1]));
        assert_eq!(iter.next(), Some(point![2, 2]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_below_left_point() {
        let walker = BBoxWalker::new(point![0, 0], point![2, 2]);

        assert_eq!(walker.next(&point![-1, -1]), Some(point![0, 0]));
    }

    #[test]
    fn test_left_point() {
        let walker = BBoxWalker::new(point![0, 0], point![2, 2]);

        assert_eq!(walker.next(&point![-1, 1]), Some(point![0, 0]));
    }

    #[test]
    fn test_over_left_point() {
        let walker = BBoxWalker::new(point![0, 0], point![2, 2]);

        assert_eq!(walker.next(&point![-1, 3]), Some(point![0, 0]));
    }

    #[test]
    fn test_below_point() {
        let walker = BBoxWalker::new(point![0, 0], point![2, 2]);

        assert_eq!(walker.next(&point![1, -1]), Some(point![1, 0]));
    }

    #[test]
    fn test_over_point() {
        let walker = BBoxWalker::new(point![0, 0], point![2, 2]);

        assert_eq!(walker.next(&point![1, 3]), Some(point![2, 0]));
    }

    #[test]
    fn test_below_right_point() {
        let walker = BBoxWalker::new(point![0, 0], point![2, 2]);

        assert_eq!(walker.next(&point![3, -1]), None);
    }

    #[test]
    fn test_right_point() {
        let walker = BBoxWalker::new(point![0, 0], point![2, 2]);

        assert_eq!(walker.next(&point![3, 1]), None);
    }

    #[test]
    fn test_over_right_point() {
        let walker = BBoxWalker::new(point![0, 0], point![2, 2]);

        assert_eq!(walker.next(&point![3, 3]), None);
    }
}