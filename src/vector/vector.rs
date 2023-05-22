use num_traits::{Float, Num, Signed, Zero};
use std::ops;
use std::slice::Iter;

use crate::Scalar;
use crate::traits::Dimension;

/// `Vector<T, const D: usize>` structure for n dimension vectors
#[derive(Clone, Copy, Debug, Default, Eq)]
pub struct Vector<T: Copy + Num, const D: usize> {
    pub(crate) scalar: Scalar<T, D>,
}

// Methods
impl<T: Copy + Num, const D: usize> Vector<T, D> {
    /// Returns iterator on vector elements
    pub fn iter(&self) -> Iter<'_, T> {
        self.scalar[0..D-1].iter()
    }

    /// Returns a null vector
    ///
    /// ## Example
    /// ```
    /// use pythagore::{vector, Vector2D, Vector3D};
    ///
    /// assert_eq!(Vector2D::null(), vector!{ dx: 0, dy: 0 });
    /// assert_eq!(Vector3D::null(), vector!{ dx: 0, dy: 0, dz: 0 });
    /// ```
    pub fn null() -> Self {
        Self::zero()
    }

    /// Returns true if vector is null
    pub fn is_null(&self) -> bool {
        self.is_zero()
    }
}

impl<T: Copy + Num + ops::AddAssign, const D: usize> Vector<T, D> {
    /// Returns the squared norm of vector
    ///
    /// ## Example
    /// ```
    /// use pythagore::vector;
    ///
    /// assert_eq!(vector!{ dx: 2, dy: 3 }.square_norm(), 13);
    /// assert_eq!(vector!{ dx: 2, dy: 3, dz: 4 }.square_norm(), 29);
    /// ```
    pub fn square_norm(&self) -> T {
        let mut result = T::zero();

        for n in 0..D {
            result += self[n] * self[n];
        }

        result
    }
}

impl<T: Copy + Float + ops::AddAssign, const D: usize> Vector<T, D> {
    /// Returns the norm of vector (only for float vectors)
    ///
    /// ## Example
    /// ```
    /// use pythagore::vector;
    ///
    /// assert_eq!(vector!{ dx: 1.0, dy: 0.0 }.norm(), 1.0);
    /// assert_eq!(vector!{ dx: 0.0, dy: 0.0, dz: 4.0 }.norm(), 4.0);
    /// ```
    pub fn norm(&self) -> T {
        self.square_norm().sqrt()
    }

    /// Returns a unit vector from vector (only for float vectors)
    ///
    /// ## Example
    /// ```
    /// use pythagore::vector;
    ///
    /// assert_eq!(vector!{ dx: 10.0, dy: 0.0 }.unit(), vector!{ dx: 1.0, dy: 0.0 });
    /// assert_eq!(vector!{ dx: 0.0, dy: 0.0, dz: 5.0 }.unit(), vector!{ dx: 0.0, dy: 0.0, dz: 1.0 });
    /// ```
    pub fn unit(&self) -> Self {
        self / self.norm()
    }
}

impl<T: Copy + Signed + ops::AddAssign, const D: usize> Vector<T, D> {
    /// Returns the norm of vector (only for signed vectors)
    ///
    /// ## Example
    /// ```
    /// use pythagore::vector;
    ///
    /// assert_eq!(vector!{ dx: 1, dy: -2 }.manhattan_norm(), 3);
    /// assert_eq!(vector!{ dx: 1, dy: -2, dz: 3 }.manhattan_norm(), 6);
    /// ```
    pub fn manhattan_norm(&self) -> T {
        let mut result = T::zero();

        for n in 0..D {
            result += self[n].abs();
        }

        result
    }
}

// Utils
impl<T: Copy + Num, const D: usize> Dimension<D> for Vector<T, D> {
    /// Returns vector's dimension
    #[inline]
    fn dimension() -> usize {
        D - 1
    }
}

macro_rules! vector_from_array_impl {
    ($dim:literal) => {
        impl<T: Copy + Num> From<[T; { $dim - 1 }]> for Vector<T, $dim> {
            fn from(value: [T; { $dim - 1 }]) -> Self {
                Scalar::from(value).into()
            }
        }
    };
}

vector_from_array_impl!(3);
vector_from_array_impl!(4);

macro_rules! vector_from_scalar_impl {
    ($dim:literal) => {
        impl<T: Copy + Num> From<Scalar<T, { $dim - 1 }>> for Vector<T, $dim> {
            fn from(value: Scalar<T, { $dim - 1 }>) -> Self {
                let mut scalar = Scalar::zero();

                for n in 0..$dim - 1 {
                    scalar[n] = value[n];
                }

                Vector { scalar }
            }
        }
    };
}

vector_from_scalar_impl!(3);
vector_from_scalar_impl!(4);

impl<T: Copy + Num, const D: usize> Zero for Vector<T, D> {
    fn zero() -> Self {
        Vector { scalar: Scalar::zero() }
    }

    fn is_zero(&self) -> bool {
        self.scalar.is_zero()
    }
}

// Operators
impl<T: Copy + Num, const D: usize> PartialEq for Vector<T, D> {
    fn eq(&self, other: &Self) -> bool {
        self.scalar == other.scalar
    }
}

impl<T: Copy + Num, const D: usize> ops::Index<usize> for Vector<T, D> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.scalar[index]
    }
}

impl<T: Copy + Num, const D: usize> ops::IndexMut<usize> for Vector<T, D> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.scalar[index]
    }
}

macro_rules! vector_neg_impl {
    ($tp:ident, $dp:ident, $lhs:ty) => {
        impl<$tp: Copy + Signed, const $dp: usize> ops::Neg for $lhs {
            type Output = Vector<$tp, $dp>;

            fn neg(self) -> Self::Output {
                Vector { scalar: -self.scalar }
            }
        }
    };
}

vector_neg_impl!(T, D, Vector<T, D>);
vector_neg_impl!(T, D, &Vector<T, D>);

macro_rules! vector_add_assign_impl {
    ($tp:ident, $dp:ident, $rhs:ty) => {
        impl<$tp: Copy + Num, const $dp: usize> ops::AddAssign<$rhs> for Vector<$tp, $dp> {
            fn add_assign(&mut self, rhs: $rhs) {
                self.scalar += rhs.scalar;
            }
        }
    }
}

vector_add_assign_impl!(T, D, Vector<T, D>);
vector_add_assign_impl!(T, D, &Vector<T, D>);

macro_rules! vector_add_impl {
    ($tp:ident, $dp:ident, $lhs:ty, $rhs:ty) => {
        impl<$tp: Copy + Num, const $dp: usize> ops::Add<$rhs> for $lhs {
            type Output = Vector<$tp, $dp>;

            fn add(self, rhs: $rhs) -> Self::Output {
                Vector { scalar: self.scalar + rhs.scalar }
            }
        }
    }
}

vector_add_impl!(T, D, Vector<T, D>, Vector<T, D>);
vector_add_impl!(T, D, &Vector<T, D>, Vector<T, D>);
vector_add_impl!(T, D, Vector<T, D>, &Vector<T, D>);
vector_add_impl!(T, D, &Vector<T, D>, &Vector<T, D>);

macro_rules! vector_sub_assign_impl {
    ($tp:ident, $dp:ident, $rhs:ty) => {
        impl<$tp: Copy + Num, const $dp: usize> ops::SubAssign<$rhs> for Vector<$tp, $dp> {
            fn sub_assign(&mut self, rhs: $rhs) {
                self.scalar -= rhs.scalar;
            }
        }
    }
}

vector_sub_assign_impl!(T, D, Vector<T, D>);
vector_sub_assign_impl!(T, D, &Vector<T, D>);

macro_rules! vector_sub_impl {
    ($tp:ident, $dp:ident, $lhs:ty, $rhs:ty) => {
        impl<$tp: Copy + Num, const $dp: usize> ops::Sub<$rhs> for $lhs {
            type Output = Vector<$tp, $dp>;

            fn sub(self, rhs: $rhs) -> Self::Output {
                Vector { scalar: self.scalar - rhs.scalar }
            }
        }
    }
}

vector_sub_impl!(T, D, Vector<T, D>, Vector<T, D>);
vector_sub_impl!(T, D, &Vector<T, D>, Vector<T, D>);
vector_sub_impl!(T, D, Vector<T, D>, &Vector<T, D>);
vector_sub_impl!(T, D, &Vector<T, D>, &Vector<T, D>);

macro_rules! vector_mul_assign_impl {
    ($tp:ident, $dp:ident, $rhs:ty $(, $defer:tt)?) => {
        impl<$tp: Copy + Num, const $dp: usize> ops::MulAssign<$rhs> for Vector<$tp, $dp> {
            fn mul_assign(&mut self, rhs: $rhs) {
                self.scalar *= $($defer)?rhs;
            }
        }
    }
}

vector_mul_assign_impl!(T, D, T);
vector_mul_assign_impl!(T, D, &T, *);

macro_rules! vector_mul_impl {
    ($tp:ident, $dp:ident, $lhs:ty, $rhs:ty $(, $defer:tt)?) => {
        impl<$tp: Copy + Num, const $dp: usize> ops::Mul<$rhs> for $lhs {
            type Output = Vector<$tp, $dp>;

            fn mul(self, rhs: $rhs) -> Self::Output {
                Vector { scalar: self.scalar * $($defer)?rhs }
            }
        }
    }
}

vector_mul_impl!(T, D, Vector<T, D>, T);
vector_mul_impl!(T, D, &Vector<T, D>, T);
vector_mul_impl!(T, D, Vector<T, D>, &T, *);
vector_mul_impl!(T, D, &Vector<T, D>, &T, *);

macro_rules! vector_scalar_impl {
    ($tp:ident, $dp:ident, $lhs:ty, $rhs:ty) => {
        impl<$tp: Copy + Num + ops::AddAssign, const $dp: usize> ops::Mul<$rhs> for $lhs {
            type Output = $tp;

            fn mul(self, rhs: $rhs) -> Self::Output {
                self.scalar * rhs.scalar
            }
        }
    }
}

vector_scalar_impl!(T, D, Vector<T, D>, Vector<T, D>);
vector_scalar_impl!(T, D, &Vector<T, D>, Vector<T, D>);
vector_scalar_impl!(T, D, Vector<T, D>, &Vector<T, D>);
vector_scalar_impl!(T, D, &Vector<T, D>, &Vector<T, D>);

macro_rules! vector_div_assign_impl {
    ($tp:ident, $dp:ident, $rhs:ty $(, $defer:tt)?) => {
        impl<$tp: Copy + Num, const $dp: usize> ops::DivAssign<$rhs> for Vector<$tp, $dp> {
            fn div_assign(&mut self, rhs: $rhs) {
                self.scalar /= $($defer)?rhs;
            }
        }
    }
}

vector_div_assign_impl!(T, D, T);
vector_div_assign_impl!(T, D, &T, *);

macro_rules! vector_div_impl {
    ($tp:ident, $dp:ident, $lhs:ty, $rhs:ty $(, $defer:tt)?) => {
        impl<$tp: Copy + Num, const $dp: usize> ops::Div<$rhs> for $lhs {
            type Output = Vector<$tp, $dp>;

            fn div(self, rhs: $rhs) -> Self::Output {
                Vector { scalar: self.scalar / $($defer)?rhs }
            }
        }
    }
}

vector_div_impl!(T, D, Vector<T, D>, T);
vector_div_impl!(T, D, &Vector<T, D>, T);
vector_div_impl!(T, D, Vector<T, D>, &T, *);
vector_div_impl!(T, D, &Vector<T, D>, &T, *);

// Tests
#[cfg(test)]
mod tests {
    use crate::vector;
    use super::*;

    #[test]
    fn vector_is_null() {
        assert!(Vector::<f32, 4>::null().is_null());

        assert!(!vector!{ dx: 1, dy: 2 }.is_null());
        assert!(!vector!{ dx: 1, dy: 2, dz: 3 }.is_null());
    }

    #[test]
    fn vector_2d_norm() {
        assert_eq!(vector!{ dx: 2.0, dy: 3.0 }.norm(), (13.0 as f32).sqrt());
    }

    #[test]
    fn vector_3d_norm() {
        assert_eq!(vector!{ dx: 2.0, dy: 3.0, dz: 4.0 }.norm(), (29.0 as f32).sqrt());
    }

    #[test]
    fn vector_2d_unit() {
        let v = vector!{ dx: 2.0, dy: 3.0 };
        let norm = v.norm();

        assert_eq!(v.unit(), vector!{ dx: v.dx() / norm, dy: v.dy() / norm });

        assert_eq!(v.unit().norm(), 1.0);
        assert_eq!(v.unit().scalar[2], 0.0);
    }

    #[test]
    fn vector_3d_unit() {
        let v = vector!{ dx: 2.0, dy: 3.0, dz: 4.0 };
        let norm = v.norm();

        assert_eq!(v.unit(), vector!{ dx: v.dx() / norm, dy: v.dy() / norm, dz: v.dz() / norm });

        assert_eq!(v.unit().norm(), 1.0);
        assert_eq!(v.unit().scalar[3], 0.0);
    }
}