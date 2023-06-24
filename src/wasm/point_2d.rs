use crate::wasm::vector_2d::Vector2D;
use na::{point, Point2};
use wasm_bindgen::prelude::wasm_bindgen;

/// 2D point defined in js
#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub struct Point2D(Point2<f64>);

#[wasm_bindgen]
impl Point2D {
    // Statics
    /// Creates a new point from given coordinates
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Point2D {
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
    pub fn x(&self) -> f64 {
        self.0[0]
    }

    #[wasm_bindgen(setter)]
    pub fn set_x(&mut self, x: f64) {
        self.0[0] = x;
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> f64 {
        self.0[1]
    }

    #[wasm_bindgen(setter)]
    pub fn set_y(&mut self, y: f64) {
        self.0[1] = y;
    }
}

// Conversions
impl AsRef<Point2<f64>> for Point2D {
    fn as_ref(&self) -> &Point2<f64> {
        &self.0
    }
}

impl AsMut<Point2<f64>> for Point2D {
    fn as_mut(&mut self) -> &mut Point2<f64> {
        &mut self.0
    }
}

impl From<Point2<f64>> for Point2D {
    fn from(value: Point2<f64>) -> Self {
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
