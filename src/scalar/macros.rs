/// Builds a new scalar from given elements
///
/// ## Example
/// ```
/// use pythagore::{scalar, Scalar};
///
/// assert_eq!(scalar![1, 2, 3], Scalar::from([1, 2, 3]));
/// assert_eq!(scalar![1; 5], Scalar::from([1; 5]));
/// ```
#[macro_export]
macro_rules! scalar {
    [$elem:expr; $d:expr] => {
        scalar::Scalar::from([$elem; $d])
    };
    [$($x:expr),*] => {
        scalar::Scalar::from([$($x), +])
    };
}