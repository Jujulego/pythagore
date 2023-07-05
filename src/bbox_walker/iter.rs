use std::ops::AddAssign;
use na::{Point, Scalar};
use num_traits::One;
use crate::BBoxWalker;

pub struct Iter<'a, N: Scalar, const D: usize> {
    last: Option<Point<N, D>>,
    walker: &'a BBoxWalker<N, D>
}

impl<'a, N: Scalar, const D: usize> Iter<'a, N, D> {
    pub fn new(walker: &'a BBoxWalker<N, D>) -> Iter<'a, N, D> {
        Iter {
            last: None,
            walker,
        }
    }
}

impl<'a, N: AddAssign + Copy + One + Ord + Scalar, const D: usize> Iterator for Iter<'a, N, D> {
    type Item = Point<N, D>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(last) = &self.last {
            let next = self.walker.next(last);

            if next.is_some() {
                self.last = next;
            }

            next
        } else {
            self.last = Some(*self.walker.first());
            self.last
        }
    }
}