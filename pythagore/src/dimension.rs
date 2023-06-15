/// Defines supported dimensions
pub trait SupportedDimension {}

/// Specifies object dimension
#[repr(transparent)]
pub struct Dimension<const D: usize>;

impl SupportedDimension for Dimension<2> {}
impl SupportedDimension for Dimension<3> {}
impl SupportedDimension for Dimension<4> {}