use std::ops::{Bound, RangeBounds};
use std::ops::Bound::Unbounded;
use num_traits::Num;
use crate::{BBoxBounded, Point};

#[derive(Clone, Copy, Debug)]
pub struct BBox<'a, N, const D: usize> {
    bounds: [(Bound<&'a N>, Bound<&'a N>); D],
}

// Methods
impl<N: Num + PartialOrd, const D: usize> BBox<'_, N, D> {
    /// Returns true if bbox contains given point
    pub fn contains(&self, pt: &Point<N, D>) -> bool {
        self.bounds.iter()
            .zip(pt.iter())
            .all(|(bounds, x)| bounds.contains(x))
    }
}

// Utils
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