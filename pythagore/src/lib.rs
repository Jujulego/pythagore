extern crate nalgebra as na;

pub mod bbox;
pub mod traits;

#[cfg(feature = "wasm")]
pub mod wasm;

pub use bbox::{BBox};