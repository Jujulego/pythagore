use std::hash::{Hash, Hasher};
use num_traits::Num;
use crate::{Force, Matrix, SquareMatrix};
use crate::traits::Dimension;
use crate::transform::errors::InvalidLastColumnError;

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
    type Error = InvalidLastColumnError;

    fn try_from(matrix: SquareMatrix<N, D>) -> Result<Self, Self::Error> {
        let valid = matrix.column_iter(D - 1)
            .enumerate()
            .all(|(idx, x)| if idx == D - 1 { x == &N::one() } else { x == &N::zero() });

        if valid {
            Ok(Transform { matrix })
        } else {
            Err(InvalidLastColumnError {})
        }
    }
}

// Operators
impl<N: Num, const D: usize> PartialEq for Transform<N, D> {
    fn eq(&self, other: &Self) -> bool {
        self.matrix == other.matrix
    }
}

// Tests
#[cfg(test)]
mod tests {
    use crate::{force, matrix};
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
        assert_eq!(
            Transform::scale(&force![1, 2, 3]).as_ref(),
            &matrix![
                [1, 0, 0, 0],
                [0, 2, 0, 0],
                [0, 0, 3, 0],
                [0, 0, 0, 1],
            ]
        )
    }

    #[test]
    fn transform_translate() {
        assert_eq!(
            Transform::translate(&force![1, 2, 3]).as_ref(),
            &matrix![
                [1, 0, 0, 0],
                [0, 1, 0, 0],
                [0, 0, 1, 0],
                [1, 2, 3, 1],
            ]
        )
    }
}