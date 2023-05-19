use std::array::TryFromSliceError;
use std::ops;
use num_traits::{Num, Signed, Zero};

/// `Scalar<T, const D: usize>` utility structure for n dimension compute
///
/// ## Usage
/// ```
/// use pythagore::*;
///
/// let s = scalar![1, 2, 3, 4];
///
/// assert_eq!(s[0], 1);
/// assert_eq!(s.dimension(), 4);
/// ```
#[derive(Clone, Copy, Debug, Eq)]
pub struct Scalar<T: Num, const D: usize> {
    elements: [T; D],
}

// Methods
impl<T: Num, const D: usize> Scalar<T, D> {
    /// Returns scalar's dimension
    #[inline]
    pub const fn dimension(&self) -> usize {
        D
    }
}

impl<T: Copy + Num, const D: usize> Scalar<T, D> {
    #[inline]
    fn map(&self, op: impl Fn(T, usize) -> T) -> Self {
        let mut copy = self.clone();
        copy.map_mut(op);

        copy
    }

    #[inline]
    fn map_mut(&mut self, op: impl Fn(T, usize) -> T) {
        for n in 0..D {
            self[n] = op(self[n], n);
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

impl<T: Copy + Signed, const D: usize> ops::Neg for Scalar<T, D> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.map(|x, _| -x)
    }
}

impl<T: Copy + Num, const D: usize> ops::AddAssign for Scalar<T, D> {
    fn add_assign(&mut self, rhs: Self) {
        self.map_mut(|x, n| x + rhs[n]);
    }
}

impl<T: Copy + Num, const D: usize> ops::Add for Scalar<T, D> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.map(|x, n| x + rhs[n])
    }
}

impl<T: Copy + Num, const D: usize> ops::SubAssign for Scalar<T, D> {
    fn sub_assign(&mut self, rhs: Self) {
        self.map_mut(|x, n| x - rhs[n]);
    }
}

impl<T: Copy + Num, const D: usize> ops::Sub for Scalar<T, D> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.map(|x, n| x - rhs[n])
    }
}

impl<T: Copy + Num, const D: usize> ops::MulAssign<T> for Scalar<T, D> {
    fn mul_assign(&mut self, rhs: T) {
        self.map_mut(|x, _| x * rhs);
    }
}

impl<T: Copy + Num, const D: usize> ops::Mul<T> for Scalar<T, D> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        self.map(|x, _| x * rhs)
    }
}

impl<T: Copy + Num + ops::AddAssign, const D: usize> ops::Mul for Scalar<T, D> {
    type Output = T;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = T::zero();

        for n in 0..D {
            result += self[n] * rhs[n];
        }

        result
    }
}

impl<T: Copy + Num, const D: usize> ops::DivAssign<T> for Scalar<T, D> {
    fn div_assign(&mut self, rhs: T) {
        self.map_mut(|x, _| x / rhs);
    }
}

impl<T: Copy + Num, const D: usize> ops::Div<T> for Scalar<T, D> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        self.map(|x, _| x / rhs)
    }
}

// Macros
/// Builds a new scalar from given elements
///
/// ## Example
/// ```
/// use pythagore::*;
///
/// assert_eq!(scalar![1, 2, 3], Scalar::from([1, 2, 3]));
/// assert_eq!(scalar![1; 5], Scalar::from([1; 5]));
/// ```
#[macro_export]
macro_rules! scalar {
    ($elem:expr; $d:expr) => {
        Scalar::from([$elem; $d])
    };
    ($($x:expr),*) => {
        Scalar::from([$($x), +])
    };
}