use na::Scalar;
use crate::BBox;

/// Defines object bounded by a bbox
/// Implemented on range types to help defining bboxes
///
/// ## Example
/// ```
/// use nalgebra::{point, Point};
/// use pythagore::traits::BBoxBounded;
///
/// let range = Point::origin()..point![5, 5];
///
/// assert!(range.bbox().contains(&point![2, 2]));
/// ```
pub trait BBoxBounded<N: Scalar, const D: usize> {
    fn bbox(&self) -> BBox<N, D>;
}
