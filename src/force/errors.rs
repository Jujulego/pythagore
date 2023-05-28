use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct DoesNotEndWithZeroError;

impl Display for DoesNotEndWithZeroError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Given vector is not a valid force, it must end with 0")
    }
}