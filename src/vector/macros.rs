/// Builds a new vector from given elements
///
/// ## Example
/// ```
/// use pythagore::{vector, Vector};
///
/// assert_eq!(vector![1, 2, 3], Vector::from([1, 2, 3]));
/// assert_eq!(vector![1; 5], Vector::from([1; 5]));
/// ```
#[macro_export]
macro_rules! vector {
    [$elem:expr; $d:expr] => {
        vector::Vector::from([$elem; $d])
    };
    [$($x:expr),*] => {
        vector::Vector::from([$($x), +])
    };
}