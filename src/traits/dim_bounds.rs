use std::ops::RangeBounds;
use na::Scalar;

pub trait DimBounds<N: Scalar, const D: usize> {
    type Output: RangeBounds<N>;

    /// Returns object dimension bounds at given dimension.
    ///
    /// # Examples
    /// ```
    /// use std::ops::Bound::{Excluded, Included};
    /// use nalgebra::point;
    /// use pythagore::traits::DimBounds;
    ///
    /// let bbox = point![0, 0]..point![1, 1];
    ///
    /// assert_eq!(bbox.get_bounds(0), 0..1);
    /// ```
    fn get_bounds(&self, dim: usize) -> Self::Output {
        assert!(dim < D, "Dimension index out of bounds");
        unsafe { self.get_bounds_unchecked(dim) }
    }

    /// Returns object dimension bounds at given dimension.
    ///
    /// # Safety
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*
    /// even if the resulting reference is not used.
    ///
    /// # Examples
    /// ```
    /// use std::ops::Bound::{Excluded, Included};
    /// use nalgebra::point;
    /// use pythagore::traits::DimBounds;
    ///
    /// let bbox = point![0, 0]..point![1, 1];
    ///
    /// unsafe {
    ///     assert_eq!(bbox.get_bounds_unchecked(0), 0..1);
    /// }
    /// ```
    unsafe fn get_bounds_unchecked(&self, dim: usize) -> Self::Output;
}
