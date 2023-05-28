use std::ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign};
use std::slice::{Iter, IterMut, SliceIndex};
use num_traits::{Num, Zero};

use crate::{owned_binop, owned_op_assign, reverse_owned_binop, Scalar, Force};
use crate::traits::{Dimension, BoxableScalar};

/// `Point<N, D>` structure for D dimension points
#[derive(Clone, Copy, Debug, Eq)]
pub struct Point<N: Num, const D: usize> {
    pub(crate) scalar: Scalar<N, D>,
}

// Methods
impl<N: Num, const D: usize> Point<N, D> {
    /// Returns iterator on point elements
    pub fn iter(&self) -> Iter<N> {
        self.scalar[..D-1].iter()
    }

    /// Returns mutable iterator on point elements
    pub fn iter_mut(&mut self) -> IterMut<N> {
        self.scalar[..D-1].iter_mut()
    }
}

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
        self.iter().all(|e| e.is_zero())
    }
}

// Utils
impl<N: Num, const D: usize> BoxableScalar<N> for Point<N, D> {}

impl<N: Copy + Num, const D: usize> Default for Point<N, D> {
    #[inline]
    fn default() -> Self {
        Point::origin()
    }
}

impl<N: Num, const D: usize> Dimension<D> for Point<N, D> {
    /// Returns point's dimension
    #[inline]
    fn dimension() -> usize {
        D - 1
    }
}

impl<'a, N: Num, const D: usize> IntoIterator for &'a Point<N, D> {
    type Item = &'a N;
    type IntoIter = Iter<'a, N>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, N: Num, const D: usize> IntoIterator for &'a mut Point<N, D> {
    type Item = &'a mut N;
    type IntoIter = IterMut<'a, N>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

macro_rules! from_array_impl {
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

from_array_impl!(2);
from_array_impl!(3);

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

macro_rules! from_scalar_impl {
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

from_scalar_impl!(2);
from_scalar_impl!(3);

#[cfg(feature = "generic_const_exprs")]
impl<N: Copy + Num, const D: usize> From<&Scalar<N, D>> for Point<N, { D + 1 }> {
    #[inline]
    fn from(value: &Scalar<N, D>) -> Self {
        Point::from(&value.elements)
    }
}

impl<N: Copy + Num, const D: usize> FromIterator<N> for Point<N, D> {
    fn from_iter<T: IntoIterator<Item = N>>(iter: T) -> Self {
        let mut point = Point::default();
        let mut idx = 0;

        for x in iter.into_iter().take(D - 1) {
            point[idx] = x;
            idx += 1;
        }

        point
    }
}

// Operators
impl<N: Num, const D: usize> PartialEq for Point<N, D> {
    fn eq(&self, other: &Self) -> bool {
        self.scalar == other.scalar
    }
}

impl<N: Num, I: SliceIndex<[N]>, const D: usize> Index<I> for Point<N, D> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.scalar[index]
    }
}

impl<N: Num, I: SliceIndex<[N]>, const D: usize> IndexMut<I> for Point<N, D> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.scalar[index]
    }
}

impl<N: Copy + Num + AddAssign, const D: usize> AddAssign<&Force<N, D>> for Point<N, D> {
    fn add_assign(&mut self, rhs: &Force<N, D>) {
        self.scalar += &rhs.scalar;
    }
}

owned_op_assign!(AddAssign, Point<N, D>, add_assign, Force<N, D>, <N: Copy + Num + AddAssign, const D: usize>);

impl<N: Copy + Num, const D: usize> Add<&Force<N, D>> for &Point<N, D> {
    type Output = Point<N, D>;

    fn add(self, rhs: &Force<N, D>) -> Self::Output {
        Point { scalar: &self.scalar + &rhs.scalar }
    }
}

owned_binop!(Add, Point<N, D>, add, Force<N, D>, <N: Copy + Num, const D: usize>);
reverse_owned_binop!(Add, Point<N, D>, add, Force<N, D>, <N: Copy + Num, const D: usize>);

impl<N: Copy + Num + SubAssign, const D: usize> SubAssign<&Force<N, D>> for Point<N, D> {
    fn sub_assign(&mut self, rhs: &Force<N, D>) {
        self.scalar -= &rhs.scalar;
    }
}

owned_op_assign!(SubAssign, Point<N, D>, sub_assign, Force<N, D>, <N: Copy + Num + SubAssign, const D: usize>);

impl<N: Copy + Num, const D: usize> Sub<&Force<N, D>> for &Point<N, D> {
    type Output = Point<N, D>;

    fn sub(self, rhs: &Force<N, D>) -> Self::Output {
        Point { scalar: &self.scalar - &rhs.scalar }
    }
}

owned_binop!(Sub, Point<N, D>, sub, Force<N, D>, <N: Copy + Num, const D: usize>);
reverse_owned_binop!(Sub, Point<N, D>, sub, Force<N, D>, <N: Copy + Num, const D: usize>);

impl<N: Copy + Num, const D: usize> Sub for &Point<N, D> {
    type Output = Force<N, D>;

    fn sub(self, rhs: &Point<N, D>) -> Self::Output {
        Force { scalar: &self.scalar - &rhs.scalar }
    }
}

owned_binop!(Sub, Point<N, D>, sub, Point<N, D>, <N: Copy + Num, const D: usize>);

// Tests
#[cfg(test)]
mod tests {
    use crate::{point, scalar, force};
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
        a += force!{ dx: 3, dy: 4 };

        assert_eq!(a, point!{ x: 4, y: 6 });
        assert_eq!(a.scalar[2], 1);
    }

    #[test]
    fn point_add_force() {
        let a = point!{ x: 1, y: 2 };
        let b = a + force!{ dx: 3, dy: 4 };

        assert_eq!(b, point!{ x: 4, y: 6 });
        assert_eq!(b.scalar[2], 1);
    }

    #[test]
    fn point_sub_assign() {
        let mut a = point!{ x: 1, y: 2 };
        a -= force!{ dx: 3, dy: 4 };

        assert_eq!(a, point!{ x: -2, y: -2 });
        assert_eq!(a.scalar[2], 1);
    }

    #[test]
    fn point_sub_force() {
        let a = point!{ x: 1, y: 2 };
        let b = a - force!{ dx: 3, dy: 4 };

        assert_eq!(b, point!{ x: -2, y: -2 });
        assert_eq!(b.scalar[2], 1);
    }

    #[test]
    fn point_sub_point() {
        let a = point!{ x: 1, y: 2 };
        let b = a - point!{ x: 3, y: 4 };

        assert_eq!(b, force!{ dx: -2, dy: -2 });
        assert_eq!(b.scalar[2], 0);
    }
}
