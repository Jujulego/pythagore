use std::ops::RangeBounds;
use na::Scalar;

pub trait DimensionBounds<N: Scalar, const D: usize> {
    type Output: RangeBounds<N>;

    fn get_bounds(&self, idx: usize) -> Self::Output {
        assert!(idx < D, "Dimension index out of bounds");
        unsafe { self.get_bounds_unchecked(idx) }
    }

    unsafe fn get_bounds_unchecked(&self, idx: usize) -> Self::Output;
}
