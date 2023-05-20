use num_traits::{Num, Zero};
use crate::Scalar;

/// `Point<T>` structure for 2 dimension points
///
/// ## Usage
/// ```
/// use pythagore::*;
///
/// let a = point!{ x: 1, y: 2 };
/// let b = point!{ x: 1, y: 2 };
///
/// assert_eq!(a, b);
/// assert_eq!(Point::origin(), point!{ x: 0, y: 0 });
/// ```
pub struct Point<T: Copy + Num, const D: usize> {
    scalar: Scalar<T, D>,
}

pub type Point2D<T> = Point<T, 3>;
pub type Point3D<T> = Point<T, 4>;

// Methods
impl<T: Copy + Num, const D: usize> Point<T, D> {
    pub const DIMENSION: usize = D - 1;

    /// Returns point's dimension
    #[inline]
    pub const fn dimension(&self) -> usize {
        D - 1
    }

    /// Returns a null vector
    ///
    /// ## Example
    /// ```
    /// use pythagore::*;
    ///
    /// assert_eq!(Point::origin(), point!{ x: 0, y: 0 })
    /// ```
    pub fn origin() -> Self {
        let mut pt = Point { scalar: Scalar::zero() };
        pt.scalar[D - 1] = T::one();

        pt
    }

    pub fn is_null(&self) -> bool {
        self.scalar.is_zero()
    }
}

// Utils
macro_rules! point_from_array_impl {
    ($dim:literal) => {
        impl<T: Copy + Num> From<[T; { $dim - 1 }]> for Point<T, $dim> {
            fn from(value: [T; { $dim - 1 }]) -> Self {
                Scalar::from(value).into()
            }
        }
    };
}

point_from_array_impl!(3);
point_from_array_impl!(4);

macro_rules! point_from_scalar_impl {
    ($dim:literal) => {
        impl<T: Copy + Num> From<Scalar<T, { $dim - 1 }>> for Point<T, $dim> {
            fn from(value: Scalar<T, { $dim - 1 }>) -> Self {
                let mut scalar = Scalar::zero();

                for n in 0..$dim - 1 {
                    scalar[n] = value[n];
                }

                scalar[$dim - 1] = T::one();

                Point { scalar }
            }
        }
    };
}

point_from_scalar_impl!(3);
point_from_scalar_impl!(4);

// Macros
#[macro_export]
macro_rules! point {
    (x: $x:expr, y: $y:expr) => {
        Point::from([$x, $y, 1])
    };
    (y: $y:expr, x: $x:expr) => {
        Point::from([$x, $y, 1])
    };
    (x: $x:expr, y: $y:expr, z: $z:expr) => {
        Point::from([$x, $y, $z, 1])
    };
    (y: $y:expr, x: $x:expr, z: $z:expr) => {
        Point::from([$x, $y, $z, 1])
    };
    (x: $x:expr, z: $z:expr, y: $y:expr) => {
        Point::from([$x, $y, $z, 1])
    };
    (y: $y:expr, z: $z:expr, x: $x:expr) => {
        Point::from([$x, $y, $z, 1])
    };
    (z: $z:expr, x: $x:expr, y: $y:expr) => {
        Point::from([$x, $y, $z, 1])
    };
    (z: $z:expr, y: $y:expr, x: $x:expr) => {
        Point::from([$x, $y, $z, 1])
    };
    ($elem:expr; $d:expr) => {
        Point::from([$elem; $d])
    };
    ($($x:expr),*) => {
        Point::from([$($x), +, 1])
    };
}
