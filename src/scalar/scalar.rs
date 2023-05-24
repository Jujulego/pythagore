use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};
use std::slice::{Iter, IterMut, SliceIndex};
use num_traits::{Num, Signed, Zero};
use crate::{forward_ref_binop, forward_ref_op_assign, forward_ref_unop};

use crate::traits::{Dimension, BoxableScalar};

/// `Scalar<N, const D: usize>` utility structure for n dimension compute
///
/// ## Usage
/// ```
/// use pythagore::scalar;
///
/// let s = scalar![1, 2, 3, 4];
///
/// assert_eq!(s[0], 1);
/// ```
#[derive(Clone, Copy, Debug, Eq)]
pub struct Scalar<N: Num, const D: usize> {
    pub(crate) elements: [N; D],
}

// Methods
impl<N: Num, const D: usize> Scalar<N, D> {
    /// Returns iterator on scalar elements
    pub fn iter(&self) -> Iter<N> {
        self.elements.iter()
    }

    /// Returns mutable iterator on scalar elements
    pub fn iter_mut(&mut self) -> IterMut<N> {
        self.elements.iter_mut()
    }
}

// Utils
impl<N: Num, const D: usize> BoxableScalar<N> for Scalar<N, D> {}

impl<N: Num, const D: usize> Dimension<D> for Scalar<N, D> {}

impl<N: Copy + Num, const D: usize> Default for Scalar<N, D> {
    #[inline]
    fn default() -> Self {
        Scalar { elements: [N::zero(); D] }
    }
}

impl<N: Copy + Num, const D: usize> Zero for Scalar<N, D> {
    #[inline]
    fn zero() -> Self {
        Scalar::from([N::zero(); D])
    }

    fn is_zero(&self) -> bool {
        self.elements.iter().all(|e| e.is_zero())
    }
}

impl<N: Num, const D: usize> From<[N; D]> for Scalar<N, D> {
    /// Builds a new scalar form given fixed array
    ///
    /// ## Example
    /// ```
    /// use pythagore::*;
    ///
    /// assert_eq!(Scalar::from([1, 2, 3]), scalar![1, 2, 3]);
    /// ```
    fn from(value: [N; D]) -> Self {
        Scalar { elements: value }
    }
}

impl<'a, N: Num, const D: usize> IntoIterator for &'a Scalar<N, D> {
    type Item = &'a N;
    type IntoIter = Iter<'a, N>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, N: Num, const D: usize> IntoIterator for &'a mut Scalar<N, D> {
    type Item = &'a mut N;
    type IntoIter = IterMut<'a, N>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<N: Copy + Num, const D: usize> FromIterator<N> for Scalar<N, D> {
    fn from_iter<T: IntoIterator<Item = N>>(iter: T) -> Self {
        let mut scalar = Scalar::default();
        let mut idx = 0;

        for x in iter.into_iter().take(D) {
            scalar[idx] = x;
            idx += 1;
        }

        scalar
    }
}

// Operators
impl<N: Num, const D: usize> PartialEq for Scalar<N, D> {
    fn eq(&self, other: &Self) -> bool {
        self.elements == other.elements
    }
}

impl<N: Num, I: SliceIndex<[N]>, const D: usize> Index<I> for Scalar<N, D> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.elements[index]
    }
}

impl<N: Num, I: SliceIndex<[N]>, const D: usize> IndexMut<I> for Scalar<N, D> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.elements[index]
    }
}

impl<N: Copy + Signed, const D: usize> Neg for Scalar<N, D> {
    type Output = Scalar<N, D>;

    fn neg(self) -> Self::Output {
        self.iter().map(|&x| -x).collect()
    }
}

forward_ref_unop!(Neg, Scalar<N, D>, neg, <N: Copy + Signed, const D: usize>);

impl<N: Copy + Num + AddAssign, const D: usize> AddAssign for Scalar<N, D> {
    fn add_assign(&mut self, rhs: Scalar<N, D>) {
        self.iter_mut()
            .zip(rhs.iter())
            .for_each(|(l, &r)| *l += r);
    }
}

forward_ref_op_assign!(AddAssign, Scalar<N, D>, add_assign, Scalar<N, D>, <N: Copy + Num + AddAssign, const D: usize>);

impl<N: Copy + Num, const D: usize> Add for Scalar<N, D> {
    type Output = Scalar<N, D>;

    fn add(self, rhs: Scalar<N, D>) -> Self::Output {
        self.iter()
            .zip(rhs.iter())
            .map(|(&l, &r)| l + r)
            .collect()
    }
}

forward_ref_binop!(Add, Scalar<N, D>, add, Scalar<N, D>, <N: Copy + Num, const D: usize>);

impl<N: Copy + Num, const D: usize> SubAssign for Scalar<N, D> {
    fn sub_assign(&mut self, rhs: Scalar<N, D>) {
        self.iter_mut()
            .zip(rhs.iter())
            .for_each(|(l, &r)| *l = *l - r);
    }
}

forward_ref_op_assign!(SubAssign, Scalar<N, D>, sub_assign, Scalar<N, D>, <N: Copy + Num, const D: usize>);

impl<N: Copy + Num, const D: usize> Sub for Scalar<N, D> {
    type Output = Scalar<N, D>;

    fn sub(self, rhs: Scalar<N, D>) -> Self::Output {
        self.iter()
            .zip(rhs.iter())
            .map(|(&l, &r)| l - r)
            .collect()
    }
}

forward_ref_binop!(Sub, Scalar<N, D>, sub, Scalar<N, D>, <N: Copy + Num, const D: usize>);

impl<N: Copy + Num, const D: usize> MulAssign<N> for Scalar<N, D> {
    fn mul_assign(&mut self, rhs: N) {
        self.iter_mut()
            .for_each(move |l| *l = *l * rhs);
    }
}

forward_ref_op_assign!(MulAssign, Scalar<N, D>, mul_assign, N, <N: Copy + Num, const D: usize>);

impl<N: Copy + Num, const D: usize> Mul<N> for Scalar<N, D> {
    type Output = Scalar<N, D>;

    fn mul(self, rhs: N) -> Self::Output {
        self.iter()
            .map(move |&l| l * rhs)
            .collect()
    }
}

forward_ref_binop!(Mul, Scalar<N, D>, mul, N, <N: Copy + Num, const D: usize>);

impl<N: Copy + Num + Sum, const D: usize> Mul for Scalar<N, D> {
    type Output = N;

    fn mul(self, rhs: Scalar<N, D>) -> Self::Output {
        self.iter()
            .zip(rhs.iter())
            .map(|(&l, &r)| l * r)
            .sum()
    }
}

forward_ref_binop!(Mul, Scalar<N, D>, mul, Scalar<N, D>, <N: Copy + Num + Sum, const D: usize>);

impl<N: Copy + Num, const D: usize> DivAssign<N> for Scalar<N, D> {
    fn div_assign(&mut self, rhs: N) {
        self.iter_mut()
            .for_each(move |l| *l = *l / rhs);
    }
}

forward_ref_op_assign!(DivAssign, Scalar<N, D>, div_assign, N, <N: Copy + Num, const D: usize>);

impl<N: Copy + Num, const D: usize> Div<N> for Scalar<N, D> {
    type Output = Scalar<N, D>;

    fn div(self, rhs: N) -> Self::Output {
        self.iter()
            .map(move |&l| l / rhs)
            .collect()
    }
}

forward_ref_binop!(Div, Scalar<N, D>, div, N, <N: Copy + Num, const D: usize>);
