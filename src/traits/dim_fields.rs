use na::{Point, Scalar, SVector};

pub trait DimFields<const D: usize> {
    type Output;

    /// Returns object's field for given dimension.
    ///
    /// # Examples
    /// ```
    /// use nalgebra::vector;
    /// use pythagore::traits::DimFields;
    ///
    /// let v = vector![1, 2];
    ///
    /// assert_eq!(v.get_field(1), 2);
    /// ```
    #[inline]
    fn get_field(&self, dim: usize) -> &Self::Output {
        assert!(dim < D, "Dimension index out of bounds");
        unsafe { self.get_field_unchecked(dim) }
    }

    /// Returns object's field for given dimension.
    ///
    /// # Safety
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*
    /// even if the resulting reference is not used.
    ///
    /// # Examples
    /// ```
    /// use nalgebra::vector;
    /// use pythagore::traits::DimFields;
    ///
    /// let v = vector![1, 2];
    ///
    /// unsafe {
    ///     assert_eq!(v.get_field_unchecked(1), 2);
    /// }
    /// ```
    unsafe fn get_field_unchecked(&self, dim: usize) -> &Self::Output;
}

// Macros
macro_rules! literal_impl {
    ($t:ty) => {
        impl DimFields<1> for $t {
            type Output = Self;

            #[inline]
            unsafe fn get_field_unchecked(&self, _dim: usize) -> &Self::Output {
                self
            }
        }
    };
    ($($t:ty),+) => { $(literal_impl!($t);)* };
}

// Implementations
literal_impl!(u8, u16, u32, u64, u128);
literal_impl!(i8, i16, i32, i64, i128);

impl<N, const D: usize> DimFields<D> for SVector<N, D> {
    type Output = N;

    #[inline]
    unsafe fn get_field_unchecked(&self, dim: usize) -> &Self::Output {
        &self[dim]
    }
}

impl<N: Scalar, const D: usize> DimFields<D> for Point<N, D> {
    type Output = N;

    #[inline]
    unsafe fn get_field_unchecked(&self, dim: usize) -> &Self::Output {
        &self[dim]
    }
}