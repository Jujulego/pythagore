use crate::BBox;
use na::Scalar;

/// Defines object bounded by a bbox
/// Implemented on range types to help defining bboxes
pub trait BBoxBounded<N: Scalar, const D: usize> {
    fn bbox(&self) -> BBox<N, D>;
}
