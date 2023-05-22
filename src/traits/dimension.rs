pub trait Dimension<const D: usize> {
    /// Returns object's dimension
    #[inline]
    fn dimension() -> usize {
        D
    }
}