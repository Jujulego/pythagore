use na::{Point, Scalar};
use crate::BBoxWalker;

pub trait Walkable<N: Scalar, const D: usize> {
    fn first_point(&self) -> Option<Point<N, D>>;
    fn last_point(&self) -> Option<Point<N, D>>;

    fn walk(&self) -> Result<BBoxWalker<N, D>, &str> {
        match (self.first_point(), self.last_point()) {
            (Some(first), Some(last)) => Ok(BBoxWalker::new(first, last)),
            (None, _) => Err("No first point defined"),
            (_, None) => Err("No last point defined"),
        }
    }
}