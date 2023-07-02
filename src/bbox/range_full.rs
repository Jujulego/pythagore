use std::ops::Bound::Unbounded;
use std::ops::RangeFull;
use na::Scalar;

use crate::BBox;

/// Builds a bounding box from a range of points
///
/// # Example
/// ```
/// use std::ops::Bound::Unbounded;
/// use nalgebra::point;
/// use pythagore::BBox;
///
/// assert_eq!(
///     BBox::<i32, 2>::from(..),
///     BBox::from([
///        (Unbounded, Unbounded),
///        (Unbounded, Unbounded),
///     ])
/// )
/// ```
impl<N: Copy + Scalar, const D: usize> From<RangeFull> for BBox<N, D> {
    fn from(_value: RangeFull) -> Self {
        BBox::from([(Unbounded, Unbounded); D])
    }
}