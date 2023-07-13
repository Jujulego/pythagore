use std::borrow::{Borrow, BorrowMut};
use na::{point, Point2};
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(feature = "wasm-vector-real")]
use crate::wasm::vector_real_2d::VectorReal2D;

/// 2D point defined in js
#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub struct PointReal2D(Point2<f64>);

#[wasm_bindgen]
impl PointReal2D {
    // Statics
    /// Creates a new point from given coordinates
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> PointReal2D {
        PointReal2D(point![x, y])
    }

    /// Creates a new origin point (same as `new Point2D(0, 0)`)
    pub fn origin() -> PointReal2D {
        PointReal2D(Point2::origin())
    }

    // Methods
    pub fn equals(&self, other: &PointReal2D) -> bool {
        self.0 == other.0
    }

    #[cfg(feature = "wasm-vector-real")]
    pub fn add_vector(&self, vector: &VectorReal2D) -> PointReal2D {
        PointReal2D(self.0 + vector.as_ref())
    }

    #[cfg(feature = "wasm-vector-real")]
    pub fn sub(&self, other: &PointReal2D) -> VectorReal2D {
        VectorReal2D::from(self.0 - other.0)
    }

    #[cfg(feature = "wasm-vector-real")]
    pub fn sub_vector(&self, vector: &VectorReal2D) -> PointReal2D {
        PointReal2D(self.0 - vector.as_ref())
    }

    // Properties
    #[wasm_bindgen(getter)]
    pub fn x(&self) -> f64 {
        self.0.x
    }

    #[wasm_bindgen(setter)]
    pub fn set_x(&mut self, x: f64) {
        self.0.x = x;
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> f64 {
        self.0.y
    }

    #[wasm_bindgen(setter)]
    pub fn set_y(&mut self, y: f64) {
        self.0.y = y;
    }
}

// Conversions
impl AsRef<Point2<f64>> for PointReal2D {
    fn as_ref(&self) -> &Point2<f64> {
        &self.0
    }
}

impl AsMut<Point2<f64>> for PointReal2D {
    fn as_mut(&mut self) -> &mut Point2<f64> {
        &mut self.0
    }
}

impl Borrow<Point2<f64>> for PointReal2D {
    fn borrow(&self) -> &Point2<f64> {
        &self.0
    }
}

impl BorrowMut<Point2<f64>> for PointReal2D {
    fn borrow_mut(&mut self) -> &mut Point2<f64> {
        &mut self.0
    }
}

impl From<Point2<f64>> for PointReal2D {
    fn from(value: Point2<f64>) -> Self {
        PointReal2D(value)
    }
}

// Operators
impl PartialEq for PointReal2D {
    #[inline]
    fn eq(&self, other: &PointReal2D) -> bool {
        self.equals(other)
    }
}

impl PartialEq<Point2<f64>> for PointReal2D {
    #[inline]
    fn eq(&self, other: &Point2<f64>) -> bool {
        &self.0 == other
    }
}
