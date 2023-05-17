use std::ops;

/// `Vector<T>` structure for 2 dimension vectors
///
/// ## Usage
/// ```
/// use pythagore::Vector;
///
/// let a = Vector { dx: 1, dy: 2 };
/// let b = Vector { dx: 1, dy: 2 };
///
/// assert_eq!(a, b);
/// ```
#[derive(Debug, Eq)]
pub struct Vector<T> {
    pub dx: T,
    pub dy: T,
}

// Operators
impl<T: PartialEq> PartialEq for Vector<T> {
    fn eq(&self, other: &Self) -> bool {
        self.dx == other.dx && self.dy == other.dy
    }
}

impl<T: ops::Add> ops::Add for Vector<T> {
    type Output = Vector<T::Output>;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        Vector {
            dx: self.dx + rhs.dx,
            dy: self.dy + rhs.dy,
        }
    }
}

impl<T: ops::AddAssign> ops::AddAssign for Vector<T> {
    fn add_assign(&mut self, rhs: Vector<T>) {
        self.dx += rhs.dx;
        self.dy += rhs.dy;
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_be_equal() {
        let a = Vector { dx: 1, dy: 2 };
        let b = Vector { dx: 1, dy: 2 };

        assert_eq!(a, b);
    }

    #[test]
    fn it_should_not_be_equal_dx() {
        let a = Vector { dx: 1, dy: 2 };
        let b = Vector { dx: 2, dy: 2 };

        assert_ne!(a, b);
    }

    #[test]
    fn it_should_not_be_equal_dy() {
        let a = Vector { dx: 1, dy: 2 };
        let b = Vector { dx: 1, dy: 1 };

        assert_ne!(a, b);
    }

    #[test]
    fn it_should_return_sum_of_vectors() {
        let a = Vector { dx: 1, dy: 2 };
        let b = Vector { dx: 3, dy: 4 };

        assert_eq!(a + b, Vector { dx: 4, dy: 6 });
    }

    #[test]
    fn it_should_add_vector_to_a() {
        let mut a = Vector { dx: 1, dy: 2 };
        a += Vector { dx: 3, dy: 4 };

        assert_eq!(a, Vector { dx: 4, dy: 6 });
    }
}