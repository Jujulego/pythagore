use std::slice::{Iter, IterMut};
use num_traits::Num;
use crate::Scalar;

#[derive(Clone, Copy, Debug, Eq)]
pub struct Matrix<N: Num, const L: usize, const C: usize> {
    pub(crate) elements: [Scalar<N, C>; L],
}

// Methods
impl<N: Num, const L: usize, const C: usize> Matrix<N, L, C> {
    pub fn column_iter(&self, column: usize) -> impl Iterator<Item=&N> {
        self.elements.iter()
            .map(move |l| &l[column])
    }

    pub fn column_iter_mut(&mut self, column: usize) -> impl Iterator<Item=&mut N> {
        self.elements.iter_mut()
            .map(move |l| &mut l[column])
    }

    pub fn line_iter(&self, line: usize) -> Iter<N> {
        self.elements[line].iter()
    }

    pub fn line_iter_mut(&mut self, line: usize) -> IterMut<N> {
        self.elements[line].iter_mut()
    }
}

// Utils
impl<N: Copy + Num, const L: usize, const C: usize> Default for Matrix<N, L, C> {
    #[inline]
    fn default() -> Self {
        Matrix { elements: [Scalar::default(); L] }
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
    use crate::Matrix;

    #[test]
    fn column_iterator() {
        let matrix = Matrix::from([
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9]
        ]);

        let column = matrix.column_iter(0).collect::<Vec<&i32>>();

        assert_eq!(column, vec![&1, &4, &7]);
    }
}