use std::ops;
use num_traits::Num;

/// Mark object as possible member of bounding box
pub trait BoxableScalar<N: Num>: ops::Index<usize> {}