/// Builds a new matrix from given elements
///
/// ## Example
/// ```
/// use pythagore::{matrix, Matrix};
///
/// assert_eq!(
///     matrix![[1; 3]; 3],
///     Matrix::from([
///         [1, 1, 1],
///         [1, 1, 1],
///         [1, 1, 1],
///     ])
/// );
///
/// assert_eq!(
///     matrix![[1, 2, 3]; 3],
///     Matrix::from([
///         [1, 2, 3],
///         [1, 2, 3],
///         [1, 2, 3],
///     ])
/// );
///
/// assert_eq!(
///     matrix![
///         [1; 3],
///         [2; 3],
///         [3; 3],
///     ],
///     Matrix::from([
///         [1, 1, 1],
///         [2, 2, 2],
///         [3, 3, 3],
///     ])
/// );
///
/// assert_eq!(
///     matrix![
///         [1, 2, 3],
///         [4, 5, 6],
///         [7, 8, 9],
///     ],
///     Matrix::from([
///         [1, 2, 3],
///         [4, 5, 6],
///         [7, 8, 9],
///     ])
/// );
/// ```
#[macro_export]
macro_rules! matrix {
    [[$elem:expr; $c:expr]; $l:expr] => {
        matrix::Matrix::from([[$elem; $c]; $l])
    };
    [$([$elem:expr; $c:expr]),+$(,)?] => {
        matrix::Matrix::from([$([$elem; $c]),+])
    };
    [[$($x:expr),+]; $l:expr] => {
        matrix::Matrix::from([[$($x),+]; $l])
    };
    [$([$($x:expr),+]),+$(,)?] => {
        matrix::Matrix::from([$([$($x),+]),+])
    };
}