use num_traits::Num;
use crate::{Force, Matrix, SquareMatrix};

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

// Operators
impl<N: Num, const D: usize> PartialEq for Transform<N, D> {
    fn eq(&self, other: &Self) -> bool {
        self.matrix == other.matrix
    }
}

impl<N: Num, const D: usize> AsRef<SquareMatrix<N, D>> for Transform<N, D> {
    fn as_ref(&self) -> &SquareMatrix<N, D> {
        &self.matrix
    }
}
