use wasm_bindgen::prelude::wasm_bindgen;
use pythagore::{self as py, force};

#[wasm_bindgen]
pub struct Force2D {
    force: py::Force2D<f64>,
}

#[wasm_bindgen]
impl Force2D {
    /// Creates a new force from given coordinates
    #[wasm_bindgen(constructor)]
    pub fn new(dx: f64, dy: f64) -> Force2D {
        Force2D { force: force![dx, dy] }
    }

    /// Creates a new null force (same as `new Force2D(0, 0)`)
    pub fn null() -> Force2D {
        Force2D { force: py::Force2D::null() }
    }

    // Properties
    #[wasm_bindgen(getter)]
    pub fn dx(&self) -> f64 {
        *self.force.dx()
    }

    #[wasm_bindgen(setter)]
    pub fn set_dx(&mut self, x: f64) {
        *self.force.dx_mut() = x;
    }

    #[wasm_bindgen(getter)]
    pub fn dy(&self) -> f64 {
        *self.force.dy()
    }

    #[wasm_bindgen(setter)]
    pub fn set_dy(&mut self, y: f64) {
        *self.force.dy_mut() = y;
    }
}