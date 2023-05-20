
#[macro_export]
macro_rules! point {
    (x: $x:expr, y: $y:expr) => {
        point::Point::from([$x, $y])
    };
    (y: $y:expr, x: $x:expr) => {
        point::Point::from([$x, $y])
    };
    (x: $x:expr, y: $y:expr, z: $z:expr) => {
        point::Point::from([$x, $y, $z])
    };
    (y: $y:expr, x: $x:expr, z: $z:expr) => {
        point::Point::from([$x, $y, $z])
    };
    (x: $x:expr, z: $z:expr, y: $y:expr) => {
        point::Point::from([$x, $y, $z])
    };
    (y: $y:expr, z: $z:expr, x: $x:expr) => {
        point::Point::from([$x, $y, $z])
    };
    (z: $z:expr, x: $x:expr, y: $y:expr) => {
        point::Point::from([$x, $y, $z])
    };
    (z: $z:expr, y: $y:expr, x: $x:expr) => {
        point::Point::from([$x, $y, $z])
    };
    ($elem:expr; $d:expr) => {
        point::Point::from([$elem; $d])
    };
    ($($x:expr),*) => {
        point::Point::from([$($x), +])
    };
}
