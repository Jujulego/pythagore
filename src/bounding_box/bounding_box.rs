use std::cmp::{max, min};
use num_traits::Num;

use crate::{Point, Vector};
use crate::traits::Dimension;

/// `BoundingBox<T, const D: usize>` structure for n dimension bounding box
#[derive(Clone, Copy, Debug)]
pub struct BoundingBox<T: Copy + Num, const D: usize> {
    start: Point<T, D>,
    end: Point<T, D>,
}

// Methods
impl<T: Copy + Num + Ord, const D: usize> BoundingBox<T, D> {
    pub fn new(base: &Point<T, D>, size: &Vector<T, D>) -> BoundingBox<T, D> {
        let mut start = Point::origin();
        let mut end = Point::origin();

        let other = base + size;

        for n in 0..D {
            start[n] = min(base[n], other[n]);
            end[n] = max(base[n], other[n]);
        }

        BoundingBox { start, end }
    }

    /// Returns true if point is within the bounding box
    pub fn contains(&self, pt: &Point<T, D>) -> bool {
        let diff = pt - self.start;

        self.size().iter()
            .zip(diff.iter())
            .all(|(&size_e, diff_e)| (T::zero()..=size_e).contains(diff_e))
    }

    pub fn size(&self) -> Vector<T, D> {
        self.end - self.start
    }
}

// Utils
impl<T: Copy + Num, const D: usize> Dimension<D> for BoundingBox<T, D> {
    /// Returns bounding box's dimension
    #[inline]
    fn dimension() -> usize {
        D - 1
    }
}

// Tests
#[cfg(test)]
mod tests {
    use crate::{point, vector};
    use super::*;

    #[test]
    fn bbox_contains_point() {
        let bbox = BoundingBox::new(&Point::origin(), &vector!{ dx: 5, dy: 5 });

        assert!(bbox.contains(&Point::origin()));
        assert!(bbox.contains(&point!{ x: 1, y: 1 }));
        assert!(bbox.contains(&point!{ x: 5, y: 5 }));

        assert!(!bbox.contains(&point!{ x: -1, y: 1 }));
        assert!(!bbox.contains(&point!{ x: 10, y: 1 }));

        assert!(!bbox.contains(&point!{ x: 1, y: -1 }));
        assert!(!bbox.contains(&point!{ x: 1, y: 10 }));
    }
}