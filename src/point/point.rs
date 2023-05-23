use std::ops;
use std::slice::{Iter, SliceIndex};
use num_traits::{Num, Zero};

use crate::Scalar;
use crate::traits::{Dimension, BoxableScalar};
use crate::Vector;

/// `Point<N, const D: usize>` structure for n dimension points
#[derive(Clone, Copy, Debug, Default, Eq)]
pub struct Point<N: Copy + Num, const D: usize> {
    pub(crate) scalar: Scalar<N, D>,
}

// Methods
impl<N: Copy + Num, const D: usize> Point<N, D> {
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
        pt.scalar[D - 1] = N::one();

        pt
    }

    /// Returns true if point is origin
    pub fn is_origin(&self) -> bool {
        self.scalar.elements[..D - 1].iter().all(|e| e.is_zero())
    }
}

// Utils
impl<N: Copy + Num, const D: usize> Dimension<D> for Point<N, D> {
    /// Returns point's dimension
    #[inline]
    fn dimension() -> usize {
        D - 1
    }
}

impl<N: Copy + Num, const D: usize> BoxableScalar<N> for Point<N, D> {
    /// Returns iterator on point elements
    #[inline]
    fn iter(&self) -> Iter<'_, N> {
        self.scalar[..D-1].iter()
    }
}

macro_rules! point_from_array_impl {
    ($dim:literal) => {
        #[cfg(not(feature = "generic_const_exprs"))]
        impl<N: Copy + Num> From<&[N; $dim]> for Point<N, { $dim + 1 }> {
            fn from(value: &[N; $dim]) -> Self {
                let mut scalar = Scalar::zero();

                for n in 0..$dim {
                    scalar[n] = value[n];
                }

                scalar[$dim] = N::one();

                Point { scalar }
            }
        }
    };
}

point_from_array_impl!(2);
point_from_array_impl!(3);

#[cfg(feature = "generic_const_exprs")]
impl<N: Copy + Num, const D: usize> From<&[N; D]> for Point<N, { D + 1 }> {
    fn from(value: &[N; D]) -> Self {
        let mut scalar = Scalar::zero();

        for n in 0..D {
            scalar[n] = value[n];
        }

        scalar[D] = N::one();

        Point { scalar }
    }
}

macro_rules! point_from_scalar_impl {
    ($dim:literal) => {
        #[cfg(not(feature = "generic_const_exprs"))]
        impl<N: Copy + Num> From<&Scalar<N, $dim>> for Point<N, { $dim + 1 }> {
            #[inline]
            fn from(value: &Scalar<N, $dim>) -> Self {
                Point::from(&value.elements)
            }
        }
    };
}

point_from_scalar_impl!(2);
point_from_scalar_impl!(3);

#[cfg(feature = "generic_const_exprs")]
impl<N: Copy + Num, const D: usize> From<&Scalar<N, D>> for Point<N, { D + 1 }> {
    #[inline]
    fn from(value: &Scalar<N, D>) -> Self {
        Point::from(&value.elements)
    }
}

// Operators
impl<N: Copy + Num, const D: usize> PartialEq for Point<N, D> {
    fn eq(&self, other: &Self) -> bool {
        self.scalar == other.scalar
    }
}

impl<N: Copy + Num, I: SliceIndex<[N]>, const D: usize> ops::Index<I> for Point<N, D> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.scalar[index]
    }
}

impl<N: Copy + Num, I: SliceIndex<[N]>, const D: usize> ops::IndexMut<I> for Point<N, D> {
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

point_add_assign_impl!(N, D, Vector<N, D>);
point_add_assign_impl!(N, D, &Vector<N, D>);

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

point_add_impl!(N, D, Point<N, D>, Vector<N, D>);
point_add_impl!(N, D, &Point<N, D>, Vector<N, D>);
point_add_impl!(N, D, Point<N, D>, &Vector<N, D>);
point_add_impl!(N, D, &Point<N, D>, &Vector<N, D>);

point_add_impl!(N, D, Vector<N, D>, Point<N, D>);
point_add_impl!(N, D, &Vector<N, D>, Point<N, D>);
point_add_impl!(N, D, Vector<N, D>, &Point<N, D>);
point_add_impl!(N, D, &Vector<N, D>, &Point<N, D>);

macro_rules! point_sub_assign_impl {
    ($tp:ident, $dp:ident, $rhs:ty) => {
        impl<$tp: Copy + Num, const $dp: usize> ops::SubAssign<$rhs> for Point<$tp, $dp> {
            fn sub_assign(&mut self, rhs: $rhs) {
                self.scalar -= rhs.scalar;
            }
        }
    }
}

point_sub_assign_impl!(N, D, Vector<N, D>);
point_sub_assign_impl!(N, D, &Vector<N, D>);

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

point_sub_impl!(N, D, Point<N, D>, Point<N, D>, Vector);
point_sub_impl!(N, D, &Point<N, D>, Point<N, D>, Vector);
point_sub_impl!(N, D, Point<N, D>, &Point<N, D>, Vector);
point_sub_impl!(N, D, &Point<N, D>, &Point<N, D>, Vector);

point_sub_impl!(N, D, Point<N, D>, Vector<N, D>, Point);
point_sub_impl!(N, D, &Point<N, D>, Vector<N, D>, Point);
point_sub_impl!(N, D, Point<N, D>, &Vector<N, D>, Point);
point_sub_impl!(N, D, &Point<N, D>, &Vector<N, D>, Point);

point_sub_impl!(N, D, Vector<N, D>, Point<N, D>, Point);
point_sub_impl!(N, D, &Vector<N, D>, Point<N, D>, Point);
point_sub_impl!(N, D, Vector<N, D>, &Point<N, D>, Point);
point_sub_impl!(N, D, &Vector<N, D>, &Point<N, D>, Point);

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
