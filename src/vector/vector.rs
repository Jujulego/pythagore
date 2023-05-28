use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};
use std::slice::{Iter, IterMut, SliceIndex};
use num_traits::{Num, Signed, Zero};
use crate::{forward_ref_binop, forward_ref_op_assign, owned_binop, owned_op_assign, owned_unop};

use crate::traits::{Dimension, BoxableVector};

/// `Vector<N, D>` utility structure for D dimension compute
///
/// ## Usage
/// ```
/// use pythagore::vector;
///
/// let s = vector![1, 2, 3, 4];
///
/// assert_eq!(s[0], 1);
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash)]
pub struct Vector<N: Num, const D: usize> {
    pub(crate) elements: [N; D],
}

// Methods
impl<N: Num, const D: usize> Vector<N, D> {
    /// Returns iterator on vector elements
    pub fn iter(&self) -> Iter<N> {
        self.elements.iter()
    }

    /// Returns mutable iterator on vector elements
    pub fn iter_mut(&mut self) -> IterMut<N> {
        self.elements.iter_mut()
    }
}

// Utils
impl<N: Num, const D: usize> BoxableVector<N> for Vector<N, D> {}

impl<N: Num, const D: usize> Dimension<D> for Vector<N, D> {}

impl<N: Copy + Num, const D: usize> Default for Vector<N, D> {
    #[inline]
    fn default() -> Self {
        Vector { elements: [N::zero(); D] }
    }
}

impl<N: Copy + Num, const D: usize> Zero for Vector<N, D> {
    #[inline]
    fn zero() -> Self {
        Vector::from([N::zero(); D])
    }

    fn is_zero(&self) -> bool {
        self.elements.iter().all(|e| e.is_zero())
    }
}

impl<N: Num, const D: usize> From<[N; D]> for Vector<N, D> {
    /// Builds a new vector form given fixed array
    ///
    /// ## Example
    /// ```
    /// use pythagore::*;
    ///
    /// assert_eq!(Vector::from([1, 2, 3]), vector![1, 2, 3]);
    /// ```
    fn from(value: [N; D]) -> Self {
        Vector { elements: value }
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

impl<N: Copy + Num, const D: usize> FromIterator<N> for Vector<N, D> {
    fn from_iter<T: IntoIterator<Item = N>>(iter: T) -> Self {
        let mut vector = Vector::default();
        let mut idx = 0;

        for x in iter.into_iter().take(D) {
            vector[idx] = x;
            idx += 1;
        }

        vector
    }
}

// Operators
impl<N: Num, const D: usize> PartialEq for Vector<N, D> {
    fn eq(&self, other: &Self) -> bool {
        self.elements == other.elements
    }
}

impl<N: Num, I: SliceIndex<[N]>, const D: usize> Index<I> for Vector<N, D> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.elements[index]
    }
}

impl<N: Num, I: SliceIndex<[N]>, const D: usize> IndexMut<I> for Vector<N, D> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.elements[index]
    }
}

impl<N: Copy + Signed, const D: usize> Neg for &Vector<N, D> {
    type Output = Vector<N, D>;

    fn neg(self) -> Self::Output {
        self.iter().map(|&x| -x).collect()
    }
}

owned_unop!(Neg, Vector<N, D>, neg, <N: Copy + Signed, const D: usize>);

impl<N: Copy + Num + AddAssign, const D: usize> AddAssign<&Vector<N, D>> for Vector<N, D> {
    fn add_assign(&mut self, rhs: &Vector<N, D>) {
        self.iter_mut()
            .zip(rhs.iter())
            .for_each(|(l, &r)| *l += r);
    }
}

owned_op_assign!(AddAssign, Vector<N, D>, add_assign, Vector<N, D>, <N: Copy + Num + AddAssign, const D: usize>);

impl<N: Copy + Num, const D: usize> Add for &Vector<N, D> {
    type Output = Vector<N, D>;

    fn add(self, rhs: &Vector<N, D>) -> Self::Output {
        self.iter()
            .zip(rhs.iter())
            .map(|(&l, &r)| l + r)
            .collect()
    }
}

owned_binop!(Add, Vector<N, D>, add, Vector<N, D>, <N: Copy + Num, const D: usize>);

impl<N: Copy + Num, const D: usize> SubAssign<&Vector<N, D>> for Vector<N, D> {
    fn sub_assign(&mut self, rhs: &Vector<N, D>) {
        self.iter_mut()
            .zip(rhs.iter())
            .for_each(|(l, &r)| *l = *l - r);
    }
}

owned_op_assign!(SubAssign, Vector<N, D>, sub_assign, Vector<N, D>, <N: Copy + Num, const D: usize>);

impl<N: Copy + Num, const D: usize> Sub for &Vector<N, D> {
    type Output = Vector<N, D>;

    fn sub(self, rhs: &Vector<N, D>) -> Self::Output {
        self.iter()
            .zip(rhs.iter())
            .map(|(&l, &r)| l - r)
            .collect()
    }
}

owned_binop!(Sub, Vector<N, D>, sub, Vector<N, D>, <N: Copy + Num, const D: usize>);

impl<N: Copy + Num, const D: usize> MulAssign<N> for Vector<N, D> {
    fn mul_assign(&mut self, rhs: N) {
        self.iter_mut()
            .for_each(move |l| *l = *l * rhs);
    }
}

forward_ref_op_assign!(MulAssign, Vector<N, D>, mul_assign, N, <N: Copy + Num, const D: usize>);

impl<N: Copy + Num, const D: usize> Mul<N> for &Vector<N, D> {
    type Output = Vector<N, D>;

    fn mul(self, rhs: N) -> Self::Output {
        self.iter()
            .map(move |&l| l * rhs)
            .collect()
    }
}

forward_ref_binop!(Mul, Vector<N, D>, mul, N, <N: Copy + Num, const D: usize>);

impl<N: Copy + Num + Sum, const D: usize> Mul for &Vector<N, D> {
    type Output = N;

    fn mul(self, rhs: &Vector<N, D>) -> Self::Output {
        self.iter()
            .zip(rhs.iter())
            .map(|(&l, &r)| l * r)
            .sum()
    }
}

owned_binop!(Mul, Vector<N, D>, mul, Vector<N, D>, <N: Copy + Num + Sum, const D: usize>);

impl<N: Copy + Num, const D: usize> DivAssign<N> for Vector<N, D> {
    fn div_assign(&mut self, rhs: N) {
        self.iter_mut()
            .for_each(move |l| *l = *l / rhs);
    }
}

forward_ref_op_assign!(DivAssign, Vector<N, D>, div_assign, N, <N: Copy + Num, const D: usize>);

impl<N: Copy + Num, const D: usize> Div<N> for &Vector<N, D> {
    type Output = Vector<N, D>;

    fn div(self, rhs: N) -> Self::Output {
        self.iter()
            .map(move |&l| l / rhs)
            .collect()
    }
}

forward_ref_binop!(Div, Vector<N, D>, div, N, <N: Copy + Num, const D: usize>);
