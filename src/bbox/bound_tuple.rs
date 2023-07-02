use std::ops::Bound::{self, Excluded, Included, Unbounded};
use na::{Point, Scalar};
use num_traits::Bounded;

use crate::{BBox, BoundPoints};

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

impl<N: Bounded + Copy + Scalar, const D: usize> BoundPoints<N, D> for (Bound<Point<N, D>>, Bound<Point<N, D>>) {
    fn start_point(&self) -> Point<N, D> {
        if let Included(pt) | Excluded(pt) = self.0 {
            pt
        } else {
            Point::min_value()
        }
    }

    fn end_point(&self) -> Point<N, D> {
        if let Included(pt) | Excluded(pt) = self.1 {
            pt
        } else {
            Point::max_value()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod bound_points {
        use na::point;
        use super::*;

        #[test]
        fn test_start_point() {
            assert_eq!(
                (Included(point![0, 0]), Excluded(point![5, 5])).start_point(),
                point![0, 0]
            );

            assert_eq!(
                (Unbounded, Excluded(point![5, 5])).start_point(),
                Point::min_value()
            );
        }

        #[test]
        fn test_end_point() {
            assert_eq!(
                (Included(point![0, 0]), Excluded(point![5, 5])).end_point(),
                point![5, 5]
            );

            assert_eq!(
                (Included(point![0, 0]), Unbounded).end_point(),
                Point::max_value()
            );
        }
    }
}