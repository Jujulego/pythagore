use std::ops::Bound::{self, *};

/// Returns `true` if given range is empty
pub fn range_is_empty<'n, N: PartialOrd>(range: &'n (Bound<&'n N>, Bound<&'n N>)) -> bool {
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
pub fn select_bound<'n, N, F>(lhs: &Bound<&'n N>, rhs: &Bound<&'n N>, predicate: F) -> Bound<&'n N>
where F: FnOnce(&'n N, &'n N) -> bool {
    match (lhs, rhs) {
        (Included(l), Included(r)) |
        (Included(l), Excluded(r)) |
        (Excluded(l), Included(r)) |
        (Excluded(l), Excluded(r)) => if predicate(*l, *r) { *lhs } else { *rhs }
        (Unbounded, _) => *rhs,
        (_, Unbounded) => *lhs,
    }
}

/// Return a new bound, based on value selected using predicate (either value in bound or given one)
pub fn include_value<'n, N: PartialEq, F>(bound: &Bound<&'n N>, x: &'n N, predicate: F) -> Bound<&'n N>
where F: FnOnce(&'n N, &'n N) -> bool {
    match bound {
        Unbounded => Unbounded,
        Excluded(b) => if predicate(*b, x) { *bound } else { Included(x) },
        Included(b) => if *b == x || predicate(*b, x) { *bound } else { Included(x) }
    }
}
