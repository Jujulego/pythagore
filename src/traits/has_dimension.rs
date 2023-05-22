pub trait HasDimension {
    const DIMENSION: usize;

    /// Returns object's dimension
    fn dimension(&self) -> usize;
}