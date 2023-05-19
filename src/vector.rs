use num_traits::{Float, Num, Signed, Zero};
use std::ops;

use crate::Scalar;

/// `Vector<T, const D: usize>` structure for n dimension vectors
///
/// ## Usage
/// ```
/// use pythagore::*;
///
/// let v = vector!{ dx: 1, dy: 2 };
/// let u = vector!{ dx: 1, dy: 2 };
///
/// assert_eq!(v, u);
/// ```
#[derive(Clone, Copy, Debug, Default, Eq)]
pub struct Vector<T: Copy + Num, const D: usize> {
    scalar: Scalar<T, D>,
}

// Methods
impl<T: Copy + Num, const D: usize> Vector<T, D> {
    /// Returns vector's dimension
    #[inline]
    pub const fn dimension(&self) -> usize {
        D
    }

    /// Returns a null vector
    ///
    /// ## Example
    /// ```
    /// use pythagore::*;
    ///
    /// assert_eq!(Vector::null(), vector!{ dx: 0, dy: 0 })
    /// ```
    pub fn null() -> Self {
        Self::zero()
    }

    pub fn is_null(&self) -> bool {
        self.is_zero()
    }
}

impl<T: Copy + Num + ops::AddAssign, const D: usize> Vector<T, D> {
    pub fn square_norm(&self) -> T {
        let mut result = T::zero();

        for n in 0..D {
            result += self[n] * self[n];
        }

        result
    }
}

impl<T: Copy + Float + ops::AddAssign, const D: usize> Vector<T, D> {
    pub fn norm(&self) -> T {
        self.square_norm().sqrt()
    }

    pub fn unit(&self) -> Self {
        self / self.norm()
    }
}

impl<T: Copy + Signed + ops::AddAssign, const D: usize> Vector<T, D> {
    pub fn manhattan_norm(&self) -> T {
        let mut result = T::zero();

        for n in 0..D {
            result += self[n].abs();
        }

        result
    }
}

impl<T: Copy + Num> Vector<T, 2> {
    pub fn unit_dx() -> Self {
        Vector::from([T::one(), T::zero()])
    }

    pub fn unit_dy() -> Self {
        Vector::from([T::zero(), T::one()])
    }

    pub fn dx(&self) -> &T {
        &self.scalar[0]
    }

    pub fn dx_mut(&mut self) -> &mut T {
        &mut self.scalar[0]
    }

    pub fn dy(&self) -> &T {
        &self.scalar[1]
    }

    pub fn dy_mut(&mut self) -> &mut T {
        &mut self.scalar[1]
    }
}

impl<T: Copy + Num> Vector<T, 3> {
    pub fn unit_dx() -> Self {
        Vector::from([T::one(), T::zero(), T::zero()])
    }

    pub fn unit_dy() -> Self {
        Vector::from([T::zero(), T::one(), T::zero()])
    }

    pub fn unit_dz() -> Self {
        Vector::from([T::zero(), T::zero(), T::one()])
    }

    pub fn dx(&self) -> &T {
        &self.scalar[0]
    }

    pub fn dx_mut(&mut self) -> &mut T {
        &mut self.scalar[0]
    }

    pub fn dy(&self) -> &T {
        &self.scalar[1]
    }

    pub fn dy_mut(&mut self) -> &mut T {
        &mut self.scalar[1]
    }

    pub fn dz(&self) -> &T {
        &self.scalar[2]
    }

    pub fn dz_mut(&mut self) -> &mut T {
        &mut self.scalar[2]
    }
}

// Utils
impl<T: Copy + Num, const D: usize> From<[T; D]> for Vector<T, D> {
    fn from(value: [T; D]) -> Self {
        Scalar::from(value).into()
    }
}

impl<T: Copy + Num, const D: usize> From<Scalar<T, D>> for Vector<T, D> {
    fn from(value: Scalar<T, D>) -> Self {
        Vector { scalar: value }
    }
}

impl<T: Copy + Num, const D: usize> TryInto<Vector<T, D>> for Vec<T> {
    type Error = <Vec<T> as TryInto<Scalar<T, D>>>::Error;

    fn try_into(self) -> Result<Vector<T, D>, Self::Error> {
        self.try_into().map(|s: Scalar<T, D>| s.into())
    }
}

impl<T: Copy + Num, const D: usize> Zero for Vector<T, D> {
    fn zero() -> Self {
        Vector::from(Scalar::zero())
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

impl<T: Copy + Signed, const D: usize> ops::Neg for Vector<T, D> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector::from(-self.scalar)
    }
}

impl<T: Copy + Signed, const D: usize> ops::Neg for &Vector<T, D> {
    type Output = Vector<T, D>;

    fn neg(self) -> Self::Output {
        Vector::from(-self.scalar)
    }
}

impl<T: Copy + Num, const D: usize> ops::AddAssign for Vector<T, D> {
    fn add_assign(&mut self, rhs: Self) {
        self.scalar += rhs.scalar;
    }
}

impl<T: Copy + Num, const D: usize> ops::AddAssign<&Self> for Vector<T, D> {
    fn add_assign(&mut self, rhs: &Self) {
        self.scalar += rhs.scalar;
    }
}

impl<T: Copy + Num, const D: usize> ops::Add for Vector<T, D> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector::from(self.scalar + rhs.scalar)
    }
}

impl<T: Copy + Num, const D: usize> ops::Add<&Self> for Vector<T, D> {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        Vector::from(self.scalar + rhs.scalar)
    }
}

impl<T: Copy + Num, const D: usize> ops::Add<Vector<T, D>> for &Vector<T, D> {
    type Output = Vector<T, D>;

    fn add(self, rhs: Vector<T, D>) -> Self::Output {
        Vector::from(self.scalar + rhs.scalar)
    }
}

impl<T: Copy + Num, const D: usize> ops::Add<&Vector<T, D>> for &Vector<T, D> {
    type Output = Vector<T, D>;

    fn add(self, rhs: &Vector<T, D>) -> Self::Output {
        Vector::from(self.scalar + rhs.scalar)
    }
}

impl<T: Copy + Num, const D: usize> ops::SubAssign for Vector<T, D> {
    fn sub_assign(&mut self, rhs: Self) {
        self.scalar -= rhs.scalar;
    }
}

impl<T: Copy + Num, const D: usize> ops::SubAssign<&Self> for Vector<T, D> {
    fn sub_assign(&mut self, rhs: &Self) {
        self.scalar -= rhs.scalar;
    }
}

impl<T: Copy + Num, const D: usize> ops::Sub for Vector<T, D> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector::from(self.scalar - rhs.scalar)
    }
}

impl<T: Copy + Num, const D: usize> ops::Sub<&Self> for Vector<T, D> {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        Vector::from(self.scalar - rhs.scalar)
    }
}

impl<T: Copy + Num, const D: usize> ops::Sub<Vector<T, D>> for &Vector<T, D> {
    type Output = Vector<T, D>;

    fn sub(self, rhs: Vector<T, D>) -> Self::Output {
        Vector::from(self.scalar - rhs.scalar)
    }
}

impl<T: Copy + Num, const D: usize> ops::Sub<&Vector<T, D>> for &Vector<T, D> {
    type Output = Vector<T, D>;

    fn sub(self, rhs: &Vector<T, D>) -> Self::Output {
        Vector::from(self.scalar - rhs.scalar)
    }
}

impl<T: Copy + Num, const D: usize> ops::MulAssign<T> for Vector<T, D> {
    fn mul_assign(&mut self, rhs: T) {
        self.scalar *= rhs;
    }
}

impl<T: Copy + Num, const D: usize> ops::MulAssign<&T> for Vector<T, D> {
    fn mul_assign(&mut self, rhs: &T) {
        self.scalar *= *rhs;
    }
}

impl<T: Copy + Num, const D: usize> ops::Mul<T> for Vector<T, D> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Vector::from(self.scalar * rhs)
    }
}

impl<T: Copy + Num, const D: usize> ops::Mul<&T> for Vector<T, D> {
    type Output = Self;

    fn mul(self, rhs: &T) -> Self::Output {
        Vector::from(self.scalar * *rhs)
    }
}

impl<T: Copy + Num, const D: usize> ops::Mul<T> for &Vector<T, D> {
    type Output = Vector<T, D>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector::from(self.scalar * rhs)
    }
}

impl<T: Copy + Num, const D: usize> ops::Mul<&T> for &Vector<T, D> {
    type Output = Vector<T, D>;

    fn mul(self, rhs: &T) -> Self::Output {
        Vector::from(self.scalar * *rhs)
    }
}

impl<T: Copy + Num + ops::AddAssign, const D: usize> ops::Mul for Vector<T, D> {
    type Output = T;

    fn mul(self, rhs: Self) -> Self::Output {
        self.scalar * rhs.scalar
    }
}

impl<T: Copy + Num + ops::AddAssign, const D: usize> ops::Mul<&Self> for Vector<T, D> {
    type Output = T;

    fn mul(self, rhs: &Self) -> Self::Output {
        self.scalar * rhs.scalar
    }
}

impl<T: Copy + Num + ops::AddAssign, const D: usize> ops::Mul<Vector<T, D>> for &Vector<T, D> {
    type Output = T;

    fn mul(self, rhs: Vector<T, D>) -> Self::Output {
        self.scalar * rhs.scalar
    }
}

impl<T: Copy + Num + ops::AddAssign, const D: usize> ops::Mul<&Vector<T, D>> for &Vector<T, D> {
    type Output = T;

    fn mul(self, rhs: &Vector<T, D>) -> Self::Output {
        self.scalar * rhs.scalar
    }
}

impl<T: Copy + Num, const D: usize> ops::DivAssign<T> for Vector<T, D> {
    fn div_assign(&mut self, rhs: T) {
        self.scalar /= rhs;
    }
}

impl<T: Copy + Num, const D: usize> ops::DivAssign<&T> for Vector<T, D> {
    fn div_assign(&mut self, rhs: &T) {
        self.scalar /= *rhs;
    }
}

impl<T: Copy + Num, const D: usize> ops::Div<T> for Vector<T, D> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Vector::from(self.scalar / rhs)
    }
}

impl<T: Copy + Num, const D: usize> ops::Div<&T> for Vector<T, D> {
    type Output = Self;

    fn div(self, rhs: &T) -> Self::Output {
        Vector::from(self.scalar / *rhs)
    }
}

impl<T: Copy + Num, const D: usize> ops::Div<T> for &Vector<T, D> {
    type Output = Vector<T, D>;

    fn div(self, rhs: T) -> Self::Output {
        Vector::from(self.scalar / rhs)
    }
}

impl<T: Copy + Num, const D: usize> ops::Div<&T> for &Vector<T, D> {
    type Output = Vector<T, D>;

    fn div(self, rhs: &T) -> Self::Output {
        Vector::from(self.scalar / *rhs)
    }
}

// Macros
#[macro_export]
macro_rules! vector {
    (dx: $x:expr, dy: $y:expr) => {
        Vector::from([$x, $y])
    };
    (dy: $y:expr, dx: $x:expr) => {
        Vector::from([$x, $y])
    };
    (dx: $x:expr, dy: $y:expr, dz: $z:expr) => {
        Vector::from([$x, $y, $z])
    };
    (dy: $y:expr, dx: $x:expr, dz: $z:expr) => {
        Vector::from([$x, $y, $z])
    };
    (dx: $x:expr, dz: $z:expr, dy: $y:expr) => {
        Vector::from([$x, $y, $z])
    };
    (dy: $y:expr, dz: $z:expr, dx: $x:expr) => {
        Vector::from([$x, $y, $z])
    };
    (dz: $z:expr, dx: $x:expr, dy: $y:expr) => {
        Vector::from([$x, $y, $z])
    };
    (dz: $z:expr, dy: $y:expr, dx: $x:expr) => {
        Vector::from([$x, $y, $z])
    };
    ($elem:expr; $d:expr) => {
        Vector::from([$elem; $d])
    };
    ($($x:expr),*) => {
        Vector::from([$($x), +])
    };
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_null_vector() {
        assert_eq!(Vector::null(), vector!{ dx: 0, dy: 0 });
    }

    #[test]
    fn it_should_return_unit_vectors() {
        assert_eq!(Vector::<i32, 2>::unit_dx(), vector!{ dx: 1, dy: 0 });
        assert_eq!(Vector::<i32, 2>::unit_dy(), vector!{ dx: 0, dy: 1 });
    }

    #[test]
    fn it_should_return_true_for_null_vector() {
        assert!(Vector::<i32, 2>::null().is_null());
    }

    #[test]
    fn it_should_return_false_for_non_null_vector() {
        let v = vector!{ dx: 1, dy: 2 };

        assert!(!v.is_null());
    }

    #[test]
    fn it_should_return_square_norm_of_vector() {
        let v = vector!{ dx: 1, dy: 2 };

        assert_eq!(v.square_norm(), 5);
    }

    #[test]
    fn it_should_return_norm_of_vector() {
        let v = vector!{ dx: 1.0, dy: 2.0 };

        assert_eq!(v.norm(), 5.0.sqrt());
    }

    #[test]
    fn it_should_return_manhattan_norm_of_vector() {
        let v = vector!{ dx: 1, dy: 2 };

        assert_eq!(v.manhattan_norm(), 3);
    }

    #[test]
    fn it_should_be_equal() {
        let v = vector!{ dx: 1, dy: 2 };
        let u = vector!{ dx: 1, dy: 2 };

        assert_eq!(v, u);
    }

    #[test]
    fn it_should_not_be_equal_dx() {
        let v = vector!{ dx: 1, dy: 2 };
        let u = vector!{ dx: 2, dy: 2 };

        assert_ne!(v, u);
    }

    #[test]
    fn it_should_not_be_equal_dy() {
        let v = vector!{ dx: 1, dy: 2 };
        let u = vector!{ dx: 1, dy: 1 };

        assert_ne!(v, u);
    }

    #[test]
    fn it_should_return_negative_vector() {
        let v = vector!{ dx: 1, dy: 2 };

        assert_eq!(-v, vector!{ dx: -1, dy: -2 });
        assert_eq!(-&v, vector!{ dx: -1, dy: -2 });
    }

    #[test]
    fn it_should_return_sum_of_vectors() {
        let v = vector!{ dx: 1, dy: 2 };
        let u = vector!{ dx: 3, dy: 4 };

        assert_eq!( v +  u, vector!{ dx: 4, dy: 6 });
        assert_eq!(&v +  u, vector!{ dx: 4, dy: 6 });
        assert_eq!( v + &u, vector!{ dx: 4, dy: 6 });
        assert_eq!(&v + &u, vector!{ dx: 4, dy: 6 });
    }

    #[test]
    fn it_should_add_vector_to_v() {
        let mut v = vector!{ dx: 1, dy: 2 };
        v +=  vector!{ dx: 3, dy: 4 };
        v += &vector!{ dx: 5, dy: 6 };

        assert_eq!(v, vector!{ dx: 9, dy: 12 });
    }

    #[test]
    fn it_should_return_difference_of_vectors() {
        let v = vector!{ dx: 1, dy: 2 };
        let u = vector!{ dx: 3, dy: 4 };

        assert_eq!( v -  u, vector!{ dx: -2, dy: -2 });
        assert_eq!(&v -  u, vector!{ dx: -2, dy: -2 });
        assert_eq!( v - &u, vector!{ dx: -2, dy: -2 });
        assert_eq!(&v - &u, vector!{ dx: -2, dy: -2 });
    }

    #[test]
    fn it_should_subtract_vector_to_v() {
        let mut v = vector!{ dx: 1, dy: 2 };
        v -=  vector!{ dx: 3, dy: 4 };
        v -= &vector!{ dx: 5, dy: 6 };

        assert_eq!(v, vector!{ dx: -7, dy: -8 });
    }

    #[test]
    fn it_should_return_product_vector_by_num() {
        let v = vector!{ dx: 1, dy: 2 };

        assert_eq!( v *  3, vector!{ dx: 3, dy: 6 });
        assert_eq!(&v *  3, vector!{ dx: 3, dy: 6 });
        assert_eq!( v * &3, vector!{ dx: 3, dy: 6 });
        assert_eq!(&v * &3, vector!{ dx: 3, dy: 6 });
    }

    #[test]
    fn it_should_multiply_vector_by_num() {
        let mut v = vector!{ dx: 1, dy: 2 };
        v *=  3;
        v *= &3;

        assert_eq!(v, vector!{ dx: 9, dy: 18 });
    }

    #[test]
    fn it_should_return_scalar_product_of_vectors() {
        let v = vector!{ dx: 1, dy: 2 };
        let u = vector!{ dx: 3, dy: 4 };

        assert_eq!( v *  u, 10);
        assert_eq!(&v *  u, 10);
        assert_eq!( v * &u, 10);
        assert_eq!(&v * &u, 10);
    }

    #[test]
    fn it_should_return_division_vector_by_num() {
        let v = vector!{ dx: 2, dy: 4 };

        assert_eq!( v /  2, vector!{ dx: 1, dy: 2 });
        assert_eq!(&v /  2, vector!{ dx: 1, dy: 2 });
        assert_eq!( v / &2, vector!{ dx: 1, dy: 2 });
        assert_eq!(&v / &2, vector!{ dx: 1, dy: 2 });
    }

    #[test]
    fn it_should_divide_vector_by_num() {
        let mut v = vector!{ dx: 4, dy: 8 };
        v /=  2;
        v /= &2;

        assert_eq!(v, vector!{ dx: 1, dy: 2 });
    }
}
