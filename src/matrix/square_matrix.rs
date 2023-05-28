use std::iter::Sum;
use std::ops::MulAssign;
use num_traits::{Num, One, Zero};
use crate::{Matrix, owned_op_assign};
use crate::traits::Dimension;

// Type
pub type SquareMatrix<N, const D: usize> = Matrix<N, D, D>;

// Methods
impl<N: Copy + Num, const D: usize> SquareMatrix<N, D> {
    pub fn identity() -> SquareMatrix<N, D> {
        let mut matrix = SquareMatrix::zero();

        for d in 0..D {
            matrix[(d, d)] = N::one();
        }

        matrix
    }
}

// Utils
impl<N: Copy + Num, const D: usize> Dimension<D> for SquareMatrix<N, D> {}

impl<N: Copy + Num + Sum, const D: usize> One for SquareMatrix<N, D> {
    #[inline]
    fn one() -> Self {
        Self::identity()
    }
}

// Operators
impl<N: Copy + Num + Sum, const D: usize> MulAssign<&SquareMatrix<N, D>> for SquareMatrix<N, D> {
    fn mul_assign(&mut self, rhs: &SquareMatrix<N, D>) {
        *self = *self * rhs
    }
}

owned_op_assign!(MulAssign, SquareMatrix<N, D>, mul_assign, SquareMatrix<N, D>, <N: Copy + Num + Sum, const D: usize>);

// Tests
#[cfg(test)]
mod tests {
    use crate::{matrix, SquareMatrix};

    #[test]
    fn identity_matrix() {
        assert_eq!(SquareMatrix::identity(), matrix![
            [1, 0, 0],
            [0, 1, 0],
            [0, 0, 1],
        ]);
    }

    #[test]
    fn matrix_mul_assign() {
        let mut a = matrix![
            [1, 2],
            [3, 4]
        ];
        a *= matrix![
            [1, 2],
            [3, 4]
        ];

        assert_eq!(a, matrix![
            [ 7, 10],
            [15, 22],
        ])
    }
}