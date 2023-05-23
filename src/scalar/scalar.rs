use std::ops;
use std::slice::{Iter, SliceIndex};
use num_traits::{Num, Signed, Zero};

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
impl<N: Copy + Num, const D: usize> Scalar<N, D> {
    /// Returns iterator on point elements
    #[inline]
    pub fn iter(&self) -> Iter<'_, N> {
        self.elements.iter()
    }

    #[inline]
    fn map(&self, op: impl Fn(&N, usize) -> N) -> Self {
        let mut copy = self.clone();
        copy.map_mut(op);

        copy
    }

    #[inline]
    fn map_mut(&mut self, op: impl Fn(&N, usize) -> N) {
        for n in 0..D {
            self[n] = op(&self[n], n);
        }
    }
}

// Utils
impl<N: Copy + Num, const D: usize> BoxableScalar<N> for Scalar<N, D> {}

impl<N: Copy + Num, const D: usize> Default for Scalar<N, D> {
    #[inline]
    fn default() -> Self {
        Scalar { elements: [N::zero(); D] }
    }
}

impl<N: Num, const D: usize> Dimension<D> for Scalar<N, D> {}

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

impl<N: Copy + Num, const D: usize> Zero for Scalar<N, D> {
    #[inline]
    fn zero() -> Self {
        Scalar::from([N::zero(); D])
    }

    fn is_zero(&self) -> bool {
        self.elements.iter().all(|e| e.is_zero())
    }
}

// Operators
impl<N: Num, const D: usize> PartialEq for Scalar<N, D> {
    fn eq(&self, other: &Self) -> bool {
        self.elements == other.elements
    }
}

impl<N: Num, I: SliceIndex<[N]>, const D: usize> ops::Index<I> for Scalar<N, D> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.elements[index]
    }
}

impl<N: Num, I: SliceIndex<[N]>, const D: usize> ops::IndexMut<I> for Scalar<N, D> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.elements[index]
    }
}

macro_rules! scalar_neg_impl {
    ($tp:ident, $dp:ident, $lhs:ty) => {
        impl<$tp: Copy + Signed, const $dp: usize> ops::Neg for $lhs {
            type Output = Scalar<$tp, $dp>;

            fn neg(self) -> Self::Output {
                self.map(|&x, _| -x)
            }
        }
    };
}

scalar_neg_impl!(N, D, Scalar<N, D>);
scalar_neg_impl!(N, D, &Scalar<N, D>);

macro_rules! scalar_add_assign_impl {
    ($tp:ident, $dp:ident, $rhs:ty) => {
        impl<$tp: Copy + Num, const $dp: usize> ops::AddAssign<$rhs> for Scalar<$tp, $dp> {
            fn add_assign(&mut self, rhs: $rhs) {
                self.map_mut(|&x, n| x + rhs[n]);
            }
        }
    };
}

scalar_add_assign_impl!(N, D, Scalar<N, D>);
scalar_add_assign_impl!(N, D, &Scalar<N, D>);

macro_rules! scalar_add_impl {
    ($tp:ident, $dp:ident, $lhs:ty, $rhs:ty) => {
        impl<$tp: Copy + Num, const $dp: usize> ops::Add<$rhs> for $lhs {
            type Output = Scalar<$tp, $dp>;

            fn add(self, rhs: $rhs) -> Self::Output {
                self.map(|&x, n| x + rhs[n])
            }
        }
    }
}

scalar_add_impl!(N, D, Scalar<N, D>, Scalar<N, D>);
scalar_add_impl!(N, D, &Scalar<N, D>, Scalar<N, D>);
scalar_add_impl!(N, D, Scalar<N, D>, &Scalar<N, D>);
scalar_add_impl!(N, D, &Scalar<N, D>, &Scalar<N, D>);

macro_rules! scalar_sub_assign_impl {
    ($tp:ident, $dp:ident, $rhs:ty) => {
        impl<$tp: Copy + Num, const $dp: usize> ops::SubAssign<$rhs> for Scalar<$tp, $dp> {
            fn sub_assign(&mut self, rhs: $rhs) {
                self.map_mut(|&x, n| x - rhs[n]);
            }
        }
    };
}

scalar_sub_assign_impl!(N, D, Scalar<N, D>);
scalar_sub_assign_impl!(N, D, &Scalar<N, D>);

macro_rules! scalar_sub_impl {
    ($tp:ident, $dp:ident, $lhs:ty, $rhs:ty) => {
        impl<$tp: Copy + Num, const $dp: usize> ops::Sub<$rhs> for $lhs {
            type Output = Scalar<$tp, $dp>;

            fn sub(self, rhs: $rhs) -> Self::Output {
                self.map(|&x, n| x - rhs[n])
            }
        }
    }
}

scalar_sub_impl!(N, D, Scalar<N, D>, Scalar<N, D>);
scalar_sub_impl!(N, D, &Scalar<N, D>, Scalar<N, D>);
scalar_sub_impl!(N, D, Scalar<N, D>, &Scalar<N, D>);
scalar_sub_impl!(N, D, &Scalar<N, D>, &Scalar<N, D>);

macro_rules! scalar_mul_assign_impl {
    ($tp:ident, $dp:ident, $rhs:ty $(, $defer:tt)?) => {
        impl<$tp: Copy + Num, const $dp: usize> ops::MulAssign<$rhs> for Scalar<$tp, $dp> {
            fn mul_assign(&mut self, rhs: $rhs) {
                self.map_mut(|&x, _| x * $($defer)?rhs);
            }
        }
    };
}

scalar_mul_assign_impl!(N, D, N);
scalar_mul_assign_impl!(N, D, &N, *);

macro_rules! scalar_mul_impl {
    ($tp:ident, $dp:ident, $lhs:ty, $rhs:ty $(, $defer:tt)?) => {
        impl<$tp: Copy + Num, const $dp: usize> ops::Mul<$rhs> for $lhs {
            type Output = Scalar<$tp, $dp>;

            fn mul(self, rhs: $rhs) -> Self::Output {
                self.map(|&x, _| x * $($defer)?rhs)
            }
        }
    }
}

scalar_mul_impl!(N, D, Scalar<N, D>, N);
scalar_mul_impl!(N, D, &Scalar<N, D>, N);
scalar_mul_impl!(N, D, Scalar<N, D>, &N, *);
scalar_mul_impl!(N, D, &Scalar<N, D>, &N, *);

macro_rules! scalar_dot_scalar_impl {
    ($tp:ident, $dp:ident, $lhs:ty, $rhs:ty) => {
        impl<$tp: Copy + Num + ops::AddAssign, const $dp: usize> ops::Mul<$rhs> for $lhs {
            type Output = $tp;

            fn mul(self, rhs: $rhs) -> Self::Output {
                let mut result = N::zero();

                for n in 0..D {
                    result += self[n] * rhs[n];
                }

                result
            }
        }
    };
}

scalar_dot_scalar_impl!(N, D, Scalar<N, D>, Scalar<N, D>);
scalar_dot_scalar_impl!(N, D, &Scalar<N, D>, Scalar<N, D>);
scalar_dot_scalar_impl!(N, D, Scalar<N, D>, &Scalar<N, D>);
scalar_dot_scalar_impl!(N, D, &Scalar<N, D>, &Scalar<N, D>);

macro_rules! scalar_div_assign_impl {
    ($tp:ident, $dp:ident, $rhs:ty $(, $defer:tt)?) => {
        impl<$tp: Copy + Num, const $dp: usize> ops::DivAssign<$rhs> for Scalar<$tp, $dp> {
            fn div_assign(&mut self, rhs: $rhs) {
                self.map_mut(|&x, _| x / $($defer)?rhs);
            }
        }
    };
}

scalar_div_assign_impl!(N, D, N);
scalar_div_assign_impl!(N, D, &N, *);

macro_rules! scalar_div_impl {
    ($tp:ident, $dp:ident, $lhs:ty, $rhs:ty $(, $defer:tt)?) => {
        impl<$tp: Copy + Num, const $dp: usize> ops::Div<$rhs> for $lhs {
            type Output = Scalar<$tp, $dp>;

            fn div(self, rhs: $rhs) -> Self::Output {
                self.map(|&x, _| x / $($defer)?rhs)
            }
        }
    }
}

scalar_div_impl!(N, D, Scalar<N, D>, N);
scalar_div_impl!(N, D, &Scalar<N, D>, N);
scalar_div_impl!(N, D, Scalar<N, D>, &N, *);
scalar_div_impl!(N, D, &Scalar<N, D>, &N, *);
