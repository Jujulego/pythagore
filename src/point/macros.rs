/// Builds a new point from given elements
///
/// ## Example
/// ```
/// use pythagore::{point, Point};
///
/// // 2d point
/// assert_eq!(point!{ x: 1, y: 2 }, Point::from(&[1, 2]));
///
/// // 3d point
/// assert_eq!(point!{ x: 1, y: 2, z: 3 }, Point::from(&[1, 2, 3]));
///
/// // Array based
/// assert_eq!(point![1; 3], Point::from(&[1, 1, 1]));
/// assert_eq!(point![1, 2, 3], Point::from(&[1, 2, 3]));
/// ```
#[macro_export]
macro_rules! point {
    (x: $x:expr, y: $y:expr) => {
        point::Point::from(&[$x, $y])
    };
    (y: $y:expr, x: $x:expr) => {
        point::Point::from(&[$x, $y])
    };
    (x: $x:expr, y: $y:expr, z: $z:expr) => {
        point::Point::from(&[$x, $y, $z])
    };
    (y: $y:expr, x: $x:expr, z: $z:expr) => {
        point::Point::from(&[$x, $y, $z])
    };
    (x: $x:expr, z: $z:expr, y: $y:expr) => {
        point::Point::from(&[$x, $y, $z])
    };
    (y: $y:expr, z: $z:expr, x: $x:expr) => {
        point::Point::from(&[$x, $y, $z])
    };
    (z: $z:expr, x: $x:expr, y: $y:expr) => {
        point::Point::from(&[$x, $y, $z])
    };
    (z: $z:expr, y: $y:expr, x: $x:expr) => {
        point::Point::from(&[$x, $y, $z])
    };
    ($elem:expr; $d:expr) => {
        point::Point::from(&[$elem; $d])
    };
    ($($x:expr),*) => {
        point::Point::from(&[$($x), +])
    };
}

// Tests
#[cfg(test)]
mod tests {
    use crate::{point, Point};

    #[test]
    fn point_2d_creation() {
        assert_eq!(point!{ x: 1, y: 2 }, Point::from(&[1, 2]));
        assert_eq!(point!{ y: 2, x: 1 }, Point::from(&[1, 2]));
    }

    #[test]
    fn point_3d_creation() {
        assert_eq!(point!{ x: 1, y: 2, z: 3 }, Point::from(&[1, 2, 3]));
        assert_eq!(point!{ y: 2, x: 1, z: 3 }, Point::from(&[1, 2, 3]));
        assert_eq!(point!{ y: 2, z: 3, x: 1 }, Point::from(&[1, 2, 3]));
        assert_eq!(point!{ y: 2, z: 3, x: 1 }, Point::from(&[1, 2, 3]));
        assert_eq!(point!{ z: 3, y: 2, x: 1 }, Point::from(&[1, 2, 3]));
        assert_eq!(point!{ z: 3, y: 2, x: 1 }, Point::from(&[1, 2, 3]));
    }
}