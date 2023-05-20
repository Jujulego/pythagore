use std::ops;
use num_traits::{Num, Zero};

use crate::Scalar;
use crate::Vector;

/// `Point<T, const D: usize>` structure for n dimension points
#[derive(Clone, Copy, Debug, Default, Eq)]
pub struct Point<T: Copy + Num, const D: usize> {
    scalar: Scalar<T, D>,
}

pub type Point2D<T> = Point<T, 3>;
pub type Point3D<T> = Point<T, 4>;

// Methods
impl<T: Copy + Num, const D: usize> Point<T, D> {
    pub const DIMENSION: usize = D - 1;

    /// Returns point's dimension
    ///
    /// ### Example
    /// ```
    /// use pythagore::*;
    ///
    /// assert_eq!(point!{ x: 1, y: 2 }.dimension(), 2);
    /// assert_eq!(point!{ x: 1, y: 2, z: 3 }.dimension(), 3);
    /// ```
    #[inline]
    pub const fn dimension(&self) -> usize {
        D - 1
    }

    /// Returns origin point
    ///
    /// ## Example
    /// ```
    /// use pythagore::*;
    ///
    /// assert_eq!(Point2D::origin(), point!{ x: 0, y: 0 });
    /// assert_eq!(Point3D::origin(), point!{ x: 0, y: 0, z: 0 });
    /// ```
    pub fn origin() -> Self {
        let mut pt = Point { scalar: Scalar::zero() };
        pt.scalar[D - 1] = T::one();

        pt
    }

    /// Returns true if point is origin
    pub fn is_origin(&self) -> bool {
        self.scalar.elements[0..D - 1].iter().all(|e| e.is_zero())
    }
}

impl<T: Copy + Num> Point2D<T> {
    /// Returns ref on x element of point
    ///
    /// ## Example
    /// ```
    /// use pythagore::*;
    ///
    /// assert_eq!(point!{ x: 1, y: 2 }.x(), &1);
    /// ```
    #[inline]
    pub fn x(&self) -> &T {
        &self.scalar[0]
    }

    /// Returns mutable ref on x element of point
    ///
    /// ## Example
    /// ```
    /// use pythagore::*;
    ///
    /// let mut v = point!{ x: 1, y: 2 };
    /// *v.x_mut() = 5;
    ///
    /// assert_eq!(v.x(), &5);
    /// ```
    #[inline]
    pub fn x_mut(&mut self) -> &mut T {
        &mut self.scalar[0]
    }

    /// Returns ref on y element of point
    ///
    /// ## Example
    /// ```
    /// use pythagore::*;
    ///
    /// assert_eq!(point!{ x: 1, y: 2 }.y(), &2);
    /// ```
    #[inline]
    pub fn y(&self) -> &T {
        &self.scalar[1]
    }

    /// Returns mutable ref on y element of point
    ///
    /// ## Example
    /// ```
    /// use pythagore::*;
    ///
    /// let mut v = point!{ x: 1, y: 2 };
    /// *v.y_mut() = 5;
    ///
    /// assert_eq!(v.y(), &5);
    /// ```
    #[inline]
    pub fn y_mut(&mut self) -> &mut T {
        &mut self.scalar[1]
    }
}

impl<T: Copy + Num> Point3D<T> {
    /// Returns ref on x element of point
    ///
    /// ## Example
    /// ```
    /// use pythagore::*;
    ///
    /// assert_eq!(point!{ x: 1, y: 2, z: 3 }.x(), &1);
    /// ```
    #[inline]
    pub fn x(&self) -> &T {
        &self.scalar[0]
    }

    /// Returns mutable ref on x element of point
    ///
    /// ## Example
    /// ```
    /// use pythagore::*;
    ///
    /// let mut v = point!{ x: 1, y: 2, z: 3 };
    /// *v.x_mut() = 5;
    ///
    /// assert_eq!(v.x(), &5);
    /// ```
    #[inline]
    pub fn x_mut(&mut self) -> &mut T {
        &mut self.scalar[0]
    }

    /// Returns ref on y element of point
    ///
    /// ## Example
    /// ```
    /// use pythagore::*;
    ///
    /// assert_eq!(point!{ x: 1, y: 2, z: 3 }.y(), &2);
    /// ```
    #[inline]
    pub fn y(&self) -> &T {
        &self.scalar[1]
    }

    /// Returns mutable ref on y element of point
    ///
    /// ## Example
    /// ```
    /// use pythagore::*;
    ///
    /// let mut v = point!{ x: 1, y: 2, z: 3 };
    /// *v.y_mut() = 5;
    ///
    /// assert_eq!(v.y(), &5);
    /// ```
    #[inline]
    pub fn y_mut(&mut self) -> &mut T {
        &mut self.scalar[1]
    }

    /// Returns ref on z element of point
    ///
    /// ## Example
    /// ```
    /// use pythagore::*;
    ///
    /// assert_eq!(point!{ x: 1, y: 2, z: 3 }.z(), &3);
    /// ```
    #[inline]
    pub fn z(&self) -> &T {
        &self.scalar[2]
    }

    /// Returns mutable ref on z element of point
    ///
    /// ## Example
    /// ```
    /// use pythagore::*;
    ///
    /// let mut v = point!{ x: 1, y: 2, z: 3 };
    /// *v.z_mut() = 5;
    ///
    /// assert_eq!(v.z(), &5);
    /// ```
    #[inline]
    pub fn z_mut(&mut self) -> &mut T {
        &mut self.scalar[2]
    }
}

// Utils
macro_rules! point_from_array_impl {
    ($dim:literal) => {
        impl<T: Copy + Num> From<[T; { $dim - 1 }]> for Point<T, $dim> {
            fn from(value: [T; { $dim - 1 }]) -> Self {
                Scalar::from(value).into()
            }
        }
    };
}

point_from_array_impl!(3);
point_from_array_impl!(4);

macro_rules! point_from_scalar_impl {
    ($dim:literal) => {
        impl<T: Copy + Num> From<Scalar<T, { $dim - 1 }>> for Point<T, $dim> {
            fn from(value: Scalar<T, { $dim - 1 }>) -> Self {
                let mut scalar = Scalar::zero();

                for n in 0..$dim - 1 {
                    scalar[n] = value[n];
                }

                scalar[$dim - 1] = T::one();

                Point { scalar }
            }
        }
    };
}

point_from_scalar_impl!(3);
point_from_scalar_impl!(4);

// Operators
impl<T: Copy + Num, const D: usize> PartialEq for Point<T, D> {
    fn eq(&self, other: &Self) -> bool {
        self.scalar == other.scalar
    }
}

impl<T: Copy + Num, const D: usize> ops::Index<usize> for Point<T, D> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.scalar[index]
    }
}

impl<T: Copy + Num, const D: usize> ops::IndexMut<usize> for Point<T, D> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.scalar[index]
    }
}

macro_rules! point_add_assign_impl {
    ($tp:ident, $dp:ident, $rhs:ty) => {
        impl<$tp: Copy + Num, const $dp: usize> ops::AddAssign<$rhs> for Point<$tp, $dp> {
            fn add_assign(&mut self, rhs: $rhs) {
                self.scalar += rhs.scalar;
            }
        }
    }
}

point_add_assign_impl!(T, D, Vector<T, D>);
point_add_assign_impl!(T, D, &Vector<T, D>);

macro_rules! point_add_impl {
    ($tp:ident, $dp:ident, $lhs:ty, $rhs:ty) => {
        impl<$tp: Copy + Num, const $dp: usize> ops::Add<$rhs> for $lhs {
            type Output = Point<$tp, $dp>;

            fn add(self, rhs: $rhs) -> Self::Output {
                Point { scalar: self.scalar + rhs.scalar }
            }
        }
    }
}

point_add_impl!(T, D, Point<T, D>, Vector<T, D>);
point_add_impl!(T, D, &Point<T, D>, Vector<T, D>);
point_add_impl!(T, D, Point<T, D>, &Vector<T, D>);
point_add_impl!(T, D, &Point<T, D>, &Vector<T, D>);

point_add_impl!(T, D, Vector<T, D>, Point<T, D>);
point_add_impl!(T, D, &Vector<T, D>, Point<T, D>);
point_add_impl!(T, D, Vector<T, D>, &Point<T, D>);
point_add_impl!(T, D, &Vector<T, D>, &Point<T, D>);

macro_rules! point_sub_assign_impl {
    ($tp:ident, $dp:ident, $rhs:ty) => {
        impl<$tp: Copy + Num, const $dp: usize> ops::SubAssign<$rhs> for Point<$tp, $dp> {
            fn sub_assign(&mut self, rhs: $rhs) {
                self.scalar += rhs.scalar;
            }
        }
    }
}

point_sub_assign_impl!(T, D, Vector<T, D>);
point_sub_assign_impl!(T, D, &Vector<T, D>);

macro_rules! point_sub_impl {
    ($tp:ident, $dp:ident, $lhs:ty, $rhs:ty, $res:tt) => {
        impl<$tp: Copy + Num, const $dp: usize> ops::Sub<$rhs> for $lhs {
            type Output = $res<$tp, $dp>;

            fn sub(self, rhs: $rhs) -> Self::Output {
                $res { scalar: self.scalar + rhs.scalar }
            }
        }
    }
}

point_sub_impl!(T, D, Point<T, D>, Point<T, D>, Vector);
point_sub_impl!(T, D, &Point<T, D>, Point<T, D>, Vector);
point_sub_impl!(T, D, Point<T, D>, &Point<T, D>, Vector);
point_sub_impl!(T, D, &Point<T, D>, &Point<T, D>, Vector);

point_sub_impl!(T, D, Point<T, D>, Vector<T, D>, Point);
point_sub_impl!(T, D, &Point<T, D>, Vector<T, D>, Point);
point_sub_impl!(T, D, Point<T, D>, &Vector<T, D>, Point);
point_sub_impl!(T, D, &Point<T, D>, &Vector<T, D>, Point);

point_sub_impl!(T, D, Vector<T, D>, Point<T, D>, Point);
point_sub_impl!(T, D, &Vector<T, D>, Point<T, D>, Point);
point_sub_impl!(T, D, Vector<T, D>, &Point<T, D>, Point);
point_sub_impl!(T, D, &Vector<T, D>, &Point<T, D>, Point);

// Macros
#[macro_export]
macro_rules! point {
    (x: $x:expr, y: $y:expr) => {
        Point::from([$x, $y])
    };
    (y: $y:expr, x: $x:expr) => {
        Point::from([$x, $y])
    };
    (x: $x:expr, y: $y:expr, z: $z:expr) => {
        Point::from([$x, $y, $z])
    };
    (y: $y:expr, x: $x:expr, z: $z:expr) => {
        Point::from([$x, $y, $z])
    };
    (x: $x:expr, z: $z:expr, y: $y:expr) => {
        Point::from([$x, $y, $z])
    };
    (y: $y:expr, z: $z:expr, x: $x:expr) => {
        Point::from([$x, $y, $z])
    };
    (z: $z:expr, x: $x:expr, y: $y:expr) => {
        Point::from([$x, $y, $z])
    };
    (z: $z:expr, y: $y:expr, x: $x:expr) => {
        Point::from([$x, $y, $z])
    };
    ($elem:expr; $d:expr) => {
        Point::from([$elem; $d])
    };
    ($($x:expr),*) => {
        Point::from([$($x), +])
    };
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_is_origin() {
        assert!(Point::<f32, 4>::origin().is_origin());

        assert!(!point!{ x: 1, y: 2 }.is_origin());
        assert!(!point!{ x: 1, y: 2, z: 3 }.is_origin());
    }
}
