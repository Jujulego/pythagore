/// Computes intersection between ranges
pub trait Intersection<Lhs = Self> {
    type Output;

    fn intersection(&self, lhs: &Lhs) -> Self::Output;
}

