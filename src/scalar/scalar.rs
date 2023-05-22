use std::array::TryFromSliceError;
use std::ops;
use num_traits::{Num, Signed, Zero};

/// `Scalar<T, const D: usize>` utility structure for n dimension compute
///
/// ## Usage
/// ```
/// use pythagore::scalar;
///
/// let s = scalar![1, 2, 3, 4];
///
/// assert_eq!(s[0], 1);
/// assert_eq!(s.dimension(), 4);
/// ```
#[derive(Clone, Copy, Debug, Eq)]
pub struct Scalar<T: Num, const D: usize> {
    pub(crate) elements: [T; D],
}

// Methods
impl<T: Num, const D: usize> Scalar<T, D> {
    pub const DIMENSION: usize = D;

    /// Returns scalar's dimension
    #[inline]
    pub const fn dimension(&self) -> usize {
        D
    }
}

impl<T: Copy + Num, const D: usize> Scalar<T, D> {
    #[inline]
    fn map(&self, op: impl Fn(&T, usize) -> T) -> Self {
        let mut copy = self.clone();
        copy.map_mut(op);

        copy
    }

    #[inline]
    fn map_mut(&mut self, op: impl Fn(&T, usize) -> T) {
        for n in 0..D {
            self[n] = op(&self[n], n);
        }
    }
}

// Utils
impl<T: Copy + Num, const D: usize> Default for Scalar<T, D> {
    fn default() -> Self {
        Scalar::from([T::zero(); D])
    }
}

impl<T: Num, const D: usize> From<[T; D]> for Scalar<T, D> {
    /// Builds a new scalar form given fixed array
    ///
    /// ## Example
    /// ```
    /// use pythagore::*;
    ///
    /// assert_eq!(Scalar::from([1, 2, 3]), scalar![1, 2, 3]);
    /// ```
    fn from(value: [T; D]) -> Self {
        Scalar { elements: value }
    }
}

impl<T: Copy + Num, const D: usize> TryInto<Scalar<T, D>> for Vec<T> {
    type Error = TryFromSliceError;

    fn try_into(self) -> Result<Scalar<T, D>, Self::Error> {
        self.as_slice().try_into().map(|e: &[T; D]| (*e).into())
    }
}

impl<T: Copy + Num, const D: usize> Zero for Scalar<T, D> {
    fn zero() -> Self {
        Scalar::from([T::zero(); D])
    }

    fn is_zero(&self) -> bool {
        self.elements.iter().all(|e| e.is_zero())
    }
}

// Operators
impl<T: Num, const D: usize> PartialEq for Scalar<T, D> {
    fn eq(&self, other: &Self) -> bool {
        self.elements == other.elements
    }
}

impl<T: Num, const D: usize> ops::Index<usize> for Scalar<T, D> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

impl<T: Num, const D: usize> ops::IndexMut<usize> for Scalar<T, D> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
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

scalar_neg_impl!(T, D, Scalar<T, D>);
scalar_neg_impl!(T, D, &Scalar<T, D>);

macro_rules! scalar_add_assign_impl {
    ($tp:ident, $dp:ident, $rhs:ty) => {
        impl<$tp: Copy + Num, const $dp: usize> ops::AddAssign<$rhs> for Scalar<$tp, $dp> {
            fn add_assign(&mut self, rhs: $rhs) {
                self.map_mut(|&x, n| x + rhs[n]);
            }
        }
    };
}

scalar_add_assign_impl!(T, D, Scalar<T, D>);
scalar_add_assign_impl!(T, D, &Scalar<T, D>);

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

scalar_add_impl!(T, D, Scalar<T, D>, Scalar<T, D>);
scalar_add_impl!(T, D, &Scalar<T, D>, Scalar<T, D>);
scalar_add_impl!(T, D, Scalar<T, D>, &Scalar<T, D>);
scalar_add_impl!(T, D, &Scalar<T, D>, &Scalar<T, D>);

macro_rules! scalar_sub_assign_impl {
    ($tp:ident, $dp:ident, $rhs:ty) => {
        impl<$tp: Copy + Num, const $dp: usize> ops::SubAssign<$rhs> for Scalar<$tp, $dp> {
            fn sub_assign(&mut self, rhs: $rhs) {
                self.map_mut(|&x, n| x - rhs[n]);
            }
        }
    };
}

scalar_sub_assign_impl!(T, D, Scalar<T, D>);
scalar_sub_assign_impl!(T, D, &Scalar<T, D>);

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

scalar_sub_impl!(T, D, Scalar<T, D>, Scalar<T, D>);
scalar_sub_impl!(T, D, &Scalar<T, D>, Scalar<T, D>);
scalar_sub_impl!(T, D, Scalar<T, D>, &Scalar<T, D>);
scalar_sub_impl!(T, D, &Scalar<T, D>, &Scalar<T, D>);

macro_rules! scalar_mul_assign_impl {
    ($tp:ident, $dp:ident, $rhs:ty $(, $defer:tt)?) => {
        impl<$tp: Copy + Num, const $dp: usize> ops::MulAssign<$rhs> for Scalar<$tp, $dp> {
            fn mul_assign(&mut self, rhs: $rhs) {
                self.map_mut(|&x, _| x * $($defer)?rhs);
            }
        }
    };
}

scalar_mul_assign_impl!(T, D, T);
scalar_mul_assign_impl!(T, D, &T, *);

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

scalar_mul_impl!(T, D, Scalar<T, D>, T);
scalar_mul_impl!(T, D, &Scalar<T, D>, T);
scalar_mul_impl!(T, D, Scalar<T, D>, &T, *);
scalar_mul_impl!(T, D, &Scalar<T, D>, &T, *);

macro_rules! scalar_dot_scalar_impl {
    ($tp:ident, $dp:ident, $lhs:ty, $rhs:ty) => {
        impl<$tp: Copy + Num + ops::AddAssign, const $dp: usize> ops::Mul<$rhs> for $lhs {
            type Output = $tp;

            fn mul(self, rhs: $rhs) -> Self::Output {
                let mut result = T::zero();

                for n in 0..D {
                    result += self[n] * rhs[n];
                }

                result
            }
        }
    };
}

scalar_dot_scalar_impl!(T, D, Scalar<T, D>, Scalar<T, D>);
scalar_dot_scalar_impl!(T, D, &Scalar<T, D>, Scalar<T, D>);
scalar_dot_scalar_impl!(T, D, Scalar<T, D>, &Scalar<T, D>);
scalar_dot_scalar_impl!(T, D, &Scalar<T, D>, &Scalar<T, D>);

macro_rules! scalar_div_assign_impl {
    ($tp:ident, $dp:ident, $rhs:ty $(, $defer:tt)?) => {
        impl<$tp: Copy + Num, const $dp: usize> ops::DivAssign<$rhs> for Scalar<$tp, $dp> {
            fn div_assign(&mut self, rhs: $rhs) {
                self.map_mut(|&x, _| x / $($defer)?rhs);
            }
        }
    };
}

scalar_div_assign_impl!(T, D, T);
scalar_div_assign_impl!(T, D, &T, *);

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

scalar_div_impl!(T, D, Scalar<T, D>, T);
scalar_div_impl!(T, D, &Scalar<T, D>, T);
scalar_div_impl!(T, D, Scalar<T, D>, &T, *);
scalar_div_impl!(T, D, &Scalar<T, D>, &T, *);
