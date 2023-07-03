use std::ops::AddAssign;
use na::{Point, Scalar};
use num_traits::One;

pub struct Walker<N: Scalar, const D: usize> {
    pub(crate) first: Point<N, D>,
    pub(crate) last: Point<N, D>,
}

impl<N: Scalar, const D: usize> Walker<N, D> {
    pub fn first(&self) -> &Point<N, D> {
        &self.first
    }

    pub fn last(&self) -> &Point<N, D> {
        &self.last
    }

    pub fn next(&self, from: &Point<N, D>) -> Option<Point<N, D>>
    where
        N: AddAssign + Copy + One + Ord
    {
        if from == &self.last || unsafe { from.get_unchecked(0) > self.last.get_unchecked(0) } {
            return None;
        }

        let mut next = self.first;
        let mut addable: Option<usize> = None;

        for (idx, v) in from.iter().enumerate() {
            if v < unsafe { self.first.get_unchecked(idx) } {
                break;
            } else if v < unsafe { self.last.get_unchecked(idx) } {
                unsafe { *next.get_unchecked_mut(idx) = *v };
                addable = Some(idx);

                if idx == D - 1 {
                    unsafe { *next.get_unchecked_mut(idx) += N::one() };
                }
            } else {
                if let Some(back) = addable {
                    unsafe { *next.get_unchecked_mut(back) += N::one() };

                    return Some(next);
                } else {
                    unsafe { *next.get_unchecked_mut(idx) = *self.last.get_unchecked(idx) };
                }
            }
        }

        Some(next)
    }
}

// Tests
#[cfg(test)]
mod tests {
    use na::point;
    use super::*;

    #[test]
    fn test_next_on_whole_range() {
        let walker = Walker {
            first: point![0, 0],
            last: point![2, 2],
        };

        assert_eq!(walker.next(&point![0, 0]), Some(point![0, 1]));
        assert_eq!(walker.next(&point![0, 1]), Some(point![0, 2]));
        assert_eq!(walker.next(&point![0, 2]), Some(point![1, 0]));
        assert_eq!(walker.next(&point![1, 0]), Some(point![1, 1]));
        assert_eq!(walker.next(&point![1, 1]), Some(point![1, 2]));
        assert_eq!(walker.next(&point![1, 2]), Some(point![2, 0]));
        assert_eq!(walker.next(&point![2, 0]), Some(point![2, 1]));
        assert_eq!(walker.next(&point![2, 1]), Some(point![2, 2]));
        assert_eq!(walker.next(&point![2, 2]), None);
    }

    #[test]
    fn test_below_left_point() {
        let walker = Walker {
            first: point![0, 0],
            last: point![5, 5]
        };

        assert_eq!(walker.next(&point![-2, -2]), Some(point![0, 0]));
    }

    #[test]
    fn test_left_point() {
        let walker = Walker {
            first: point![0, 0],
            last: point![5, 5]
        };

        assert_eq!(walker.next(&point![-2, 2]), Some(point![0, 0]));
    }

    #[test]
    fn test_over_left_point() {
        let walker = Walker {
            first: point![0, 0],
            last: point![5, 5]
        };

        assert_eq!(walker.next(&point![-2, 7]), Some(point![0, 0]));
    }

    #[test]
    fn test_below_point() {
        let walker = Walker {
            first: point![0, 0],
            last: point![5, 5]
        };

        assert_eq!(walker.next(&point![2, -2]), Some(point![2, 0]));
    }

    #[test]
    fn test_over_point() {
        let walker = Walker {
            first: point![0, 0],
            last: point![5, 5]
        };

        assert_eq!(walker.next(&point![2, 7]), Some(point![3, 0]));
    }

    #[test]
    fn test_last_point() {
        let walker = Walker {
            first: point![0, 0],
            last: point![5, 5]
        };

        assert_eq!(walker.next(&point![5, 5]), None);
    }

    #[test]
    fn test_below_right_point() {
        let walker = Walker {
            first: point![0, 0],
            last: point![5, 5]
        };

        assert_eq!(walker.next(&point![7, -2]), None);
    }

    #[test]
    fn test_right_point() {
        let walker = Walker {
            first: point![0, 0],
            last: point![5, 5]
        };

        assert_eq!(walker.next(&point![7, 2]), None);
    }

    #[test]
    fn test_over_right_point() {
        let walker = Walker {
            first: point![0, 0],
            last: point![5, 5]
        };

        assert_eq!(walker.next(&point![7, 7]), None);
    }
}