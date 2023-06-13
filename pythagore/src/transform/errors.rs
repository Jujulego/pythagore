use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct InvalidTransformMatrixError;

impl Error for InvalidTransformMatrixError {}

impl Display for InvalidTransformMatrixError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "given matrix is not a valid transform matrix")
    }
}