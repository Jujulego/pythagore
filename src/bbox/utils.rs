use std::ops::Bound::{self, *};

/// Select a bound according to predicate
pub fn select_bound<N, F>(lhs: Bound<N>, rhs: Bound<N>, predicate: F) -> Bound<N>
where
    F: FnOnce(&N, &N) -> bool,
{
    match (&lhs, &rhs) {
        (Included(l), Included(r))
        | (Included(l), Excluded(r))
        | (Excluded(l), Included(r))
        | (Excluded(l), Excluded(r)) => {
            if predicate(l, r) {
                lhs
            } else {
                rhs
            }
        }
        (_, Unbounded) => lhs,
        (Unbounded, _) => rhs,
    }
}
