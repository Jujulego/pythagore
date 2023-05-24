use std::iter::Flatten;
use std::ops::Neg;
use std::slice::{Iter, IterMut};
use num_traits::{Num, Signed, Zero};
use crate::{owned_unop, Scalar};

/// `Matrix<N, L, C>` utility structure for matrix LxC compute
#[derive(Clone, Copy, Debug, Eq)]
pub struct Matrix<N: Num, const L: usize, const C: usize> {
    pub(crate) elements: [Scalar<N, C>; L],
}

type MatrixIter<'a, N, const C: usize> = Flatten<Iter<'a, Scalar<N, C>>>;
type MatrixIterMut<'a, N, const C: usize> = Flatten<IterMut<'a, Scalar<N, C>>>;

// Methods
impl<N: Num, const L: usize, const C: usize> Matrix<N, L, C> {
    /// Returns iterator on all elements
    pub fn iter(&self) -> MatrixIter<N, C> {
        self.elements.iter().flatten()
    }

    /// Returns mutable iterator on all elements
    pub fn iter_mut(&mut self) -> MatrixIterMut<N, C> {
        self.elements.iter_mut().flatten()
    }

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

impl<'a, N: Num, const L: usize, const C: usize> IntoIterator for &'a Matrix<N, L, C> {
    type Item = &'a N;
    type IntoIter = MatrixIter<'a, N, C>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, N: Num, const L: usize, const C: usize> IntoIterator for &'a mut Matrix<N, L, C> {
    type Item = &'a mut N;
    type IntoIter = MatrixIterMut<'a, N, C>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<N: Copy + Num, const L: usize, const C: usize> FromIterator<N> for Matrix<N, L, C> {
    fn from_iter<T: IntoIterator<Item=N>>(iter: T) -> Self {
        let mut matrix = Matrix::default();
        let mut line = 0;
        let mut column = 0;

        for x in iter.into_iter().take(L * C) {
            matrix.elements[line][column] = x;

            column += 1;

            if column == C {
                line += 1;
                column = 0;
            }
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

impl<N: Copy + Signed, const L: usize, const C: usize> Neg for &Matrix<N, L, C> {
    type Output = Matrix<N, L, C>;

    fn neg(self) -> Self::Output {
        self.iter().map(|&x| -x).collect()
    }
}

owned_unop!(Neg, Matrix<N, L, C>, neg, <N: Copy + Signed, const L: usize, const C: usize>);

// Tests
#[cfg(test)]
mod tests {
    use crate::matrix;

    #[test]
    fn column_iter() {
        let matrix = matrix![
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9],
        ];

        let column: Vec<&i32> = matrix.column_iter(0).collect();

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

        let column: Vec<&i32> = matrix.line_iter(0).collect();

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

    #[test]
    fn matrix_neg() {
        let matrix = matrix![
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9],
        ];

        assert_eq!(-matrix, matrix![
            [-1, -2, -3],
            [-4, -5, -6],
            [-7, -8, -9],
        ]);
    }
}