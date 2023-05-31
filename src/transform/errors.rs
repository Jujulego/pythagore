use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct InvalidLastColumnError;

impl Error for InvalidLastColumnError {}

impl Display for InvalidLastColumnError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "given matrix is not a valid transform matrix")
    }
}