use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};
use std::slice::{Iter, IterMut, SliceIndex};
use num_traits::{Float, Num, Signed, Zero};

use crate::{owned_binop, owned_op_assign, owned_unop, Vector};
use crate::traits::Dimension;

/// `Force<N, D>` structure for D dimension forces
#[derive(Clone, Copy, Debug, Eq)]
pub struct Force<N: Num, const D: usize> {
    pub(crate) vector: Vector<N, D>,
}

// Methods
impl<N: Copy + Num, const D: usize> Force<N, D> {
    /// Returns a null force
    ///
    /// ## Example
    /// ```
    /// use pythagore::{force, Force2D, Force3D};
    ///
    /// assert_eq!(Force2D::null(), force!{ dx: 0, dy: 0 });
    /// assert_eq!(Force3D::null(), force!{ dx: 0, dy: 0, dz: 0 });
    /// ```
    #[inline]
    pub fn null() -> Self {
        Force { vector: Vector::zero() }
    }

    /// Returns true if force is null
    #[inline]
    pub fn is_null(&self) -> bool {
        self.vector.is_zero()
    }
}

impl<N: Num, const D: usize> Force<N, D> {
    /// Returns iterator on force elements
    pub fn iter(&self) -> Iter<'_, N> {
        self.vector[..D-1].iter()
    }

    /// Returns iterator on force elements
    pub fn iter_mut(&mut self) -> IterMut<'_, N> {
        self.vector[..D-1].iter_mut()
    }
}

impl<N: Copy + Num + Sum, const D: usize> Force<N, D> {
    /// Returns the squared norm of force
    ///
    /// ## Example
    /// ```
    /// use pythagore::force;
    ///
    /// assert_eq!(force!{ dx: 2, dy: 3 }.square_norm(), 13);
    /// assert_eq!(force!{ dx: 2, dy: 3, dz: 4 }.square_norm(), 29);
    /// ```
    pub fn square_norm(&self) -> N {
        self.iter()
            .map(|&x| x * x)
            .sum()
    }
}

impl<N: Copy + Float + Sum, const D: usize> Force<N, D> {
    /// Returns the norm of force (only for float forces)
    ///
    /// ## Example
    /// ```
    /// use pythagore::force;
    ///
    /// assert_eq!(force!{ dx: 1.0, dy: 0.0 }.norm(), 1.0);
    /// assert_eq!(force!{ dx: 0.0, dy: 0.0, dz: 4.0 }.norm(), 4.0);
    /// ```
    pub fn norm(&self) -> N {
        self.square_norm().sqrt()
    }

    /// Returns a unit force from force (only for float forces)
    ///
    /// ## Example
    /// ```
    /// use pythagore::force;
    ///
    /// assert_eq!(force!{ dx: 10.0, dy: 0.0 }.unit(), force!{ dx: 1.0, dy: 0.0 });
    /// assert_eq!(force!{ dx: 0.0, dy: 0.0, dz: 5.0 }.unit(), force!{ dx: 0.0, dy: 0.0, dz: 1.0 });
    /// ```
    pub fn unit(&self) -> Self {
        self / self.norm()
    }
}

impl<N: Copy + Signed + Sum, const D: usize> Force<N, D> {
    /// Returns the norm of force (only for signed forces)
    ///
    /// ## Example
    /// ```
    /// use pythagore::force;
    ///
    /// assert_eq!(force!{ dx: 1, dy: -2 }.manhattan_norm(), 3);
    /// assert_eq!(force!{ dx: 1, dy: -2, dz: 3 }.manhattan_norm(), 6);
    /// ```
    pub fn manhattan_norm(&self) -> N {
        self.iter()
            .map(|x| x.abs())
            .sum()
    }
}

// Utils
impl<N: Num, const D: usize> Dimension<D> for Force<N, D> {
    /// Returns force's dimension
    #[inline]
    fn dimension() -> usize {
        D - 1
    }
}

impl<N: Copy + Num, const D: usize> Default for Force<N, D> {
    #[inline]
    fn default() -> Self {
        Force::null()
    }
}

impl<N: Copy + Num, const D: usize> Zero for Force<N, D> {
    #[inline]
    fn zero() -> Self {
        Force::null()
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.is_null()
    }
}

impl<'a, N: Num, const D: usize> IntoIterator for &'a Force<N, D> {
    type Item = &'a N;
    type IntoIter = Iter<'a, N>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, N: Num, const D: usize> IntoIterator for &'a mut Force<N, D> {
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
        impl<N: Copy + Num> From<&[N; $dim]> for Force<N, { $dim + 1 }> {
            fn from(value: &[N; $dim]) -> Self {
                let mut vector = Vector::zero();

                for n in 0..$dim {
                    vector[n] = value[n];
                }

                Force { vector }
            }
        }
    };
}

from_array_impl!(2);
from_array_impl!(3);

#[cfg(feature = "generic_const_exprs")]
impl<N: Copy + Num, const D: usize> From<&[N; D]> for Force<N, { D + 1 }> {
    fn from(value: &[N; D]) -> Self {
        let mut vector = Vector::zero();

        for n in 0..D {
            vector[n] = value[n];
        }

        vector[D] = N::one();

        Force { vector }
    }
}

macro_rules! from_vector_impl {
    ($dim:literal) => {
        #[cfg(not(feature = "generic_const_exprs"))]
        impl<N: Copy + Num> From<&Vector<N, $dim>> for Force<N, { $dim + 1 }> {
            #[inline]
            fn from(value: &Vector<N, $dim>) -> Self {
                Force::from(&value.elements)
            }
        }
    };
}

from_vector_impl!(2);
from_vector_impl!(3);

#[cfg(feature = "generic_const_exprs")]
impl<N: Copy + Num, const D: usize> From<&Vector<N, D>> for Force<N, { D + 1 }> {
    #[inline]
    fn from(value: &Vector<N, D>) -> Self {
        Point::from(&value.elements)
    }
}

impl<N: Copy + Num, const D: usize> FromIterator<N> for Force<N, D> {
    fn from_iter<T: IntoIterator<Item = N>>(iter: T) -> Self {
        let mut force = Force::default();
        let mut idx = 0;

        for x in iter.into_iter().take(D - 1) {
            force[idx] = x;
            idx += 1;
        }

        force
    }
}

// Operators
impl<N: Num, const D: usize> PartialEq for Force<N, D> {
    fn eq(&self, other: &Self) -> bool {
        self.vector == other.vector
    }
}

impl<N: Copy + Num, I: SliceIndex<[N]>, const D: usize> Index<I> for Force<N, D> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.vector[index]
    }
}

impl<N: Copy + Num, I: SliceIndex<[N]>, const D: usize> IndexMut<I> for Force<N, D> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.vector[index]
    }
}

impl<N: Copy + Signed, const D: usize> Neg for &Force<N, D> {
    type Output = Force<N, D>;

    fn neg(self) -> Self::Output {
        Force { vector: -self.vector }
    }
}

owned_unop!(Neg, Force<N, D>, neg, <N: Copy + Signed, const D: usize>);

impl<N: Copy + Num + AddAssign, const D: usize> AddAssign<&Force<N, D>> for Force<N, D> {
    fn add_assign(&mut self, rhs: &Force<N, D>) {
        self.vector += &rhs.vector;
    }
}

owned_op_assign!(AddAssign, Force<N, D>, add_assign, Force<N, D>, <N: Copy + Num + AddAssign, const D: usize>);

impl<N: Copy + Num, const D: usize> Add for &Force<N, D> {
    type Output = Force<N, D>;

    fn add(self, rhs: &Force<N, D>) -> Self::Output {
        Force { vector: &self.vector + &rhs.vector }
    }
}

owned_binop!(Add, Force<N, D>, add, Force<N, D>, <N: Copy + Num, const D: usize>);

impl<N: Copy + Num + SubAssign, const D: usize> SubAssign<&Force<N, D>> for Force<N, D> {
    fn sub_assign(&mut self, rhs: &Force<N, D>) {
        self.vector -= &rhs.vector;
    }
}

owned_op_assign!(SubAssign, Force<N, D>, sub_assign, Force<N, D>, <N: Copy + Num + SubAssign, const D: usize>);

impl<N: Copy + Num, const D: usize> Sub for &Force<N, D> {
    type Output = Force<N, D>;

    fn sub(self, rhs: &Force<N, D>) -> Self::Output {
        Force { vector: &self.vector - &rhs.vector }
    }
}

owned_binop!(Sub, Force<N, D>, sub, Force<N, D>, <N: Copy + Num, const D: usize>);

impl<N: Copy + Num + MulAssign, const D: usize> MulAssign<&N> for Force<N, D> {
    fn mul_assign(&mut self, rhs: &N) {
        self.vector *= rhs;
    }
}

owned_op_assign!(MulAssign, Force<N, D>, mul_assign, N, <N: Copy + Num + MulAssign, const D: usize>);

impl<N: Copy + Num, const D: usize> Mul<&N> for &Force<N, D> {
    type Output = Force<N, D>;

    fn mul(self, rhs: &N) -> Self::Output {
        Force { vector: &self.vector * rhs }
    }
}

owned_binop!(Mul, Force<N, D>, mul, N, <N: Copy + Num, const D: usize>);

impl<N: Copy + Num + Sum, const D: usize> Mul for &Force<N, D> {
    type Output = N;

    fn mul(self, rhs: &Force<N, D>) -> Self::Output {
        &self.vector * &rhs.vector
    }
}

owned_binop!(Mul, Force<N, D>, mul, Force<N, D>, <N: Copy + Num + Sum, const D: usize>);

impl<N: Copy + Num + DivAssign, const D: usize> DivAssign<&N> for Force<N, D> {
    fn div_assign(&mut self, rhs: &N) {
        self.vector /= rhs;
    }
}

owned_op_assign!(DivAssign, Force<N, D>, div_assign, N, <N: Copy + Num + DivAssign, const D: usize>);

impl<N: Copy + Num, const D: usize> Div<&N> for &Force<N, D> {
    type Output = Force<N, D>;

    fn div(self, rhs: &N) -> Self::Output {
        Force { vector: &self.vector / rhs }
    }
}

owned_binop!(Div, Force<N, D>, div, N, <N: Copy + Num, const D: usize>);

// Tests
#[cfg(test)]
mod tests {
    use crate::{vector, force};
    use super::*;

    #[test]
    fn force_is_null() {
        assert!(Force::<f32, 4>::null().is_null());

        assert!(!force!{ dx: 1, dy: 2 }.is_null());
        assert!(!force!{ dx: 1, dy: 2, dz: 3 }.is_null());
    }

    #[test]
    fn force_2d_norm() {
        assert_eq!(force!{ dx: 2.0, dy: 3.0 }.norm(), (13.0 as f32).sqrt());
    }

    #[test]
    fn force_3d_norm() {
        assert_eq!(force!{ dx: 2.0, dy: 3.0, dz: 4.0 }.norm(), (29.0 as f32).sqrt());
    }

    #[test]
    fn force_2d_unit() {
        let v = force!{ dx: 2.0, dy: 3.0 };
        let norm = v.norm();

        assert_eq!(v.unit(), force!{ dx: v.dx() / norm, dy: v.dy() / norm });

        assert_eq!(v.unit().norm(), 1.0);
        assert_eq!(v.unit().vector[2], 0.0);
    }

    #[test]
    fn force_3d_unit() {
        let v = force!{ dx: 2.0, dy: 3.0, dz: 4.0 };
        let norm = v.norm();

        assert_eq!(v.unit(), force!{ dx: v.dx() / norm, dy: v.dy() / norm, dz: v.dz() / norm });

        assert_eq!(v.unit().norm(), 1.0);
        assert_eq!(v.unit().vector[3], 0.0);
    }

    #[test]
    fn force_from_array() {
        let v = Force::from(&[1, 2, 3]);

        assert_eq!(v.vector.elements, [1, 2, 3, 0]);
    }

    #[test]
    fn force_from_vector() {
        let v = Force::from(&vector![1, 2, 3]);

        assert_eq!(v.vector.elements, [1, 2, 3, 0]);
    }

    #[test]
    fn force_neg_operator() {
        let v = -force!{ dx: 1, dy: 2 };

        assert_eq!(v, force!{ dx: -1, dy: -2 });
        assert_eq!(v.vector[2], 0);
    }

    #[test]
    fn force_add_assign() {
        let mut v = force!{ dx: 1, dy: 2 };
        v += force!{ dx: 3, dy: 4 };

        assert_eq!(v, force!{ dx: 4, dy: 6 });
        assert_eq!(v.vector[2], 0);
    }

    #[test]
    fn force_add_force() {
        let v = force!{ dx: 1, dy: 2 };
        let u = v + force!{ dx: 3, dy: 4 };

        assert_eq!(u, force!{ dx: 4, dy: 6 });
        assert_eq!(u.vector[2], 0);
    }

    #[test]
    fn force_sub_assign() {
        let mut v = force!{ dx: 1, dy: 2 };
        v -= force!{ dx: 3, dy: 4 };

        assert_eq!(v, force!{ dx: -2, dy: -2 });
        assert_eq!(v.vector[2], 0);
    }

    #[test]
    fn force_sub_force() {
        let v = force!{ dx: 1, dy: 2 };
        let u = v - force!{ dx: 3, dy: 4 };

        assert_eq!(u, force!{ dx: -2, dy: -2 });
        assert_eq!(u.vector[2], 0);
    }

    #[test]
    fn force_mul_assign() {
        let mut v = force!{ dx: 1, dy: 2 };
        v *= 2;

        assert_eq!(v, force!{ dx: 2, dy: 4 });
        assert_eq!(v.vector[2], 0);
    }

    #[test]
    fn force_mul_num() {
        let v = force!{ dx: 1, dy: 2 };
        let u = v * 2;

        assert_eq!(u, force!{ dx: 2, dy: 4 });
        assert_eq!(u.vector[2], 0);
    }

    #[test]
    fn force_mul_force() {
        let r = force!{ dx: 1, dy: 2 } * force!{ dx: 3, dy: 4 };

        assert_eq!(r, 11);
    }

    #[test]
    fn force_div_assign() {
        let mut v = force!{ dx: 2, dy: 4 };
        v /= 2;

        assert_eq!(v, force!{ dx: 1, dy: 2 });
        assert_eq!(v.vector[2], 0);
    }

    #[test]
    fn force_div_num() {
        let v = force!{ dx: 2, dy: 4 };
        let u = v / 2;

        assert_eq!(u, force!{ dx: 1, dy: 2 });
        assert_eq!(u.vector[2], 0);
    }
}