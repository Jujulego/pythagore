use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct DoesNotEndWithOneError;

impl Display for DoesNotEndWithOneError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Given vector is not a valid point, it must end with 1")
    }
}