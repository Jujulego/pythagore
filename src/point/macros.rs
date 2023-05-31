/// Builds a new point from given elements
///
/// ## Example
/// ```
/// use pythagore::{point, vector};
///
/// // 2d point
/// assert_eq!(point!{ x: 1, y: 2 }.as_ref(), &vector![1, 2, 1]);
///
/// // 3d point
/// assert_eq!(point!{ x: 1, y: 2, z: 3 }.as_ref(), &vector![1, 2, 3, 1]);
///
/// // Array based
/// assert_eq!(point![1; 2].as_ref(), &vector![1, 1, 1]);
/// assert_eq!(point![1, 2, 3].as_ref(), &vector![1, 2, 3, 1]);
/// ```
#[macro_export]
macro_rules! point {
    { x: $x:expr, y: $y:expr } => {
        point![$x, $y]
    };
    { y: $y:expr, x: $x:expr } => {
        point![$x, $y]
    };
    { x: $x:expr, y: $y:expr, z: $z:expr } => {
        point![$x, $y, $z]
    };
    { y: $y:expr, x: $x:expr, z: $z:expr } => {
        point![$x, $y, $z]
    };
    { x: $x:expr, z: $z:expr, y: $y:expr } => {
        point![$x, $y, $z]
    };
    { y: $y:expr, z: $z:expr, x: $x:expr } => {
        point![$x, $y, $z]
    };
    { z: $z:expr, x: $x:expr, y: $y:expr } => {
        point![$x, $y, $z]
    };
    { z: $z:expr, y: $y:expr, x: $x:expr } => {
        point![$x, $y, $z]
    };
    [$elem:expr; $d:expr] => {
        [$elem; $d].iter().copied().collect::<point::Point<_, { $d + 1 }>>()
    };
    [$($x:expr),+] => {{
        [$($x), +].iter().copied().collect::<point::Point<_, { (point!(@count $($x)+)) + 1 }>>()
    }};
    (@count) => { 0 };
    (@count $odd:tt $($a:tt $b:tt)*) => { (point!(@count $($a)*) << 1) | 1 };
    (@count $($a:tt $even:tt)*) => { point!(@count $($a)*) << 1 };
}

// Tests
#[cfg(test)]
mod tests {
    use crate::{point, vector};

    #[test]
    fn point_2d_creation() {
        assert_eq!(point!{ x: 1, y: 2 }.as_ref(), &vector![1, 2, 1]);
        assert_eq!(point!{ y: 2, x: 1 }.as_ref(), &vector![1, 2, 1]);
    }

    #[test]
    fn point_3d_creation() {
        assert_eq!(point!{ x: 1, y: 2, z: 3 }.as_ref(), &vector![1, 2, 3, 1]);
        assert_eq!(point!{ y: 2, x: 1, z: 3 }.as_ref(), &vector![1, 2, 3, 1]);
        assert_eq!(point!{ y: 2, z: 3, x: 1 }.as_ref(), &vector![1, 2, 3, 1]);
        assert_eq!(point!{ y: 2, z: 3, x: 1 }.as_ref(), &vector![1, 2, 3, 1]);
        assert_eq!(point!{ z: 3, y: 2, x: 1 }.as_ref(), &vector![1, 2, 3, 1]);
        assert_eq!(point!{ z: 3, y: 2, x: 1 }.as_ref(), &vector![1, 2, 3, 1]);
    }
}