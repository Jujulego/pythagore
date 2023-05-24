use std::iter::Sum;
use std::ops;
use std::slice::{Iter, SliceIndex};
use num_traits::{Float, Num, Signed, Zero};

use crate::Scalar;
use crate::traits::Dimension;

/// `Vector<N, const D: usize>` structure for n dimension vectors
#[derive(Clone, Copy, Debug, Eq)]
pub struct Vector<N: Num, const D: usize> {
    pub(crate) scalar: Scalar<N, D>,
}

// Methods
impl<N: Copy + Num, const D: usize> Vector<N, D> {
    /// Returns iterator on vector elements
    #[inline]
    pub fn iter(&self) -> Iter<'_, N> {
        self.scalar[..D-1].iter()
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
    #[inline]
    pub fn null() -> Self {
        Self::zero()
    }

    /// Returns true if vector is null
    #[inline]
    pub fn is_null(&self) -> bool {
        self.is_zero()
    }
}

impl<N: Copy + Num + ops::AddAssign, const D: usize> Vector<N, D> {
    /// Returns the squared norm of vector
    ///
    /// ## Example
    /// ```
    /// use pythagore::vector;
    ///
    /// assert_eq!(vector!{ dx: 2, dy: 3 }.square_norm(), 13);
    /// assert_eq!(vector!{ dx: 2, dy: 3, dz: 4 }.square_norm(), 29);
    /// ```
    pub fn square_norm(&self) -> N {
        let mut result = N::zero();

        for n in 0..D {
            result += self[n] * self[n];
        }

        result
    }
}

impl<N: Copy + Float + ops::AddAssign, const D: usize> Vector<N, D> {
    /// Returns the norm of vector (only for float vectors)
    ///
    /// ## Example
    /// ```
    /// use pythagore::vector;
    ///
    /// assert_eq!(vector!{ dx: 1.0, dy: 0.0 }.norm(), 1.0);
    /// assert_eq!(vector!{ dx: 0.0, dy: 0.0, dz: 4.0 }.norm(), 4.0);
    /// ```
    pub fn norm(&self) -> N {
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

impl<N: Copy + Signed + ops::AddAssign, const D: usize> Vector<N, D> {
    /// Returns the norm of vector (only for signed vectors)
    ///
    /// ## Example
    /// ```
    /// use pythagore::vector;
    ///
    /// assert_eq!(vector!{ dx: 1, dy: -2 }.manhattan_norm(), 3);
    /// assert_eq!(vector!{ dx: 1, dy: -2, dz: 3 }.manhattan_norm(), 6);
    /// ```
    pub fn manhattan_norm(&self) -> N {
        let mut result = N::zero();

        for n in 0..D {
            result += self[n].abs();
        }

        result
    }
}

// Utils
impl<N: Copy + Num, const D: usize> Dimension<D> for Vector<N, D> {
    /// Returns vector's dimension
    #[inline]
    fn dimension() -> usize {
        D - 1
    }
}

macro_rules! vector_from_array_impl {
    ($dim:literal) => {
        #[cfg(not(feature = "generic_const_exprs"))]
        impl<N: Copy + Num> From<&[N; $dim]> for Vector<N, { $dim + 1 }> {
            fn from(value: &[N; $dim]) -> Self {
                let mut scalar = Scalar::zero();

                for n in 0..$dim {
                    scalar[n] = value[n];
                }

                Vector { scalar }
            }
        }
    };
}

vector_from_array_impl!(2);
vector_from_array_impl!(3);

macro_rules! vector_from_scalar_impl {
    ($dim:literal) => {
        #[cfg(not(feature = "generic_const_exprs"))]
        impl<N: Copy + Num> From<&Scalar<N, $dim>> for Vector<N, { $dim + 1 }> {
            #[inline]
            fn from(value: &Scalar<N, $dim>) -> Self {
                Vector::from(&value.elements)
            }
        }
    };
}

vector_from_scalar_impl!(2);
vector_from_scalar_impl!(3);

#[cfg(feature = "generic_const_exprs")]
impl<N: Copy + Num, const D: usize> From<&Scalar<N, D>> for Vector<N, { D + 1 }> {
    #[inline]
    fn from(value: &Scalar<N, D>) -> Self {
        Point::from(&value.elements)
    }
}

impl<N: Copy + Num, const D: usize> Zero for Vector<N, D> {
    #[inline]
    fn zero() -> Self {
        Vector { scalar: Scalar::zero() }
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.scalar.is_zero()
    }
}

// Operators
impl<N: Num, const D: usize> PartialEq for Vector<N, D> {
    fn eq(&self, other: &Self) -> bool {
        self.scalar == other.scalar
    }
}

impl<N: Copy + Num, I: SliceIndex<[N]>, const D: usize> ops::Index<I> for Vector<N, D> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.scalar[index]
    }
}

impl<N: Copy + Num, I: SliceIndex<[N]>, const D: usize> ops::IndexMut<I> for Vector<N, D> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
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

vector_neg_impl!(N, D, Vector<N, D>);
vector_neg_impl!(N, D, &Vector<N, D>);

macro_rules! vector_add_assign_impl {
    ($tp:ident, $dp:ident, $rhs:ty) => {
        impl<$tp: Copy + Num + ops::AddAssign, const $dp: usize> ops::AddAssign<$rhs> for Vector<$tp, $dp> {
            fn add_assign(&mut self, rhs: $rhs) {
                self.scalar += rhs.scalar;
            }
        }
    }
}

vector_add_assign_impl!(N, D, Vector<N, D>);
vector_add_assign_impl!(N, D, &Vector<N, D>);

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

vector_add_impl!(N, D, Vector<N, D>, Vector<N, D>);
vector_add_impl!(N, D, &Vector<N, D>, Vector<N, D>);
vector_add_impl!(N, D, Vector<N, D>, &Vector<N, D>);
vector_add_impl!(N, D, &Vector<N, D>, &Vector<N, D>);

macro_rules! vector_sub_assign_impl {
    ($tp:ident, $dp:ident, $rhs:ty) => {
        impl<$tp: Copy + Num, const $dp: usize> ops::SubAssign<$rhs> for Vector<$tp, $dp> {
            fn sub_assign(&mut self, rhs: $rhs) {
                self.scalar -= rhs.scalar;
            }
        }
    }
}

vector_sub_assign_impl!(N, D, Vector<N, D>);
vector_sub_assign_impl!(N, D, &Vector<N, D>);

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

vector_sub_impl!(N, D, Vector<N, D>, Vector<N, D>);
vector_sub_impl!(N, D, &Vector<N, D>, Vector<N, D>);
vector_sub_impl!(N, D, Vector<N, D>, &Vector<N, D>);
vector_sub_impl!(N, D, &Vector<N, D>, &Vector<N, D>);

macro_rules! vector_mul_assign_impl {
    ($tp:ident, $dp:ident, $rhs:ty $(, $defer:tt)?) => {
        impl<$tp: Copy + Num, const $dp: usize> ops::MulAssign<$rhs> for Vector<$tp, $dp> {
            fn mul_assign(&mut self, rhs: $rhs) {
                self.scalar *= $($defer)?rhs;
            }
        }
    }
}

vector_mul_assign_impl!(N, D, N);
vector_mul_assign_impl!(N, D, &N, *);

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

vector_mul_impl!(N, D, Vector<N, D>, N);
vector_mul_impl!(N, D, &Vector<N, D>, N);
vector_mul_impl!(N, D, Vector<N, D>, &N, *);
vector_mul_impl!(N, D, &Vector<N, D>, &N, *);

macro_rules! vector_scalar_impl {
    ($tp:ident, $dp:ident, $lhs:ty, $rhs:ty) => {
        impl<$tp: Copy + Num + Sum, const $dp: usize> ops::Mul<$rhs> for $lhs {
            type Output = $tp;

            fn mul(self, rhs: $rhs) -> Self::Output {
                self.scalar * rhs.scalar
            }
        }
    }
}

vector_scalar_impl!(N, D, Vector<N, D>, Vector<N, D>);
vector_scalar_impl!(N, D, &Vector<N, D>, Vector<N, D>);
vector_scalar_impl!(N, D, Vector<N, D>, &Vector<N, D>);
vector_scalar_impl!(N, D, &Vector<N, D>, &Vector<N, D>);

macro_rules! vector_div_assign_impl {
    ($tp:ident, $dp:ident, $rhs:ty $(, $defer:tt)?) => {
        impl<$tp: Copy + Num, const $dp: usize> ops::DivAssign<$rhs> for Vector<$tp, $dp> {
            fn div_assign(&mut self, rhs: $rhs) {
                self.scalar /= $($defer)?rhs;
            }
        }
    }
}

vector_div_assign_impl!(N, D, N);
vector_div_assign_impl!(N, D, &N, *);

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

vector_div_impl!(N, D, Vector<N, D>, N);
vector_div_impl!(N, D, &Vector<N, D>, N);
vector_div_impl!(N, D, Vector<N, D>, &N, *);
vector_div_impl!(N, D, &Vector<N, D>, &N, *);

// Tests
#[cfg(test)]
mod tests {
    use crate::{scalar, vector};
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

    #[test]
    fn vector_from_array() {
        let v = Vector::from(&[1, 2, 3]);

        assert_eq!(v.scalar.elements, [1, 2, 3, 0]);
    }

    #[test]
    fn vector_from_scalar() {
        let v = Vector::from(&scalar![1, 2, 3]);

        assert_eq!(v.scalar.elements, [1, 2, 3, 0]);
    }

    #[test]
    fn vector_neg_operator() {
        let v = -vector!{ dx: 1, dy: 2 };

        assert_eq!(v, vector!{ dx: -1, dy: -2 });
        assert_eq!(v.scalar[2], 0);
    }

    #[test]
    fn vector_add_assign() {
        let mut v = vector!{ dx: 1, dy: 2 };
        v += vector!{ dx: 3, dy: 4 };

        assert_eq!(v, vector!{ dx: 4, dy: 6 });
        assert_eq!(v.scalar[2], 0);
    }

    #[test]
    fn vector_add_vector() {
        let v = vector!{ dx: 1, dy: 2 };
        let u = v + vector!{ dx: 3, dy: 4 };

        assert_eq!(u, vector!{ dx: 4, dy: 6 });
        assert_eq!(u.scalar[2], 0);
    }

    #[test]
    fn vector_sub_assign() {
        let mut v = vector!{ dx: 1, dy: 2 };
        v -= vector!{ dx: 3, dy: 4 };

        assert_eq!(v, vector!{ dx: -2, dy: -2 });
        assert_eq!(v.scalar[2], 0);
    }

    #[test]
    fn vector_sub_vector() {
        let v = vector!{ dx: 1, dy: 2 };
        let u = v - vector!{ dx: 3, dy: 4 };

        assert_eq!(u, vector!{ dx: -2, dy: -2 });
        assert_eq!(u.scalar[2], 0);
    }

    #[test]
    fn vector_mul_assign() {
        let mut v = vector!{ dx: 1, dy: 2 };
        v *= 2;

        assert_eq!(v, vector!{ dx: 2, dy: 4 });
        assert_eq!(v.scalar[2], 0);
    }

    #[test]
    fn vector_mul_num() {
        let v = vector!{ dx: 1, dy: 2 };
        let u = v * 2;

        assert_eq!(u, vector!{ dx: 2, dy: 4 });
        assert_eq!(u.scalar[2], 0);
    }

    #[test]
    fn vector_mul_vector() {
        let r = vector!{ dx: 1, dy: 2 } * vector!{ dx: 3, dy: 4 };

        assert_eq!(r, 11);
    }

    #[test]
    fn vector_div_assign() {
        let mut v = vector!{ dx: 2, dy: 4 };
        v /= 2;

        assert_eq!(v, vector!{ dx: 1, dy: 2 });
        assert_eq!(v.scalar[2], 0);
    }

    #[test]
    fn vector_div_num() {
        let v = vector!{ dx: 2, dy: 4 };
        let u = v / 2;

        assert_eq!(u, vector!{ dx: 1, dy: 2 });
        assert_eq!(u.scalar[2], 0);
    }
}