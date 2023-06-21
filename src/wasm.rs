#[cfg(feature = "wasm-point")]
pub mod point_2d;

#[cfg(feature = "wasm-vector")]
pub mod vector_2d;

#[cfg(feature = "wasm-point")]
pub use point_2d::Point2D;

#[cfg(feature = "wasm-vector")]
pub use vector_2d::Vector2D;
