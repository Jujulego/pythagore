use std::borrow::{Borrow, BorrowMut};
use na::Point2;
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(feature = "wasm-vector-int")]
use crate::wasm::vector_int_2d::VectorInt2D;

/// 2D point defined in js
#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub struct PointInt2D(Point2<i32>);

#[wasm_bindgen]
impl PointInt2D {
    // Statics
    /// Creates a new point from given coordinates
    #[wasm_bindgen(constructor)]
    pub fn new(x: i32, y: i32) -> PointInt2D {
        PointInt2D(Point2::new(x, y))
    }

    /// Creates a new origin point (same as `new Point2D(0, 0)`)
    pub fn origin() -> PointInt2D {
        PointInt2D(Point2::origin())
    }

    // Methods
    pub fn equals(&self, other: &PointInt2D) -> bool {
        self.0 == other.0
    }

    #[cfg(feature = "wasm-vector-int")]
    pub fn add_vector(&self, vector: &VectorInt2D) -> PointInt2D {
        PointInt2D(self.0 + vector.as_ref())
    }

    #[cfg(feature = "wasm-vector-int")]
    pub fn sub(&self, other: &PointInt2D) -> VectorInt2D {
        VectorInt2D::from(self.0 - other.0)
    }

    #[cfg(feature = "wasm-vector-int")]
    pub fn sub_vector(&self, vector: &VectorInt2D) -> PointInt2D {
        PointInt2D(self.0 - vector.as_ref())
    }

    // Properties
    #[wasm_bindgen(getter)]
    pub fn x(&self) -> i32 {
        self.0.x
    }

    #[wasm_bindgen(setter)]
    pub fn set_x(&mut self, x: i32) {
        self.0.x = x;
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> i32 {
        self.0.y
    }

    #[wasm_bindgen(setter)]
    pub fn set_y(&mut self, y: i32) {
        self.0.y = y;
    }
}

// Conversions
impl AsRef<Point2<i32>> for PointInt2D {
    fn as_ref(&self) -> &Point2<i32> {
        &self.0
    }
}

impl AsMut<Point2<i32>> for PointInt2D {
    fn as_mut(&mut self) -> &mut Point2<i32> {
        &mut self.0
    }
}

impl Borrow<Point2<i32>> for PointInt2D {
    fn borrow(&self) -> &Point2<i32> {
        &self.0
    }
}

impl BorrowMut<Point2<i32>> for PointInt2D {
    fn borrow_mut(&mut self) -> &mut Point2<i32> {
        &mut self.0
    }
}

impl From<Point2<i32>> for PointInt2D {
    fn from(value: Point2<i32>) -> Self {
        PointInt2D(value)
    }
}

// Operators
impl PartialEq for PointInt2D {
    #[inline]
    fn eq(&self, other: &PointInt2D) -> bool {
        self.equals(other)
    }
}

impl PartialEq<Point2<i32>> for PointInt2D {
    #[inline]
    fn eq(&self, other: &Point2<i32>) -> bool {
        &self.0 == other
    }
}
