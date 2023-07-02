use na::{Point, Scalar};

/// Limit points
pub trait BoundPoints<N: Scalar, const D: usize> {
    /// Returns points made from start bounds
    fn start_point(&self) -> Point<N, D>;

    /// Returns points made from end bounds
    fn end_point(&self) -> Point<N, D>;
}