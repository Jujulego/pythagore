use std::ops::{Bound, RangeBounds};
use na::{Point, Scalar};

/// Aligned Axis Bounding Box
pub trait BoundingBox<N: Scalar, const D: usize> {
    /// Returns range at given dimension
    fn get_range(&self, d: usize) -> (Bound<&N>, Bound<&N>);

    /// Test if given point is in the bbox
    fn holds(&self, pt: &Point<N, D>) -> bool
    where
        N: PartialOrd
    {
        (0..D)
            .map(|d| self.get_range(d))
            .zip(pt.iter())
            .all(|(range, x)| range.contains(x))
    }
}