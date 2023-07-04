use std::ops::Bound::{self, Excluded, Included, Unbounded};
use na::{ClosedAdd, ClosedSub, Point, Scalar, SVector};
use num_traits::One;

use crate::{BBox, PointBounds, Walkable};

/// Builds a bounding box from a range of points
///
/// # Example
/// ```
/// use std::ops::Bound::{Excluded, Included};
/// use nalgebra::point;
/// use pythagore::BBox;
///
/// assert_eq!(
///     BBox::from((Excluded(point![1, 2]), Included(point![3, 4]))),
///     BBox::from([
///        (Excluded(1), Included(3)),
///        (Excluded(2), Included(4)),
///     ])
/// )
/// ```
impl<N: Copy + Scalar, const D: usize> From<(Bound<Point<N, D>>, Bound<Point<N, D>>)> for BBox<N, D> {
    fn from(value: (Bound<Point<N, D>>, Bound<Point<N, D>>)) -> Self {
        let mut ranges = [(Unbounded, Unbounded); D];

        for (idx, range) in ranges.iter_mut().enumerate() {
            range.0 = match value.0 {
                Included(x) => Included(unsafe { *x.get_unchecked(idx) }),
                Excluded(x) => Excluded(unsafe { *x.get_unchecked(idx) }),
                Unbounded => Unbounded,
            };

            range.1 = match value.1 {
                Included(x) => Included(unsafe { *x.get_unchecked(idx) }),
                Excluded(x) => Excluded(unsafe { *x.get_unchecked(idx) }),
                Unbounded => Unbounded,
            };
        }

        BBox::from(ranges)
    }
}

impl<N: Copy + Scalar, const D: usize> PointBounds<N, D> for (Bound<Point<N, D>>, Bound<Point<N, D>>) {
    fn start_point(&self) -> Option<Point<N, D>> {
        if let Included(pt) | Excluded(pt) = self.0 {
            Some(pt)
        } else {
            None
        }
    }

    fn end_point(&self) -> Option<Point<N, D>> {
        if let Included(pt) | Excluded(pt) = self.1 {
            Some(pt)
        } else {
            None
        }
    }
}

impl<N: ClosedAdd + ClosedSub + Copy + One + Scalar, const D: usize> Walkable<N, D> for (Bound<Point<N, D>>, Bound<Point<N, D>>) {
    fn first_point(&self) -> Option<Point<N, D>> {
        match self.0 {
            Included(pt) => Some(pt),
            Excluded(pt) => Some(pt + SVector::repeat(N::one())),
            Unbounded => None
        }
    }

    fn last_point(&self) -> Option<Point<N, D>> {
        match self.1 {
            Included(pt) => Some(pt),
            Excluded(pt) => Some(pt - SVector::repeat(N::one())),
            Unbounded => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod point_bounds {
        use na::point;
        use super::*;

        #[test]
        fn test_start_point() {
            assert_eq!(
                (Included(point![0, 0]), Excluded(point![5, 5])).start_point(),
                Some(point![0, 0])
            );

            assert_eq!(
                (Unbounded, Excluded(point![5, 5])).start_point(),
                None
            );
        }

        #[test]
        fn test_end_point() {
            assert_eq!(
                (Included(point![0, 0]), Excluded(point![5, 5])).end_point(),
                Some(point![5, 5])
            );

            assert_eq!(
                (Included(point![0, 0]), Unbounded).end_point(),
                None
            );
        }
    }

    mod walkable {
        use na::point;
        use super::*;

        #[test]
        fn test_first_point() {
            assert_eq!(
                (Included(point![0, 0]), Excluded(point![5, 5])).first_point(),
                Some(point![0, 0])
            );

            assert_eq!(
                (Excluded(point![0, 0]), Excluded(point![5, 5])).first_point(),
                Some(point![1, 1])
            );

            assert_eq!(
                (Unbounded, Excluded(point![5, 5])).first_point(),
                None
            );
        }

        #[test]
        fn test_last_point() {
            assert_eq!(
                (Included(point![0, 0]), Included(point![5, 5])).last_point(),
                Some(point![5, 5])
            );

            assert_eq!(
                (Included(point![0, 0]), Excluded(point![5, 5])).last_point(),
                Some(point![4, 4])
            );

            assert_eq!(
                (Included(point![0, 0]), Unbounded).last_point(),
                None
            );
        }
    }
}