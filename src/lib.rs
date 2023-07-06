extern crate nalgebra as na;

pub mod bbox;
pub mod bbox_walker;
pub mod traits;

#[cfg(feature = "wasm")]
pub mod wasm;

pub use bbox::BBox;
pub use bbox_walker::BBoxWalker;
pub use traits::{Holds, Intersection, IsRangeEmpty, PointBounds, Walkable};
