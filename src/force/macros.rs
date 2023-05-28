/// Builds a new force from given elements
///
/// ## Example
/// ```
/// use pythagore::{force, Force};
///
/// // 2d force
/// assert_eq!(force!{ dx: 1, dy: 2 }, Force::from(&[1, 2]));
///
/// // 3d force
/// assert_eq!(force!{ dx: 1, dy: 2, dz: 3 }, Force::from(&[1, 2, 3]));
///
/// // Array based
/// assert_eq!(force![1; 3], Force::from(&[1, 1, 1]));
/// assert_eq!(force![1, 2, 3], Force::from(&[1, 2, 3]));
/// ```
#[macro_export]
macro_rules! force {
    { dx: $x:expr, dy: $y:expr } => {
        force::Force::from(&[$x, $y])
    };
    { dy: $y:expr, dx: $x:expr } => {
        force::Force::from(&[$x, $y])
    };
    { dx: $x:expr, dy: $y:expr, dz: $z:expr } => {
        force::Force::from(&[$x, $y, $z])
    };
    { dy: $y:expr, dx: $x:expr, dz: $z:expr } => {
        force::Force::from(&[$x, $y, $z])
    };
    { dx: $x:expr, dz: $z:expr, dy: $y:expr } => {
        force::Force::from(&[$x, $y, $z])
    };
    { dy: $y:expr, dz: $z:expr, dx: $x:expr } => {
        force::Force::from(&[$x, $y, $z])
    };
    { dz: $z:expr, dx: $x:expr, dy: $y:expr } => {
        force::Force::from(&[$x, $y, $z])
    };
    { dz: $z:expr, dy: $y:expr, dx: $x:expr } => {
        force::Force::from(&[$x, $y, $z])
    };
    [$elem:expr; $d:expr] => {
        force::Force::from(&[$elem; $d])
    };
    [$($x:expr),*] => {
        force::Force::from(&[$($x), +])
    };
}

// Tests
#[cfg(test)]
mod tests {
    use crate::{force, Force};

    #[test]
    fn force_2d_creation() {
        assert_eq!(force! { dx: 1, dy: 2 }, Force::from(&[1, 2]));
        assert_eq!(force! { dy: 2, dx: 1 }, Force::from(&[1, 2]));
    }

    #[test]
    fn force_3d_creation() {
        assert_eq!(force!{ dx: 1, dy: 2, dz: 3 }, Force::from(&[1, 2, 3]));
        assert_eq!(force!{ dy: 2, dx: 1, dz: 3 }, Force::from(&[1, 2, 3]));
        assert_eq!(force!{ dy: 2, dz: 3, dx: 1 }, Force::from(&[1, 2, 3]));
        assert_eq!(force!{ dy: 2, dz: 3, dx: 1 }, Force::from(&[1, 2, 3]));
        assert_eq!(force!{ dz: 3, dy: 2, dx: 1 }, Force::from(&[1, 2, 3]));
        assert_eq!(force!{ dz: 3, dy: 2, dx: 1 }, Force::from(&[1, 2, 3]));
    }
}