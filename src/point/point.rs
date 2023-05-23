use std::ops;
use std::slice::{Iter, SliceIndex};
use num_traits::{Num, Zero};

use crate::Scalar;
use crate::traits::Dimension;
use crate::Vector;

/// `Point<T, const D: usize>` structure for n dimension points
#[derive(Clone, Copy, Debug, Default, Eq)]
pub struct Point<T: Copy + Num, const D: usize> {
    pub(crate) scalar: Scalar<T, D>,
}

// Methods
impl<T: Copy + Num, const D: usize> Point<T, D> {
    /// Returns iterator on point elements
    pub fn iter(&self) -> Iter<'_, T> {
        self.scalar[..D-1].iter()
    }

    /// Returns origin point
    ///
    /// ## Example
    /// ```
    /// use pythagore::{point, Point2D, Point3D};
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
        self.scalar.elements[..D - 1].iter().all(|e| e.is_zero())
    }
}

// Utils
impl<T: Copy + Num, const D: usize> Dimension<D> for Point<T, D> {
    /// Returns point's dimension
    #[inline]
    fn dimension() -> usize {
        D - 1
    }
}

macro_rules! point_from_array_impl {
    ($dim:literal) => {
        impl<T: Copy + Num> From<&[T; { $dim - 1 }]> for Point<T, $dim> {
            fn from(value: &[T; { $dim - 1 }]) -> Self {
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

point_from_array_impl!(3);
point_from_array_impl!(4);

macro_rules! point_from_scalar_impl {
    ($dim:literal) => {
        impl<T: Copy + Num> From<&Scalar<T, { $dim - 1 }>> for Point<T, $dim> {
            #[inline]
            fn from(value: &Scalar<T, { $dim - 1 }>) -> Self {
                Point::from(&value.elements)
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

impl<T: Copy + Num, I: SliceIndex<[T]>, const D: usize> ops::Index<I> for Point<T, D> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.scalar[index]
    }
}

impl<T: Copy + Num, I: SliceIndex<[T]>, const D: usize> ops::IndexMut<I> for Point<T, D> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
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
                self.scalar -= rhs.scalar;
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
                $res { scalar: self.scalar - rhs.scalar }
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

// Tests
#[cfg(test)]
mod tests {
    use crate::{point, scalar, vector};
    use super::*;

    #[test]
    fn point_is_origin() {
        assert!(Point::<f32, 4>::origin().is_origin());

        assert!(!point!{ x: 1, y: 2 }.is_origin());
        assert!(!point!{ x: 1, y: 2, z: 3 }.is_origin());
    }

    #[test]
    fn point_from_array() {
        let pt = Point::from(&[1, 2, 3]);

        assert_eq!(pt.scalar.elements, [1, 2, 3, 1]);
    }

    #[test]
    fn point_from_scalar() {
        let pt = Point::from(&scalar![1, 2, 3]);

        assert_eq!(pt.scalar.elements, [1, 2, 3, 1]);
    }

    #[test]
    fn point_add_assign() {
        let mut a = point!{ x: 1, y: 2 };
        a += vector!{ dx: 3, dy: 4 };

        assert_eq!(a, point!{ x: 4, y: 6 });
        assert_eq!(a.scalar[2], 1);
    }

    #[test]
    fn point_add_vector() {
        let a = point!{ x: 1, y: 2 };
        let b = a + vector!{ dx: 3, dy: 4 };

        assert_eq!(b, point!{ x: 4, y: 6 });
        assert_eq!(b.scalar[2], 1);
    }

    #[test]
    fn point_sub_assign() {
        let mut a = point!{ x: 1, y: 2 };
        a -= vector!{ dx: 3, dy: 4 };

        assert_eq!(a, point!{ x: -2, y: -2 });
        assert_eq!(a.scalar[2], 1);
    }

    #[test]
    fn point_sub_vector() {
        let a = point!{ x: 1, y: 2 };
        let b = a - vector!{ dx: 3, dy: 4 };

        assert_eq!(b, point!{ x: -2, y: -2 });
        assert_eq!(b.scalar[2], 1);
    }

    #[test]
    fn point_sub_point() {
        let a = point!{ x: 1, y: 2 };
        let b = a - point!{ x: 3, y: 4 };

        assert_eq!(b, vector!{ dx: -2, dy: -2 });
        assert_eq!(b.scalar[2], 0);
    }
}
