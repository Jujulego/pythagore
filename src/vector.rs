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
    pub(crate) scalar: Scalar<T, D>,
}

pub type Vector2D<T> = Vector<T, 3>;
pub type Vector3D<T> = Vector<T, 4>;

// Methods
impl<T: Copy + Num, const D: usize> Vector<T, D> {
    pub const DIMENSION: usize = D;

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

impl<T: Copy + Num> Vector2D<T> {
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

impl<T: Copy + Num> Vector3D<T> {
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
        assert_eq!(Vector::<i32, 3>::unit_dx(), vector!{ dx: 1, dy: 0 });
        assert_eq!(Vector::<i32, 3>::unit_dy(), vector!{ dx: 0, dy: 1 });
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

        assert_eq!( v *  u, 11);
        assert_eq!(&v *  u, 11);
        assert_eq!( v * &u, 11);
        assert_eq!(&v * &u, 11);
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
