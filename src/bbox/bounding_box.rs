use na::{center, ClosedSub, Point, SVector, Scalar, SimdComplexField};
use num_traits::Bounded;
use std::ops::Bound::{self, *};
use std::ops::RangeBounds;

use crate::bbox::utils::select_bound;
use crate::BBox;

/// Aligned Axis Bounding Box
pub trait BoundingBox<N: Scalar, const D: usize>: Sized {
    /// Returns start bound for given dimension
    fn get_start(&self, d: usize) -> Bound<N>;

    /// Returns start bound for given dimension
    fn get_end(&self, d: usize) -> Bound<N>;

    /// Returns range at given dimension
    fn get_range(&self, d: usize) -> (Bound<N>, Bound<N>) {
        (self.get_start(d), self.get_end(d))
    }

    /// Test if given point is in the bbox
    fn holds(&self, pt: &Point<N, D>) -> bool
    where
        N: PartialOrd,
    {
        pt.iter()
            .enumerate()
            .all(|(d, x)| self.get_range(d).contains(x))
    }

    /// Computes a bbox equal to the intersection of given bboxes
    fn intersection(&self, other: &Self) -> BBox<N, D>
    where
        N: Copy + PartialOrd,
    {
        let mut ranges = [(Unbounded, Unbounded); D];

        for (d, pair) in ranges.iter_mut().enumerate() {
            pair.0 = select_bound(self.get_start(d), other.get_start(d), |a, b| a >= b);
            pair.1 = select_bound(self.get_end(d), other.get_end(d), |a, b| a <= b);
        }

        BBox::from(ranges)
    }

    /// Returns point with all starts of ranges
    ///
    /// ## Examples
    /// ```
    /// use nalgebra::point;
    /// use pythagore::BoundingBox;
    ///
    /// assert_eq!((point![0, 0]..point![5, 5]).start_point(), point![0, 0]);
    /// ```
    fn start_point(&self) -> Point<N, D>
    where
        N: Bounded,
    {
        let mut point = Point::min_value();

        for dim in 0..D {
            if let Included(v) | Excluded(v) = self.get_start(dim) {
                point[dim] = v;
            }
        }

        point
    }

    /// Returns point with all ends of ranges
    ///
    /// ## Examples
    /// ```
    /// use nalgebra::point;
    /// use pythagore::BoundingBox;
    ///
    /// assert_eq!((point![0, 0]..point![5, 5]).end_point(), point![5, 5]);
    /// ```
    fn end_point(&self) -> Point<N, D>
    where
        N: Bounded,
    {
        let mut point = Point::max_value();

        for dim in 0..D {
            if let Included(v) | Excluded(v) = self.get_end(dim) {
                point[dim] = v;
            }
        }

        point
    }

    /// Returns center point of bbox
    ///
    /// ## Examples
    /// ```
    /// use nalgebra::point;
    /// use pythagore::BoundingBox;
    ///
    /// assert_eq!((point![0.0, 0.0]..point![5.0, 5.0]).center_point(), point![2.5, 2.5]);
    /// ```
    fn center_point(&self) -> Point<N, D>
    where
        N: Bounded + SimdComplexField,
    {
        center(&self.start_point(), &self.end_point())
    }

    /// Returns center point of bbox
    ///
    /// ## Examples
    /// ```
    /// use nalgebra::{point, vector};
    /// use pythagore::BoundingBox;
    ///
    /// assert_eq!((point![0, 0]..point![5, 5]).size(), vector![5, 5]);
    /// ```
    fn size(&self) -> SVector<N, D>
    where
        N: Bounded + ClosedSub,
    {
        self.end_point() - self.start_point()
    }
}
