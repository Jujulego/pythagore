use num_traits::{Num, Zero};
use crate::Matrix;

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
}