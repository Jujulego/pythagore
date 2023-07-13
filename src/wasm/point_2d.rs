use std::borrow::{Borrow, BorrowMut};
use na::{point, Point2};
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(feature = "wasm-vector")]
use crate::wasm::vector_2d::Vector2D;

/// 2D point defined in js
#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub struct Point2D(Point2<i64>);

#[wasm_bindgen]
impl Point2D {
    // Statics
    /// Creates a new point from given coordinates
    #[wasm_bindgen(constructor)]
    pub fn new(x: i64, y: i64) -> Point2D {
        Point2D(point![x, y])
    }

    /// Creates a new origin point (same as `new Point2D(0, 0)`)
    pub fn origin() -> Point2D {
        Point2D(Point2::origin())
    }

    // Methods
    pub fn equals(&self, other: &Point2D) -> bool {
        self.0 == other.0
    }

    #[cfg(feature = "wasm-vector")]
    pub fn add_vector(&self, vector: &Vector2D) -> Point2D {
        Point2D(self.0 + vector.as_ref())
    }

    #[cfg(feature = "wasm-vector")]
    pub fn sub(&self, other: &Point2D) -> Vector2D {
        Vector2D::from(self.0 - other.0)
    }

    #[cfg(feature = "wasm-vector")]
    pub fn sub_vector(&self, vector: &Vector2D) -> Point2D {
        Point2D(self.0 - vector.as_ref())
    }

    // Properties
    #[wasm_bindgen(getter)]
    pub fn x(&self) -> i64 {
        self.0.x
    }

    #[wasm_bindgen(setter)]
    pub fn set_x(&mut self, x: i64) {
        self.0.x = x;
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> i64 {
        self.0.y
    }

    #[wasm_bindgen(setter)]
    pub fn set_y(&mut self, y: i64) {
        self.0.y = y;
    }
}

// Conversions
impl AsRef<Point2<i64>> for Point2D {
    fn as_ref(&self) -> &Point2<i64> {
        &self.0
    }
}

impl AsMut<Point2<i64>> for Point2D {
    fn as_mut(&mut self) -> &mut Point2<i64> {
        &mut self.0
    }
}

impl Borrow<Point2<i64>> for Point2D {
    fn borrow(&self) -> &Point2<i64> {
        &self.0
    }
}

impl BorrowMut<Point2<i64>> for Point2D {
    fn borrow_mut(&mut self) -> &mut Point2<i64> {
        &mut self.0
    }
}

impl From<Point2<i64>> for Point2D {
    fn from(value: Point2<i64>) -> Self {
        Point2D(value)
    }
}

// Operators
impl PartialEq for Point2D {
    #[inline]
    fn eq(&self, other: &Point2D) -> bool {
        self.equals(other)
    }
}

impl PartialEq<Point2<i64>> for Point2D {
    #[inline]
    fn eq(&self, other: &Point2<i64>) -> bool {
        &self.0 == other
    }
}
