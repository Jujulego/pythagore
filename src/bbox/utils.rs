use std::ops::Bound::{self, *};

/// Return a new bound, based on value selected using predicate (either value in bound or given one)
pub fn include_value<N: Copy + PartialEq, F>(bound: &Bound<N>, x: &N, predicate: F) -> Bound<N>
where
    F: FnOnce(&N, &N) -> bool,
{
    match bound {
        Unbounded => Unbounded,
        Excluded(b) => {
            if predicate(b, x) {
                *bound
            } else {
                Included(*x)
            }
        }
        Included(b) => {
            if b == x || predicate(b, x) {
                *bound
            } else {
                Included(*x)
            }
        }
    }
}
