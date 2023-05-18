use num_traits::real::Real;
use num_traits::{One, Signed, Zero};
use std::ops;

/// `Vector<T>` structure for 2 dimension vectors
///
/// ## Usage
/// ```
/// use pythagore::Vector;
///
/// let v = Vector { dx: 1, dy: 2 };
/// let u = Vector { dx: 1, dy: 2 };
///
/// assert_eq!(v, u);
/// assert_eq!(Vector::null(), Vector { dx: 0, dy: 0 })
/// ```
#[derive(Clone, Copy, Debug, Eq)]
pub struct Vector<T> {
    pub dx: T,
    pub dy: T,
}

// Methods
impl<T: Zero> Vector<T> {
    pub fn null() -> Self {
        Vector::zero()
    }

    pub fn is_null(&self) -> bool {
        self.is_zero()
    }
}

impl<T: One + Zero> Vector<T> {
    pub fn unit_dx() -> Self {
        Vector {
            dx: T::one(),
            dy: T::zero(),
        }
    }

    pub fn unit_dy() -> Self {
        Vector {
            dx: T::zero(),
            dy: T::one(),
        }
    }
}

impl<T: Zero> Zero for Vector<T> {
    fn zero() -> Self {
        Vector {
            dx: T::zero(),
            dy: T::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.dx.is_zero() && self.dy.is_zero()
    }
}

impl<T> Vector<T>
where
    T: ops::Mul + Copy,
    T::Output: ops::Add,
{
    pub fn square_norm(&self) -> <T::Output as ops::Add>::Output {
        self.dx * self.dx + self.dy * self.dy
    }
}

impl<T: Real> Vector<T> {
    pub fn norm(&self) -> T {
        self.square_norm().sqrt()
    }

    pub fn unit(&self) -> Self {
        self / self.norm()
    }
}

impl<T: Signed> Vector<T> {
    pub fn manhattan_norm(&self) -> T {
        self.dx.abs() + self.dy.abs()
    }
}

// Macros
macro_rules! vector_binop_impl {
    ($tp:ident, $lhs:ty, $rhs:ty, $op:tt, $op_trait:ty, $constraint:path $(, $copy:path)?) => {
        impl<$tp: $constraint $(+ $copy)*> $op_trait for $lhs {
            type Output = Vector<$tp::Output>;

            fn $op(self, rhs: $rhs) -> Self::Output {
                Vector {
                    dx: self.dx.$op(rhs.dx),
                    dy: self.dy.$op(rhs.dy),
                }
            }
        }
    };
}

macro_rules! vector_add_impl {
    ($tp:ident, $lhs:ty) => {
        vector_add_impl!($tp, $lhs, $lhs);
        vector_add_impl!($tp, &$lhs, $lhs, Copy);
        vector_add_impl!($tp, $lhs, &$lhs, Copy);
        vector_add_impl!($tp, &$lhs, &$lhs, Copy);
    };
    ($tp:ident, $lhs:ty, $rhs:ty $(, $copy:path)?) => {
        vector_binop_impl!($tp,  $lhs, $rhs, add, ops::Add<$rhs>, ops::Add$(, $copy)?);
    };
}

macro_rules! vector_sub_impl {
    ($tp:ident, $lhs:ty) => {
        vector_sub_impl!($tp, $lhs, $lhs);
        vector_sub_impl!($tp, &$lhs, $lhs, Copy);
        vector_sub_impl!($tp, $lhs, &$lhs, Copy);
        vector_sub_impl!($tp, &$lhs, &$lhs, Copy);
    };
    ($tp:ident, $lhs:ty, $rhs:ty $(, $copy:path)?) => {
        vector_binop_impl!($tp,  $lhs, $rhs, sub, ops::Sub<$rhs>, ops::Sub$(, $copy)?);
    };
}

// Operators
impl<T: PartialEq> PartialEq for Vector<T> {
    fn eq(&self, other: &Self) -> bool {
        self.dx == other.dx && self.dy == other.dy
    }
}

vector_add_impl!(T, Vector<T>);
vector_sub_impl!(T, Vector<T>);

impl<T: ops::AddAssign> ops::AddAssign for Vector<T> {
    fn add_assign(&mut self, rhs: Vector<T>) {
        self.dx += rhs.dx;
        self.dy += rhs.dy;
    }
}

impl<T: ops::SubAssign> ops::SubAssign for Vector<T> {
    fn sub_assign(&mut self, rhs: Vector<T>) {
        self.dx -= rhs.dx;
        self.dy -= rhs.dy;
    }
}

impl<T: ops::Mul + Copy> ops::Mul<T> for Vector<T> {
    type Output = Vector<T::Output>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector {
            dx: self.dx * rhs,
            dy: self.dy * rhs,
        }
    }
}

impl<T: ops::MulAssign + Copy> ops::MulAssign<T> for Vector<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.dx *= rhs;
        self.dy *= rhs;
    }
}

impl<T> ops::Mul for Vector<T>
where
    T: ops::Mul,
    T::Output: ops::Add,
{
    type Output = <T::Output as ops::Add>::Output;

    fn mul(self, rhs: Self) -> Self::Output {
        self.dx * rhs.dy + self.dy * rhs.dx
    }
}

impl<T: ops::Div + Copy> ops::Div<T> for Vector<T> {
    type Output = Vector<T::Output>;

    fn div(self, rhs: T) -> Self::Output {
        Vector {
            dx: self.dx / rhs,
            dy: self.dy / rhs,
        }
    }
}

impl<T: ops::Div + Copy> ops::Div<T> for &Vector<T> {
    type Output = Vector<T::Output>;

    fn div(self, rhs: T) -> Self::Output {
        Vector {
            dx: self.dx / rhs,
            dy: self.dy / rhs,
        }
    }
}

impl<T: ops::DivAssign + Copy> ops::DivAssign<T> for Vector<T> {
    fn div_assign(&mut self, rhs: T) {
        self.dx /= rhs;
        self.dy /= rhs;
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_null_vector() {
        assert_eq!(Vector::null(), Vector { dx: 0, dy: 0 });
    }

    #[test]
    fn it_should_return_unit_vectors() {
        assert_eq!(Vector::unit_dx(), Vector { dx: 1, dy: 0 });
        assert_eq!(Vector::unit_dy(), Vector { dx: 0, dy: 1 });
    }

    #[test]
    fn it_should_return_true_for_null_vector() {
        assert!(Vector::<f32>::null().is_null());
    }

    #[test]
    fn it_should_return_false_for_non_null_vector() {
        let v = Vector { dx: 1, dy: 2 };

        assert!(!v.is_null());
    }

    #[test]
    fn it_should_return_square_norm_of_vector() {
        let v = Vector { dx: 1, dy: 2 };

        assert_eq!(v.square_norm(), 5);
    }

    #[test]
    fn it_should_return_norm_of_vector() {
        let v = Vector { dx: 1.0, dy: 2.0 };

        assert_eq!(v.norm(), 5.0.sqrt());
    }

    #[test]
    fn it_should_return_manhattan_norm_of_vector() {
        let v = Vector { dx: 1, dy: 2 };

        assert_eq!(v.manhattan_norm(), 3);
    }

    #[test]
    fn it_should_be_equal() {
        let v = Vector { dx: 1, dy: 2 };
        let u = Vector { dx: 1, dy: 2 };

        assert_eq!(v, u);
    }

    #[test]
    fn it_should_not_be_equal_dx() {
        let v = Vector { dx: 1, dy: 2 };
        let u = Vector { dx: 2, dy: 2 };

        assert_ne!(v, u);
    }

    #[test]
    fn it_should_not_be_equal_dy() {
        let v = Vector { dx: 1, dy: 2 };
        let u = Vector { dx: 1, dy: 1 };

        assert_ne!(v, u);
    }

    #[test]
    fn it_should_return_sum_of_vectors() {
        let v = Vector { dx: 1, dy: 2 };
        let u = Vector { dx: 3, dy: 4 };

        assert_eq!(v + u, Vector { dx: 4, dy: 6 });
    }

    #[test]
    fn it_should_add_vector_to_v() {
        let mut v = Vector { dx: 1, dy: 2 };
        v += Vector { dx: 3, dy: 4 };

        assert_eq!(v, Vector { dx: 4, dy: 6 });
    }

    #[test]
    fn it_should_return_difference_of_vectors() {
        let v = Vector { dx: 1, dy: 2 };
        let u = Vector { dx: 3, dy: 4 };

        assert_eq!(v - u, Vector { dx: -2, dy: -2 });
    }

    #[test]
    fn it_should_subtract_vector_to_v() {
        let mut v = Vector { dx: 1, dy: 2 };
        v -= Vector { dx: 3, dy: 4 };

        assert_eq!(v, Vector { dx: -2, dy: -2 });
    }

    #[test]
    fn it_should_return_product_vector_by_num() {
        let v = Vector { dx: 1, dy: 2 };

        assert_eq!(v * 3, Vector { dx: 3, dy: 6 });
    }

    #[test]
    fn it_should_multiply_vector_by_num() {
        let mut v = Vector { dx: 1, dy: 2 };
        v *= 3;

        assert_eq!(v, Vector { dx: 3, dy: 6 });
    }

    #[test]
    fn it_should_return_scalar_product_of_vectors() {
        let v = Vector { dx: 1, dy: 2 };
        let u = Vector { dx: 3, dy: 4 };

        assert_eq!(v * u, 10);
    }

    #[test]
    fn it_should_return_division_vector_by_num() {
        let v = Vector { dx: 2, dy: 4 };

        assert_eq!(v / 2, Vector { dx: 1, dy: 2 });
    }

    #[test]
    fn it_should_divide_vector_by_num() {
        let mut v = Vector { dx: 2, dy: 4 };
        v /= 2;

        assert_eq!(v, Vector { dx: 1, dy: 2 });
    }
}
