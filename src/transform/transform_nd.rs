use std::hash::{Hash, Hasher};
use std::iter::Sum;
use std::ops::{Mul, MulAssign};
use num_traits::{Float, Num};
use crate::{Force, Matrix, owned_binop, owned_op_assign, Point, SquareMatrix};
use crate::traits::Dimension;
use crate::transform::errors::InvalidTransformMatrixError;

/// `Transform<N, D>` structure for D dimension transformations
#[derive(Clone, Copy, Debug, Eq)]
pub struct Transform<N: Num, const D: usize> {
    matrix: SquareMatrix<N, D>,
}

// Methods
impl<N: Copy + Num, const D: usize> Transform<N, D> {
    #[inline]
    pub fn identity() -> Self {
        Transform { matrix: Matrix::identity() }
    }

    pub fn scale(by: &Force<N, D>) -> Self {
        let mut result = Transform::identity();

        for (idx, &x) in by.iter().enumerate() {
            result.matrix[(idx, idx)] = x;
        }

        result
    }

    pub fn translate(by: &Force<N, D>) -> Self {
        let mut result = Transform::identity();

        for (idx, &x) in by.iter().enumerate() {
            result.matrix[(D - 1, idx)] = x;
        }

        result
    }
}

impl<N: Copy + Float> Transform<N, 3> {
    pub fn rotate(theta: N) -> Self {
        let mut result = Transform::identity();
        let (sin, cos) = theta.sin_cos();

        result.matrix[(0, 0)] = cos;
        result.matrix[(0, 1)] = -sin;
        result.matrix[(1, 0)] = sin;
        result.matrix[(1, 1)] = cos;

        result
    }
}

impl<N: Copy + Float> Transform<N, 4> {
    pub fn rotate_x(theta: N) -> Self {
        let mut result = Transform::identity();
        let (sin, cos) = theta.sin_cos();

        result.matrix[(1, 1)] = cos;
        result.matrix[(1, 2)] = -sin;
        result.matrix[(2, 1)] = sin;
        result.matrix[(2, 2)] = cos;

        result
    }

    pub fn rotate_y(theta: N) -> Self {
        let mut result = Transform::identity();
        let (sin, cos) = theta.sin_cos();

        result.matrix[(0, 0)] = cos;
        result.matrix[(0, 2)] = sin;
        result.matrix[(2, 0)] = -sin;
        result.matrix[(2, 2)] = cos;

        result
    }

    pub fn rotate_z(theta: N) -> Self {
        let mut result = Transform::identity();
        let (sin, cos) = theta.sin_cos();

        result.matrix[(0, 0)] = cos;
        result.matrix[(0, 1)] = -sin;
        result.matrix[(1, 0)] = sin;
        result.matrix[(1, 1)] = cos;

        result
    }
}

// Utils
impl<N: Copy + Num, const D: usize> Default for Transform<N, D> {
    #[inline]
    fn default() -> Self {
        Transform::identity()
    }
}

impl<N: Num, const D: usize> Dimension<D> for Transform<N, D> {
    #[inline]
    fn dimension() -> usize {
        D - 1
    }
}

impl<N: Num + Hash, const D: usize> Hash for Transform<N, D> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.matrix.hash(state);
    }
}

impl<N: Num, const D: usize> AsRef<SquareMatrix<N, D>> for Transform<N, D> {
    fn as_ref(&self) -> &SquareMatrix<N, D> {
        &self.matrix
    }
}

impl<N: Num, const D: usize> TryFrom<SquareMatrix<N, D>> for Transform<N, D> {
    type Error = InvalidTransformMatrixError;

    fn try_from(matrix: SquareMatrix<N, D>) -> Result<Self, Self::Error> {
        let valid = matrix.column_iter(D - 1)
            .enumerate()
            .all(|(idx, x)| if idx == D - 1 { x == &N::one() } else { x == &N::zero() });

        if valid {
            Ok(Transform { matrix })
        } else {
            Err(InvalidTransformMatrixError {})
        }
    }
}

// Operators
impl<N: Num, const D: usize> PartialEq for Transform<N, D> {
    fn eq(&self, other: &Self) -> bool {
        self.matrix == other.matrix
    }
}

impl<N: Copy + Num + Sum, const D: usize> MulAssign<&Transform<N, D>> for Force<N, D> {
    fn mul_assign(&mut self, rhs: &Transform<N, D>) {
        *self = *self * rhs;
    }
}

owned_op_assign!(MulAssign, Force<N, D>, mul_assign, Transform<N, D>, <N: Copy + Num + Sum, const D: usize>);

impl<N: Copy + Num + Sum, const D: usize> Mul<&Transform<N, D>> for &Force<N, D> {
    type Output = Force<N, D>;

    fn mul(self, rhs: &Transform<N, D>) -> Self::Output {
        Force::try_from(self.as_ref() * rhs.matrix).unwrap()
    }
}

owned_binop!(Mul, Force<N, D>, mul, Transform<N, D>, <N: Copy + Num + Sum, const D: usize>);

impl<N: Copy + Num + Sum, const D: usize> MulAssign<&Transform<N, D>> for Point<N, D> {
    fn mul_assign(&mut self, rhs: &Transform<N, D>) {
        *self = *self * rhs;
    }
}

owned_op_assign!(MulAssign, Point<N, D>, mul_assign, Transform<N, D>, <N: Copy + Num + Sum, const D: usize>);

impl<N: Copy + Num + Sum, const D: usize> Mul<&Transform<N, D>> for &Point<N, D> {
    type Output = Point<N, D>;

    fn mul(self, rhs: &Transform<N, D>) -> Self::Output {
        Point::try_from(self.as_ref() * rhs.matrix).unwrap()
    }
}

owned_binop!(Mul, Point<N, D>, mul, Transform<N, D>, <N: Copy + Num + Sum, const D: usize>);

impl<N: Copy + Num + Sum, const D: usize> MulAssign<&Transform<N, D>> for Transform<N, D> {
    fn mul_assign(&mut self, rhs: &Transform<N, D>) {
        self.matrix *= rhs.matrix;
    }
}

owned_op_assign!(MulAssign, Transform<N, D>, mul_assign, Transform<N, D>, <N: Copy + Num + Sum, const D: usize>);

impl<N: Copy + Num + Sum, const D: usize> Mul<&Transform<N, D>> for &Transform<N, D> {
    type Output = Transform<N, D>;

    fn mul(self, rhs: &Transform<N, D>) -> Self::Output {
        Transform::try_from(self.matrix * rhs.matrix).unwrap()
    }
}

owned_binop!(Mul, Transform<N, D>, mul, Transform<N, D>, <N: Copy + Num + Sum, const D: usize>);

// Tests
#[cfg(test)]
mod tests {
    use std::f64::consts::PI;
    use crate::{force, matrix, point};
    use super::*;

    #[test]
    fn transform_identity() {
        assert_eq!(
            Transform::<i32, 4>::identity().as_ref(),
            &Matrix::<i32, 4, 4>::identity()
        );
    }

    #[test]
    fn transform_scale() {
        let matrix = Transform::scale(&force![1, 2, 3]);

        assert_eq!(
            matrix.as_ref(),
            &matrix![
                [1, 0, 0, 0],
                [0, 2, 0, 0],
                [0, 0, 3, 0],
                [0, 0, 0, 1],
            ]
        );

        assert_eq!(force![1, 1, 1] * matrix, force![1, 2, 3]);
        assert_eq!(point![1, 1, 1] * matrix, point![1, 2, 3]);
    }

    #[test]
    fn transform_translate() {
        let matrix = Transform::translate(&force![1, 2, 3]);

        assert_eq!(
            matrix.as_ref(),
            &matrix![
                [1, 0, 0, 0],
                [0, 1, 0, 0],
                [0, 0, 1, 0],
                [1, 2, 3, 1],
            ]
        );

        assert_eq!(force![1, 1, 1] * matrix, force![1, 1, 1]);
        assert_eq!(point![1, 1, 1] * matrix, point![2, 3, 4]);
    }

    #[test]
    fn transform_2d_rotate() {
        let matrix = Transform::rotate(PI / 2.0);

        let res = force![1.0, 2.0] * matrix; // ~eq force![2.0, -1.0]
        assert!((res.dx() - 2.0).abs() < 1e-10, "res.dx() -> {} should be near 2.0", res.dx());
        assert!((res.dy() + 1.0).abs() < 1e-10, "res.dy() -> {} should be near -1.0", res.dy());

        let res = point![1.0, 2.0] * matrix; // ~eq point![2.0, -1.0]
        assert!((res.x() - 2.0).abs() < 1e-10, "res.x() -> {} should be near 2.0", res.x());
        assert!((res.y() + 1.0).abs() < 1e-10, "res.y() -> {} should be near -1.0", res.y());
    }

    #[test]
    fn transform_3d_rotate_x() {
        let matrix = Transform::rotate_x(PI / 2.0);

        let res = force![1.0, 2.0, 3.0] * matrix; // ~eq force![1.0, 3.0, -2.0]
        assert_eq!(res.dx(), &1.0);
        assert!((res.dy() - 3.0).abs() < 1e-10, "res.dy() -> {} should be near 3.0", res.dy());
        assert!((res.dz() + 2.0).abs() < 1e-10, "res.dz() -> {} should be near -2.0", res.dz());

        let res = point![1.0, 2.0, 3.0] * matrix; // ~eq point![1.0, 3.0, -2.0]
        assert_eq!(res.x(), &1.0);
        assert!((res.y() - 3.0).abs() < 1e-10, "res.y() -> {} should be near 3.0", res.y());
        assert!((res.z() + 2.0).abs() < 1e-10, "res.z() -> {} should be near -2.0", res.z());
    }

    #[test]
    fn transform_3d_rotate_y() {
        let matrix = Transform::rotate_y(PI / 2.0);

        let res = force![1.0, 2.0, 3.0] * matrix; // ~eq force![-3.0, 2.0, 1.0]
        assert!((res.dx() + 3.0).abs() < 1e-10, "res.dx() -> {} should be near -3.0", res.dx());
        assert_eq!(res.dy(), &2.0);
        assert!((res.dz() - 1.0).abs() < 1e-10, "res.dz() -> {} should be near 1.0", res.dz());

        let res = point![1.0, 2.0, 3.0] * matrix; // ~eq point![-3.0, 2.0, 1.0]
        assert!((res.x() + 3.0).abs() < 1e-10, "res.x() -> {} should be near -3.0", res.x());
        assert_eq!(res.y(), &2.0);
        assert!((res.z() - 1.0).abs() < 1e-10, "res.z() -> {} should be near 1.0", res.z());
    }

    #[test]
    fn transform_3d_rotate_z() {
        let matrix = Transform::rotate_z(PI / 2.0);

        let res = force![1.0, 2.0, 3.0] * matrix; // ~eq force![2.0, -1.0, 3.0]
        assert!((res.dx() - 2.0).abs() < 1e-10, "res.dx() -> {} should be near 2.0", res.dx());
        assert!((res.dy() + 1.0).abs() < 1e-10, "res.dy() -> {} should be near -1.0", res.dy());
        assert_eq!(res.dz(), &3.0);

        let res = point![1.0, 2.0, 3.0] * matrix; // ~eq point![0.0, -1.0]
        assert!((res.x() - 2.0).abs() < 1e-10, "res.x() -> {} should be near 2.0", res.x());
        assert!((res.y() + 1.0).abs() < 1e-10, "res.y() -> {} should be near -1.0", res.y());
        assert_eq!(res.z(), &3.0);
    }

    #[test]
    fn transform_combination() {
        let scale = Transform::scale(&force![1, 2, 3]);
        let translate = Transform::translate(&force![1, 2, 3]);

        assert_eq!(
            (scale * translate).as_ref(),
            &matrix![
                [1, 0, 0, 0],
                [0, 2, 0, 0],
                [0, 0, 3, 0],
                [1, 2, 3, 1],
            ]
        );
    }
}