/// `Vector<T>` structure for 2 dimension vectors
///
/// ## Usage
/// ```
/// use pythagore::Vector;
///
/// let a = Vector { dx: 1, dy: 2 };
/// let b = Vector { dx: 1, dy: 2 };
///
/// assert_eq!(a == b);
/// ```
#[derive(Debug, Eq)]
pub struct Vector<T> {
    pub dx: T,
    pub dy: T,
}

// Operators
impl<T: PartialEq> PartialEq for Vector<T> {
    fn eq(&self, other: &Vector<T>) -> bool {
        self.dx == other.dx && self.dy == other.dy
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
}