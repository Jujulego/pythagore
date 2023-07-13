use std::borrow::{Borrow, BorrowMut};
use na::Vector2;
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(feature = "wasm-vector-real")]
use crate::wasm::VectorReal2D;

/// 2D vector defined in js
#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub struct VectorInt2D(Vector2<i64>);

#[wasm_bindgen]
impl VectorInt2D {
    // Statics
    /// Create a new vector from given scalars
    #[wasm_bindgen(constructor)]
    pub fn new(dx: i64, dy: i64) -> VectorInt2D {
        VectorInt2D(Vector2::new(dx, dy))
    }

    pub fn null() -> VectorInt2D {
        VectorInt2D(Vector2::zeros())
    }

    // Methods
    pub fn equals(&self, other: &VectorInt2D) -> bool {
        self.0 == other.0
    }

    pub fn add(&self, other: &VectorInt2D) -> VectorInt2D {
        VectorInt2D(self.0 + other.0)
    }

    pub fn sub(&self, other: &VectorInt2D) -> VectorInt2D {
        VectorInt2D(self.0 - other.0)
    }

    pub fn dot(&self, other: &VectorInt2D) -> i64 {
        self.0.dot(&other.0)
    }

    pub fn dot_scalar(&self, scalar: i64) -> VectorInt2D {
        VectorInt2D(self.0 * scalar)
    }

    pub fn div_scalar(&self, scalar: i64) -> VectorInt2D {
        VectorInt2D(self.0 / scalar)
    }

    // Properties
    #[wasm_bindgen(getter)]
    pub fn dx(&self) -> i64 {
        self.0.x
    }

    #[wasm_bindgen(setter)]
    pub fn set_dx(&mut self, dx: i64) {
        self.0.x = dx;
    }

    #[wasm_bindgen(getter)]
    pub fn dy(&self) -> i64 {
        self.0.y
    }

    #[wasm_bindgen(setter)]
    pub fn set_dy(&mut self, dy: i64) {
        self.0.y = dy;
    }

    #[wasm_bindgen(getter)]
    pub fn norm(&self) -> f64 {
        self.0.cast::<f64>().norm()
    }

    #[wasm_bindgen(getter)]
    pub fn norm_squared(&self) -> f64 {
        self.0.cast::<f64>().norm_squared()
    }

    #[cfg(feature = "wasm-vector-real")]
    #[wasm_bindgen(getter)]
    pub fn unit(&self) -> VectorReal2D {
        VectorReal2D::from(self.0.cast::<f64>().normalize())
    }
}

// Conversions
impl AsRef<Vector2<i64>> for VectorInt2D {
    fn as_ref(&self) -> &Vector2<i64> {
        &self.0
    }
}

impl AsMut<Vector2<i64>> for VectorInt2D {
    fn as_mut(&mut self) -> &mut Vector2<i64> {
        &mut self.0
    }
}

impl Borrow<Vector2<i64>> for VectorInt2D {
    fn borrow(&self) -> &Vector2<i64> {
        &self.0
    }
}

impl BorrowMut<Vector2<i64>> for VectorInt2D {
    fn borrow_mut(&mut self) -> &mut Vector2<i64> {
        &mut self.0
    }
}

impl From<Vector2<i64>> for VectorInt2D {
    fn from(value: Vector2<i64>) -> Self {
        VectorInt2D(value)
    }
}

// Operators
impl PartialEq for VectorInt2D {
    #[inline]
    fn eq(&self, other: &VectorInt2D) -> bool {
        self.equals(other)
    }
}
