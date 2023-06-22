use std::ops::Bound::{self, *};

/// Returns `true` if given range is empty
pub fn range_is_empty<N: PartialOrd>(range: &(Bound<N>, Bound<N>)) -> bool {
    match range {
        (Included(l), Included(r)) => l > r,
        (Included(l), Excluded(r)) |
        (Excluded(l), Included(r)) |
        (Excluded(l), Excluded(r)) => l >= r,
        (Unbounded, _) => false,
        (_, Unbounded) => false,
    }
}

/// Select a bound according to predicate
pub fn select_bound<'b, N, F>(lhs: &'b Bound<N>, rhs: &'b Bound<N>, predicate: F) -> &'b Bound<N>
where F: FnOnce(&N, &N) -> bool {
    match (lhs, rhs) {
        (Included(l), Included(r)) |
        (Included(l), Excluded(r)) |
        (Excluded(l), Included(r)) |
        (Excluded(l), Excluded(r)) => if predicate(l, r) { lhs } else { rhs }
        (Unbounded, _) => rhs,
        (_, Unbounded) => lhs,
    }
}

/// Return a new bound, based on value selected using predicate (either value in bound or given one)
pub fn include_value<N: Copy + PartialEq, F>(bound: &Bound<N>, x: &N, predicate: F) -> Bound<N>
where F: FnOnce(&N, &N) -> bool {
    match &bound {
        Unbounded => Unbounded,
        Excluded(b) => if predicate(b, x) { *bound } else { Included(*x) },
        Included(b) => if b == x || predicate(b, x) { *bound } else { Included(*x) }
    }
}
