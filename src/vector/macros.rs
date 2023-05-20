
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
