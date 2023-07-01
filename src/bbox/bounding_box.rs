use std::ops::RangeBounds;
use std::ops::Bound::{self, *};
use na::{center, ClosedSub, Point, Scalar, SimdComplexField, SVector};
use num_traits::bounds::{LowerBounded, UpperBounded};
use num_traits::{Bounded, Zero};

use crate::BBox;
use crate::bbox::utils;

/// Aligned Axis Bounding Box
pub trait BoundingBox<N: Scalar, const D: usize>: Sized {
    /// Returns range at given dimension
    fn get_range(&self, d: usize) -> (Bound<&N>, Bound<&N>);

    /// Test if given point is in the bbox
    fn holds(&self, pt: &Point<N, D>) -> bool
    where
        N: PartialOrd
    {
        (0..D)
            .map(|d| self.get_range(d))
            .zip(pt.iter())
            .all(|(range, x)| range.contains(x))
    }

    /// Computes a bbox equal to the intersection of given bboxes
    fn intersection(&self, other: &Self) -> BBox<N, D>
    where
        N: Copy + PartialOrd
    {
        let mut ranges = [(Unbounded, Unbounded); D];

        for (dim, pair) in ranges.iter_mut().enumerate() {
            let rng = self.get_range(dim);
            let oth = other.get_range(dim);

            pair.0 = utils::select_bound(rng.0, oth.0, |a, b| a >= b);
            pair.1 = utils::select_bound(rng.1, oth.1, |a, b| a <= b);
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
        N: Copy + LowerBounded + Zero
    {
        let mut point = Point::default();

        for dim in 0..D {
            point[dim] = *utils::value_of_bound(self.get_range(dim).0).unwrap_or(&N::min_value())
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
        N: Copy + UpperBounded + Zero
    {
        let mut point = Point::default();

        for dim in 0..D {
            point[dim] = *utils::value_of_bound(self.get_range(dim).1).unwrap_or(&N::max_value())
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
        N: Copy + Bounded + SimdComplexField + Zero
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
        N: Copy + Bounded + ClosedSub + Zero
    {
        self.end_point() - self.start_point()
    }
}
