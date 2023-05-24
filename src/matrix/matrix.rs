use std::slice::{Iter, IterMut};
use num_traits::{Num, Zero};
use crate::Scalar;

/// `Matrix<N, L, C>` utility structure for matrix LxC compute
#[derive(Clone, Copy, Debug, Eq)]
pub struct Matrix<N: Num, const L: usize, const C: usize> {
    pub(crate) elements: [Scalar<N, C>; L],
}

// Methods
impl<N: Num, const L: usize, const C: usize> Matrix<N, L, C> {
    /// Returns iterator on column elements
    pub fn column_iter(&self, column: usize) -> impl Iterator<Item=&N> {
        self.elements.iter()
            .map(move |l| &l[column])
    }

    /// Returns mutable iterator on column elements
    pub fn column_iter_mut(&mut self, column: usize) -> impl Iterator<Item=&mut N> {
        self.elements.iter_mut()
            .map(move |l| &mut l[column])
    }

    /// Returns iterator on line elements
    pub fn line_iter(&self, line: usize) -> Iter<N> {
        self.elements[line].iter()
    }

    /// Returns mutable iterator on line elements
    pub fn line_iter_mut(&mut self, line: usize) -> IterMut<N> {
        self.elements[line].iter_mut()
    }
}

// Utils
impl<N: Copy + Num, const L: usize, const C: usize> Default for Matrix<N, L, C> {
    #[inline]
    fn default() -> Self {
        Matrix { elements: [Scalar::zero(); L] }
    }
}

impl<N: Num, const L: usize, const C: usize> From<[Scalar<N, C>; L]> for Matrix<N, L, C> {
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
impl<N: Num, const L: usize, const C: usize> PartialEq for Matrix<N, L, C> {
    fn eq(&self, other: &Self) -> bool {
        self.elements == other.elements
    }
}

// Tests
#[cfg(test)]
mod tests {
    use crate::{Matrix, matrix};

    #[test]
    fn column_iter() {
        let matrix = matrix![
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9],
        ];

        let column = matrix.column_iter(0).collect::<Vec<&i32>>();

        assert_eq!(column, vec![&1, &4, &7]);
    }

    #[test]
    fn column_iter_mut() {
        let mut matrix = matrix![
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9],
        ];

        matrix.column_iter_mut(0).for_each(|x| *x = 0);

        assert_eq!(matrix, matrix![
            [0, 2, 3],
            [0, 5, 6],
            [0, 8, 9],
        ]);
    }

    #[test]
    fn line_iter() {
        let matrix = matrix![
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9],
        ];

        let column = matrix.line_iter(0).collect::<Vec<&i32>>();

        assert_eq!(column, vec![&1, &2, &3]);
    }

    #[test]
    fn line_iter_mut() {
        let mut matrix = matrix![
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9],
        ];

        matrix.line_iter_mut(0).for_each(|x| *x = 0);

        assert_eq!(matrix, matrix![
            [0, 0, 0],
            [4, 5, 6],
            [7, 8, 9],
        ]);
    }
}