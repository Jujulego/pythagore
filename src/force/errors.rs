use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct ForceMustEndWithZeroError;

impl Error for ForceMustEndWithZeroError {}

impl Display for ForceMustEndWithZeroError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "given vector is not a valid force, it must end with 0")
    }
}