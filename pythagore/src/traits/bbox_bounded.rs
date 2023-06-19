use na::Scalar;
use crate::BBox;

/// Defines object bounded by a bbox
/// Implemented on range types to help defining bboxes
///
/// ## Example
/// ```
/// use pythagore::{point, Point};
/// use pythagore::traits::BBoxBounded;
///
/// let range = Point::origin()..point!{ x: 5, y: 5 };
///
/// assert!(range.bbox().contains(&point!{ x: 2, y: 2 }));
/// ```
pub trait BBoxBounded<N: Scalar, const D: usize> {
    fn bbox(&self) -> BBox<'_, N, D>;
}
