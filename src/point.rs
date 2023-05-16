/// `Point<T>` structure for 2 dimension points
///
/// ## Usage
/// ```
/// use pythagore::Point;
///
/// let a = Point { x: 1, y: 2 };
/// let b = Point { x: 1, y: 2 };
///
/// assert_eq!(a == b);
/// ```
#[derive(Debug, Eq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

// Operators
impl<T: PartialEq> PartialEq for Point<T> {
    fn eq(&self, other: &Point<T>) -> bool {
        self.x == other.x && self.y == other.y
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_be_equal() {
        let a = Point { x: 1, y: 2 };
        let b = Point { x: 1, y: 2 };

        assert_eq!(a, b);
    }

    #[test]
    fn it_should_not_be_equal_x() {
        let a = Point { x: 1, y: 2 };
        let b = Point { x: 2, y: 2 };

        assert_ne!(a, b);
    }

    #[test]
    fn it_should_not_be_equal_y() {
        let a = Point { x: 1, y: 2 };
        let b = Point { x: 1, y: 1 };

        assert_ne!(a, b);
    }
}