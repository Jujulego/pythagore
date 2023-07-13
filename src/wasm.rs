#[cfg(feature = "wasm-point")]
mod point_2d;

#[cfg(feature = "wasm-point-real")]
mod point_real_2d;

#[cfg(feature = "wasm-vector")]
mod vector_2d;

#[cfg(feature = "wasm-vector-real")]
mod vector_real_2d;

#[cfg(feature = "wasm-point")]
pub use point_2d::Point2D;

#[cfg(feature = "wasm-point-real")]
pub use point_real_2d::PointReal2D;

#[cfg(feature = "wasm-vector")]
pub use vector_2d::Vector2D;

#[cfg(feature = "wasm-vector-real")]
pub use vector_real_2d::VectorReal2D;
