/// Builds a new force from given elements
///
/// ## Example
/// ```
/// use pythagore::{force, vector};
///
/// // 2d force
/// assert_eq!(force!{ dx: 1, dy: 2 }.as_ref(), &vector![1, 2, 0]);
///
/// // 3d force
/// assert_eq!(force!{ dx: 1, dy: 2, dz: 3 }.as_ref(), &vector![1, 2, 3, 0]);
///
/// // Array based
/// assert_eq!(force![1; 3].as_ref(), &vector![1, 1, 1, 0]);
/// assert_eq!(force![1, 2, 3].as_ref(), &vector![1, 2, 3, 0]);
/// ```
#[macro_export]
macro_rules! force {
    { dx: $dx:expr, dy: $dy:expr } => {
        [$dx, $dy].iter().copied().collect::<force::Force<_, 3>>()
    };
    { dy: $dy:expr, dx: $dx:expr } => {
        force!{ dx: $dx, dy: $dy }
    };
    { dx: $dx:expr, dy: $dy:expr, dz: $dz:expr } => {
        [$dx, $dy, $dz].iter().copied().collect::<force::Force<_, 4>>()
    };
    { dy: $dy:expr, dx: $dx:expr, dz: $dz:expr } => {
        force!{ dx: $dx, dy: $dy, dz: $dz }
    };
    { dx: $dx:expr, dz: $dz:expr, dy: $dy:expr } => {
        force!{ dx: $dx, dy: $dy, dz: $dz }
    };
    { dy: $dy:expr, dz: $dz:expr, dx: $dx:expr } => {
        force!{ dx: $dx, dy: $dy, dz: $dz }
    };
    { dz: $dz:expr, dx: $dx:expr, dy: $dy:expr } => {
        force!{ dx: $dx, dy: $dy, dz: $dz }
    };
    { dz: $dz:expr, dy: $dy:expr, dx: $dx:expr } => {
        force!{ dx: $dx, dy: $dy, dz: $dz }
    };
    [$elem:expr; $d:expr] => {
        [$elem; $d].iter().copied().collect::<force::Force<_, { $d + 1 }>>()
    };
    [$($dx:expr),*] => {
        [$($dx), +].iter().copied().collect::<force::Force<_, { (force!(@count $($dx)+)) + 1 }>>()
    };
    (@count) => { 0 };
    (@count $odd:tt $($a:tt $b:tt)*) => { (force!(@count $($a)*) << 1) | 1 };
    (@count $($a:tt $even:tt)*) => { force!(@count $($a)*) << 1 };
}

// Tests
#[cfg(test)]
mod tests {
    use crate::{force, vector};

    #[test]
    fn force_2d_creation() {
        assert_eq!(force!{ dx: 1, dy: 2 }.as_ref(), &vector![1, 2, 0]);
        assert_eq!(force!{ dy: 2, dx: 1 }.as_ref(), &vector![1, 2, 0]);
    }

    #[test]
    fn force_3d_creation() {
        assert_eq!(force!{ dx: 1, dy: 2, dz: 3 }.as_ref(), &vector![1, 2, 3, 0]);
        assert_eq!(force!{ dy: 2, dx: 1, dz: 3 }.as_ref(), &vector![1, 2, 3, 0]);
        assert_eq!(force!{ dy: 2, dz: 3, dx: 1 }.as_ref(), &vector![1, 2, 3, 0]);
        assert_eq!(force!{ dy: 2, dz: 3, dx: 1 }.as_ref(), &vector![1, 2, 3, 0]);
        assert_eq!(force!{ dz: 3, dy: 2, dx: 1 }.as_ref(), &vector![1, 2, 3, 0]);
        assert_eq!(force!{ dz: 3, dy: 2, dx: 1 }.as_ref(), &vector![1, 2, 3, 0]);
    }
}