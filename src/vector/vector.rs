use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};
use std::slice::{Iter, IterMut, SliceIndex};
use num_traits::{Float, Num, Signed, Zero};

use crate::{forward_ref_binop, forward_ref_op_assign, forward_ref_unop, Scalar};
use crate::traits::Dimension;

/// `Vector<N, const D: usize>` structure for n dimension vectors
#[derive(Clone, Copy, Debug, Eq)]
pub struct Vector<N: Num, const D: usize> {
    pub(crate) scalar: Scalar<N, D>,
}

// Methods
impl<N: Copy + Num, const D: usize> Vector<N, D> {
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
        Vector { scalar: Scalar::zero() }
    }

    /// Returns true if vector is null
    #[inline]
    pub fn is_null(&self) -> bool {
        self.scalar.is_zero()
    }
}

impl<N: Num, const D: usize> Vector<N, D> {
    /// Returns iterator on vector elements
    pub fn iter(&self) -> Iter<'_, N> {
        self.scalar[..D-1].iter()
    }

    /// Returns iterator on vector elements
    pub fn iter_mut(&mut self) -> IterMut<'_, N> {
        self.scalar[..D-1].iter_mut()
    }
}

impl<N: Copy + Num + Sum, const D: usize> Vector<N, D> {
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
        self.iter()
            .map(|&x| x * x)
            .sum()
    }
}

impl<N: Copy + Float + Sum, const D: usize> Vector<N, D> {
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

impl<N: Copy + Signed + Sum, const D: usize> Vector<N, D> {
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
        self.iter()
            .map(|x| x.abs())
            .sum()
    }
}

// Utils
impl<N: Num, const D: usize> Dimension<D> for Vector<N, D> {
    /// Returns vector's dimension
    #[inline]
    fn dimension() -> usize {
        D - 1
    }
}

impl<N: Copy + Num, const D: usize> Default for Vector<N, D> {
    #[inline]
    fn default() -> Self {
        Vector::null()
    }
}

impl<N: Copy + Num, const D: usize> Zero for Vector<N, D> {
    #[inline]
    fn zero() -> Self {
        Vector::null()
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.is_null()
    }
}

impl<'a, N: Num, const D: usize> IntoIterator for &'a Vector<N, D> {
    type Item = &'a N;
    type IntoIter = Iter<'a, N>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, N: Num, const D: usize> IntoIterator for &'a mut Vector<N, D> {
    type Item = &'a mut N;
    type IntoIter = IterMut<'a, N>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

macro_rules! from_array_impl {
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

from_array_impl!(2);
from_array_impl!(3);

#[cfg(feature = "generic_const_exprs")]
impl<N: Copy + Num, const D: usize> From<&[N; D]> for Vector<N, { D + 1 }> {
    fn from(value: &[N; D]) -> Self {
        let mut scalar = Scalar::zero();

        for n in 0..D {
            scalar[n] = value[n];
        }

        scalar[D] = N::one();

        Vector { scalar }
    }
}

macro_rules! from_scalar_impl {
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

from_scalar_impl!(2);
from_scalar_impl!(3);

#[cfg(feature = "generic_const_exprs")]
impl<N: Copy + Num, const D: usize> From<&Scalar<N, D>> for Vector<N, { D + 1 }> {
    #[inline]
    fn from(value: &Scalar<N, D>) -> Self {
        Point::from(&value.elements)
    }
}

impl<N: Copy + Num, const D: usize> FromIterator<N> for Vector<N, D> {
    fn from_iter<T: IntoIterator<Item = N>>(iter: T) -> Self {
        let mut vector = Vector::default();
        let mut idx = 0;

        for x in iter.into_iter().take(D - 1) {
            vector[idx] = x;
            idx += 1;
        }

        vector
    }
}

// Operators
impl<N: Num, const D: usize> PartialEq for Vector<N, D> {
    fn eq(&self, other: &Self) -> bool {
        self.scalar == other.scalar
    }
}

impl<N: Copy + Num, I: SliceIndex<[N]>, const D: usize> Index<I> for Vector<N, D> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.scalar[index]
    }
}

impl<N: Copy + Num, I: SliceIndex<[N]>, const D: usize> IndexMut<I> for Vector<N, D> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.scalar[index]
    }
}

impl<N: Copy + Signed, const D: usize> Neg for Vector<N, D> {
    type Output = Vector<N, D>;

    fn neg(self) -> Self::Output {
        Vector { scalar: -self.scalar }
    }
}

forward_ref_unop!(Neg, Vector<N, D>, neg, <N: Copy + Signed, const D: usize>);

impl<N: Copy + Num + AddAssign, const D: usize> AddAssign<Vector<N, D>> for Vector<N, D> {
    fn add_assign(&mut self, rhs: Vector<N, D>) {
        self.scalar += rhs.scalar;
    }
}

forward_ref_op_assign!(AddAssign, Vector<N, D>, add_assign, Vector<N, D>, <N: Copy + Num + AddAssign, const D: usize>);

impl<N: Copy + Num, const D: usize> Add for Vector<N, D> {
    type Output = Vector<N, D>;

    fn add(self, rhs: Vector<N, D>) -> Self::Output {
        Vector { scalar: self.scalar + rhs.scalar }
    }
}

forward_ref_binop!(Add, Vector<N, D>, add, Vector<N, D>, <N: Copy + Num, const D: usize>);

impl<N: Copy + Num + SubAssign, const D: usize> SubAssign<Vector<N, D>> for Vector<N, D> {
    fn sub_assign(&mut self, rhs: Vector<N, D>) {
        self.scalar -= rhs.scalar;
    }
}

forward_ref_op_assign!(SubAssign, Vector<N, D>, sub_assign, Vector<N, D>, <N: Copy + Num + SubAssign, const D: usize>);

impl<N: Copy + Num, const D: usize> Sub for Vector<N, D> {
    type Output = Vector<N, D>;

    fn sub(self, rhs: Vector<N, D>) -> Self::Output {
        Vector { scalar: self.scalar - rhs.scalar }
    }
}

forward_ref_binop!(Sub, Vector<N, D>, sub, Vector<N, D>, <N: Copy + Num, const D: usize>);

impl<N: Copy + Num + MulAssign, const D: usize> MulAssign<N> for Vector<N, D> {
    fn mul_assign(&mut self, rhs: N) {
        self.scalar *= rhs;
    }
}

forward_ref_op_assign!(MulAssign, Vector<N, D>, mul_assign, N, <N: Copy + Num + MulAssign, const D: usize>);

impl<N: Copy + Num, const D: usize> Mul<N> for Vector<N, D> {
    type Output = Vector<N, D>;

    fn mul(self, rhs: N) -> Self::Output {
        Vector { scalar: self.scalar * rhs }
    }
}

forward_ref_binop!(Mul, Vector<N, D>, mul, N, <N: Copy + Num, const D: usize>);

impl<N: Copy + Num + Sum, const D: usize> Mul for Vector<N, D> {
    type Output = N;

    fn mul(self, rhs: Vector<N, D>) -> Self::Output {
        self.scalar * rhs.scalar
    }
}

forward_ref_binop!(Mul, Vector<N, D>, mul, Vector<N, D>, <N: Copy + Num + Sum, const D: usize>);

impl<N: Copy + Num + DivAssign, const D: usize> DivAssign<N> for Vector<N, D> {
    fn div_assign(&mut self, rhs: N) {
        self.scalar /= rhs;
    }
}

forward_ref_op_assign!(DivAssign, Vector<N, D>, div_assign, N, <N: Copy + Num + DivAssign, const D: usize>);

impl<N: Copy + Num, const D: usize> Div<N> for Vector<N, D> {
    type Output = Vector<N, D>;

    fn div(self, rhs: N) -> Self::Output {
        Vector { scalar: self.scalar / rhs }
    }
}

forward_ref_binop!(Div, Vector<N, D>, div, N, <N: Copy + Num, const D: usize>);

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