use std::borrow::{Borrow, BorrowMut};
use na::Vector2;
use wasm_bindgen::prelude::wasm_bindgen;

/// 2D vector defined in js
#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub struct VectorReal2D(Vector2<f64>);

#[wasm_bindgen]
impl VectorReal2D {
    // Statics
    /// Create a new vector from given scalars
    #[wasm_bindgen(constructor)]
    pub fn new(dx: f64, dy: f64) -> VectorReal2D {
        VectorReal2D(Vector2::new(dx, dy))
    }

    pub fn null() -> VectorReal2D {
        VectorReal2D(Vector2::zeros())
    }

    // Methods
    pub fn equals(&self, other: &VectorReal2D) -> bool {
        self.0 == other.0
    }

    pub fn add(&self, other: &VectorReal2D) -> VectorReal2D {
        VectorReal2D(self.0 + other.0)
    }

    pub fn sub(&self, other: &VectorReal2D) -> VectorReal2D {
        VectorReal2D(self.0 - other.0)
    }

    pub fn dot(&self, other: &VectorReal2D) -> f64 {
        self.0.dot(&other.0)
    }

    pub fn dot_scalar(&self, scalar: f64) -> VectorReal2D {
        VectorReal2D(self.0 * scalar)
    }

    pub fn div_scalar(&self, scalar: f64) -> VectorReal2D {
        VectorReal2D(self.0 / scalar)
    }

    // Properties
    #[wasm_bindgen(getter)]
    pub fn dx(&self) -> f64 {
        self.0[0]
    }

    #[wasm_bindgen(setter)]
    pub fn set_dx(&mut self, dx: f64) {
        self.0[0] = dx;
    }

    #[wasm_bindgen(getter)]
    pub fn dy(&self) -> f64 {
        self.0[1]
    }

    #[wasm_bindgen(setter)]
    pub fn set_dy(&mut self, dy: f64) {
        self.0[1] = dy;
    }

    #[wasm_bindgen(getter)]
    pub fn norm(&self) -> f64 {
        self.0.norm()
    }

    #[wasm_bindgen(getter)]
    pub fn norm_squared(&self) -> f64 {
        self.0.norm_squared()
    }

    #[wasm_bindgen(getter)]
    pub fn unit(&self) -> VectorReal2D {
        VectorReal2D(self.0.normalize())
    }
}

// Conversions
impl AsRef<Vector2<f64>> for VectorReal2D {
    fn as_ref(&self) -> &Vector2<f64> {
        &self.0
    }
}

impl AsMut<Vector2<f64>> for VectorReal2D {
    fn as_mut(&mut self) -> &mut Vector2<f64> {
        &mut self.0
    }
}

impl Borrow<Vector2<f64>> for VectorReal2D {
    fn borrow(&self) -> &Vector2<f64> {
        &self.0
    }
}

impl BorrowMut<Vector2<f64>> for VectorReal2D {
    fn borrow_mut(&mut self) -> &mut Vector2<f64> {
        &mut self.0
    }
}

impl From<Vector2<f64>> for VectorReal2D {
    fn from(value: Vector2<f64>) -> Self {
        VectorReal2D(value)
    }
}

// Operators
impl PartialEq for VectorReal2D {
    #[inline]
    fn eq(&self, other: &VectorReal2D) -> bool {
        self.equals(other)
    }
}
