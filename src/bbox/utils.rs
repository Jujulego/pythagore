use std::cmp::{max, min};
use std::ops::Bound;
use std::ops::Bound::{Excluded, Included, Unbounded};
use na::{Point, Scalar};

/// Compute maximum bound
pub fn max_bound<'a, N: PartialOrd>(a: &'a Bound<N>, b: &'a Bound<N>) -> &'a Bound<N> {
    match (a, b) {
        (Included(va), Included(vb)) |
        (Excluded(va), Excluded(vb)) |
        (Excluded(va), Included(vb)) => if va < vb { b } else { a },
        (Included(va), Excluded(vb)) => if va <= vb { b } else { a },
        (_, Unbounded) => a,
        (Unbounded, _) => b,
    }
}

/// Compute point with maximum coordinates
pub fn max_point<N: Default + Copy + Ord + Scalar, const D: usize>(a: &Point<N, D>, b: &Point<N, D>) -> Point<N, D> {
    let mut coords = [N::default(); D];

    for (idx, x) in coords.iter_mut().enumerate() {
        *x = *max(
            unsafe { a.get_unchecked(idx) },
            unsafe { b.get_unchecked(idx) }
        );
    }

    Point::from(coords)
}

/// Compute minimum bound
pub fn min_bound<'a, N: PartialOrd>(a: &'a Bound<N>, b: &'a Bound<N>) -> &'a Bound<N> {
    match (a, b) {
        (Included(va), Included(vb)) |
        (Excluded(va), Excluded(vb)) |
        (Included(va), Excluded(vb)) => if va < vb { a } else { b },
        (Excluded(va), Included(vb)) => if va <= vb { a } else { b },
        (_, Unbounded) => a,
        (Unbounded, _) => b,
    }
}

/// Compute point with minimum coordinates
pub fn min_point<N: Default + Copy + Ord + Scalar, const D: usize>(a: &Point<N, D>, b: &Point<N, D>) -> Point<N, D> {
    let mut coords = [N::default(); D];

    for (idx, x) in coords.iter_mut().enumerate() {
        *x = *min(
            unsafe { a.get_unchecked(idx) },
            unsafe { b.get_unchecked(idx) }
        );
    }

    Point::from(coords)
}

// Tests
#[cfg(test)]
mod tests {
    use na::point;
    use super::*;

    #[test]
    fn test_max_bound() {
        assert_eq!(max_bound(&Included(0), &Included(5)), &Included(5));
        assert_eq!(max_bound(&Included(0), &Excluded(5)), &Excluded(5));
        assert_eq!(max_bound(&Included(0), &Excluded(0)), &Excluded(0));
        assert_eq!(max_bound(&Excluded(0), &Included(5)), &Included(5));
        assert_eq!(max_bound(&Excluded(0), &Included(0)), &Excluded(0));
        assert_eq!(max_bound(&Excluded(0), &Excluded(5)), &Excluded(5));
        assert_eq!(max_bound(&Excluded(0), &Unbounded), &Excluded(0));
        assert_eq!(max_bound(&Unbounded, &Included(5)), &Included(5));
    }

    #[test]
    fn test_max_point() {
        assert_eq!(max_point(&point![0, 5], &point![5, 0]), point![5, 5]);
    }

    #[test]
    fn test_min_bound() {
        assert_eq!(min_bound(&Included(0), &Included(5)), &Included(0));
        assert_eq!(min_bound(&Included(0), &Excluded(5)), &Included(0));
        assert_eq!(min_bound(&Included(0), &Excluded(0)), &Excluded(0));
        assert_eq!(min_bound(&Excluded(0), &Included(5)), &Excluded(0));
        assert_eq!(min_bound(&Excluded(0), &Included(0)), &Excluded(0));
        assert_eq!(min_bound(&Excluded(0), &Excluded(5)), &Excluded(0));
        assert_eq!(min_bound(&Excluded(0), &Unbounded), &Excluded(0));
        assert_eq!(min_bound(&Unbounded, &Included(5)), &Included(5));
    }

    #[test]
    fn test_min_point() {
        assert_eq!(min_point(&point![0, 5], &point![5, 0]), point![0, 0]);
    }
}