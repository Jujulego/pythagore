use std::ops::Index;

/// Mark object as possible member of bounding box
pub trait BoxableScalar<N>: Index<usize, Output = N> {}