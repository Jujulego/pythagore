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

    pub fn eq(&self, other: &Force2D) -> bool {
        self.force == other.force
    }

    pub fn add(&self, other: &Force2D) -> Force2D {
        (self.force + other.force).into()
    }

    pub fn sub(&self, other: &Force2D) -> Force2D {
        (self.force - other.force).into()
    }

    pub fn dot(&self, other: &Force2D) -> f64 {
        self.force * other.force
    }

    pub fn dot_scalar(&self, scalar: f64) -> Force2D {
        (self.force * scalar).into()
    }

    pub fn div_scalar(&self, scalar: f64) -> Force2D {
        (self.force / scalar).into()
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

    #[wasm_bindgen(getter)]
    pub fn norm(&self) -> f64 {
        self.force.norm()
    }

    #[wasm_bindgen(getter)]
    pub fn manhattan_norm(&self) -> f64 {
        self.force.manhattan_norm()
    }

    #[wasm_bindgen(getter)]
    pub fn square_norm(&self) -> f64 {
        self.force.square_norm()
    }

    #[wasm_bindgen(getter)]
    pub fn unit(&self) -> Force2D {
        self.force.unit().into()
    }
}

// Utils
impl AsRef<py::Force2D<f64>> for Force2D {
    fn as_ref(&self) -> &py::Force2D<f64> {
        &self.force
    }
}

impl AsMut<py::Force2D<f64>> for Force2D {
    fn as_mut(&mut self) -> &mut py::Force2D<f64> {
        &mut self.force
    }
}

impl From<py::Force2D<f64>> for Force2D {
    fn from(force: py::Force2D<f64>) -> Self {
        Force2D { force }
    }
}