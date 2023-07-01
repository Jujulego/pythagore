use std::ops::Bound::{self, *};

/// Select a bound according to predicate
pub fn select_bound<N, F>(lhs: Bound<&N>, rhs: Bound<&N>, predicate: F) -> Bound<N>
where
    N: Copy,
    F: FnOnce(&N, &N) -> bool,
{
    match (lhs, rhs) {
        (Included(l), Included(r))
        | (Included(l), Excluded(r))
        | (Excluded(l), Included(r))
        | (Excluded(l), Excluded(r)) => {
            if predicate(l, r) {
                lhs.cloned()
            } else {
                rhs.cloned()
            }
        }
        (_, Unbounded) => lhs.cloned(),
        (Unbounded, _) => rhs.cloned(),
    }
}

/// Extracts value from bound (if any)
pub fn value_of_bound<N>(bound: Bound<&N>) -> Option<&N> {
    match bound {
        Included(x) | Excluded(x) => Some(x),
        Unbounded => None,
    }
}
