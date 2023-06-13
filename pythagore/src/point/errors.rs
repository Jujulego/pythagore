use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct PointMustEndWithOneError;

impl Error for PointMustEndWithOneError {}

impl Display for PointMustEndWithOneError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "given vector is not a valid point, it must end with 1")
    }
}