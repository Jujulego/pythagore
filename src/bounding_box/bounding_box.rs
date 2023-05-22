use std::cmp::{max, min};
use num_traits::Num;

use crate::{Point, Vector};
use crate::traits::Dimension;

/// `BoundingBox<T, const D: usize>` structure for n dimension bounding box
#[derive(Clone, Copy, Debug)]
pub struct BoundingBox<T: Copy + Num, const D: usize> {
    pub origin: Point<T, D>,
    pub size: Vector<T, D>,
}

// Methods
impl<T: Copy + Num + Ord, const D: usize> BoundingBox<T, D> {
    /// Returns true if point is within the bounding box
    pub fn contains(&self, pt: &Point<T, D>) -> bool {
        let d = pt - self.origin;

        for n in 0..D-1 {
            if min(self.size[n], T::zero()) > d[n] || d[n] >= max(self.size[n], T::zero()) {
                return false;
            }
        }

        return true;
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
        let bbox = BoundingBox {
            origin: Point::origin(),
            size: vector!{ dx: 5, dy: 5 },
        };

        assert!(bbox.contains(&Point::origin()));
        assert!(bbox.contains(&point!{ x: 1, y: 1 }));

        assert!(!bbox.contains(&point!{ x: -1, y: 1 }));
        assert!(!bbox.contains(&point!{ x: 10, y: 1 }));

        assert!(!bbox.contains(&point!{ x: 1, y: -1 }));
        assert!(!bbox.contains(&point!{ x: 1, y: 10 }));
    }
}