use na::{vector, Vector2};
use wasm_bindgen::prelude::wasm_bindgen;

/// 2D vector defined in js
#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub struct Vector2D(Vector2<i64>);

#[wasm_bindgen]
impl Vector2D {
    // Statics
    /// Create a new vector from given scalars
    #[wasm_bindgen(constructor)]
    pub fn new(dx: i64, dy: i64) -> Vector2D {
        Vector2D(vector![dx, dy])
    }

    pub fn null() -> Vector2D {
        Vector2D(Vector2::zeros())
    }

    // Methods
    pub fn equals(&self, other: &Vector2D) -> bool {
        self.0 == other.0
    }

    pub fn add(&self, other: &Vector2D) -> Vector2D {
        Vector2D(self.0 + other.0)
    }

    pub fn sub(&self, other: &Vector2D) -> Vector2D {
        Vector2D(self.0 - other.0)
    }

    pub fn dot(&self, other: &Vector2D) -> i64 {
        self.0.dot(&other.0)
    }

    pub fn dot_scalar(&self, scalar: i64) -> Vector2D {
        Vector2D(self.0 * scalar)
    }

    pub fn div_scalar(&self, scalar: i64) -> Vector2D {
        Vector2D(self.0 / scalar)
    }

    // Properties
    #[wasm_bindgen(getter)]
    pub fn dx(&self) -> i64 {
        self.0[0]
    }

    #[wasm_bindgen(setter)]
    pub fn set_dx(&mut self, dx: i64) {
        self.0[0] = dx;
    }

    #[wasm_bindgen(getter)]
    pub fn dy(&self) -> i64 {
        self.0[1]
    }

    #[wasm_bindgen(setter)]
    pub fn set_dy(&mut self, dy: i64) {
        self.0[1] = dy;
    }
}

// Conversions
impl AsRef<Vector2<i64>> for Vector2D {
    fn as_ref(&self) -> &Vector2<i64> {
        &self.0
    }
}

impl AsMut<Vector2<i64>> for Vector2D {
    fn as_mut(&mut self) -> &mut Vector2<i64> {
        &mut self.0
    }
}

impl From<Vector2<i64>> for Vector2D {
    fn from(value: Vector2<i64>) -> Self {
        Vector2D(value)
    }
}

// Operators
impl PartialEq for Vector2D {
    #[inline]
    fn eq(&self, other: &Vector2D) -> bool {
        self.equals(other)
    }
}
