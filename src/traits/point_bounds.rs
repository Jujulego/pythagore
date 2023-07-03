use na::{Point, Scalar};

pub trait PointBounds<N: Scalar, const D: usize> {
    fn start_point(&self) -> Option<Point<N, D>>;

    fn end_point(&self) -> Option<Point<N, D>>;
}