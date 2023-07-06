use std::cmp::{max, min};
use na::{Point, Scalar};

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
    fn test_max_point() {
        assert_eq!(max_point(&point![0, 5], &point![5, 0]), point![5, 5]);
    }

    #[test]
    fn test_min_point() {
        assert_eq!(min_point(&point![0, 5], &point![5, 0]), point![0, 0]);
    }
}