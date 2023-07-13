#[cfg(feature = "wasm-point-real")]
mod point_real_2d;

#[cfg(feature = "wasm-point-int")]
mod point_int_2d;

#[cfg(feature = "wasm-vector-real")]
mod vector_real_2d;

#[cfg(feature = "wasm-vector-int")]
mod vector_int_2d;

#[cfg(feature = "wasm-point-real")]
pub use point_real_2d::PointReal2D;

#[cfg(feature = "wasm-point-int")]
pub use point_int_2d::PointInt2D;

#[cfg(feature = "wasm-vector-real")]
pub use vector_real_2d::VectorReal2D;

#[cfg(feature = "wasm-vector-int")]
pub use vector_int_2d::VectorInt2D;
