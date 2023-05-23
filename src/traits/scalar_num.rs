use std::ops;
use std::slice::Iter;
use num_traits::Num;

pub trait ScalarNum<T: Num>: PartialEq + ops::Index<usize> + ops::IndexMut<usize> {
    /// Returns iterator on scalar elements
    fn iter(&self) -> Iter<'_, T>;
}