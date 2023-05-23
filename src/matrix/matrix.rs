use num_traits::Num;
use crate::Scalar;

#[derive(Clone, Copy, Debug, Eq)]
pub struct Matrix<N: Copy + Num, const L: usize, const C: usize> {
    elements: [Scalar<N, C>; L],
}

// Methods
impl<N: Copy + Num, const L: usize, const C: usize> Matrix<N, L, C> {
    pub fn column(&self, c: usize) -> Scalar<N, L> {
        let mut column = Scalar::default();

        for l in 0..L {
            column[l] = self.elements[l][c]
        }

        column
    }

    pub fn line(&self, l: usize) -> &Scalar<N, C> {
        &self.elements[l]
    }

    pub fn line_mut(&mut self, l: usize) -> &mut Scalar<N, C> {
        &mut self.elements[l]
    }
}

// Utils
impl<N: Copy + Num, const L: usize, const C: usize> Default for Matrix<N, L, C> {
    #[inline]
    fn default() -> Self {
        Matrix { elements: [Scalar::default(); L] }
    }
}

impl<N: Copy + Num, const L: usize, const C: usize> From<[Scalar<N, C>; L]> for Matrix<N, L, C> {
    fn from(value: [Scalar<N, C>; L]) -> Self {
        Matrix { elements: value }
    }
}

impl<N: Copy + Num, const L: usize, const C: usize> From<[[N; C]; L]> for Matrix<N, L, C> {
    fn from(value: [[N; C]; L]) -> Self {
        let mut matrix = Matrix::default();

        for l in 0..L {
            matrix.elements[l] = Scalar::from(value[l])
        }

        matrix
    }
}

// Operators
impl<N: Copy + Num, const L: usize, const C: usize> PartialEq for Matrix<N, L, C> {
    fn eq(&self, other: &Self) -> bool {
        self.elements == other.elements
    }
}