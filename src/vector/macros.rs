/// Builds a new vector from given elements
///
/// ## Example
/// ```
/// use pythagore::{vector, Vector};
///
/// // 2d vector
/// assert_eq!(vector!{ dx: 1, dy: 2 }, Vector::from([1, 2]));
///
/// // 3d vector
/// assert_eq!(vector!{ dx: 1, dy: 2, dz: 3 }, Vector::from([1, 2, 3]));
///
/// // Array based
/// assert_eq!(vector![1; 3], Vector::from([1, 1, 1]));
/// assert_eq!(vector![1, 2, 3], Vector::from([1, 2, 3]));
/// ```
#[macro_export]
macro_rules! vector {
    (dx: $x:expr, dy: $y:expr) => {
        vector::Vector::from([$x, $y])
    };
    (dy: $y:expr, dx: $x:expr) => {
        vector::Vector::from([$x, $y])
    };
    (dx: $x:expr, dy: $y:expr, dz: $z:expr) => {
        vector::Vector::from([$x, $y, $z])
    };
    (dy: $y:expr, dx: $x:expr, dz: $z:expr) => {
        vector::Vector::from([$x, $y, $z])
    };
    (dx: $x:expr, dz: $z:expr, dy: $y:expr) => {
        vector::Vector::from([$x, $y, $z])
    };
    (dy: $y:expr, dz: $z:expr, dx: $x:expr) => {
        vector::Vector::from([$x, $y, $z])
    };
    (dz: $z:expr, dx: $x:expr, dy: $y:expr) => {
        vector::Vector::from([$x, $y, $z])
    };
    (dz: $z:expr, dy: $y:expr, dx: $x:expr) => {
        vector::Vector::from([$x, $y, $z])
    };
    ($elem:expr; $d:expr) => {
        vector::Vector::from([$elem; $d])
    };
    ($($x:expr),*) => {
        vector::Vector::from([$($x), +])
    };
}

// Tests
#[cfg(test)]
mod tests {
    use crate::{vector, Vector};

    #[test]
    fn vector_2d_creation() {
        assert_eq!(vector! { dx: 1, dy: 2 }, Vector::from([1, 2]));
        assert_eq!(vector! { dy: 2, dx: 1 }, Vector::from([1, 2]));
    }

    #[test]
    fn vector_3d_creation() {
        assert_eq!(vector!{ dx: 1, dy: 2, dz: 3 }, Vector::from([1, 2, 3]));
        assert_eq!(vector!{ dy: 2, dx: 1, dz: 3 }, Vector::from([1, 2, 3]));
        assert_eq!(vector!{ dy: 2, dz: 3, dx: 1 }, Vector::from([1, 2, 3]));
        assert_eq!(vector!{ dy: 2, dz: 3, dx: 1 }, Vector::from([1, 2, 3]));
        assert_eq!(vector!{ dz: 3, dy: 2, dx: 1 }, Vector::from([1, 2, 3]));
        assert_eq!(vector!{ dz: 3, dy: 2, dx: 1 }, Vector::from([1, 2, 3]));
    }
}