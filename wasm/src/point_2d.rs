use wasm_bindgen::prelude::wasm_bindgen;
use pythagore::{self as py, point};

#[wasm_bindgen]
pub struct Point2D {
    point: py::Point2D<f64>,
}

#[wasm_bindgen]
impl Point2D {
    /// Creates a new point from given coordinates
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Point2D {
        Point2D { point: point!(x, y) }
    }

    /// Creates a new origin point (same as `new Point2D(0, 0)`)
    pub fn origin() -> Point2D {
        Point2D { point: py::Point2D::origin() }
    }

    // Properties
    #[wasm_bindgen(getter)]
    pub fn x(&self) -> f64 {
        *self.point.x()
    }

    #[wasm_bindgen(setter)]
    pub fn set_x(&mut self, x: f64) {
        *self.point.x_mut() = x;
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> f64 {
        *self.point.y()
    }

    #[wasm_bindgen(setter)]
    pub fn set_y(&mut self, y: f64) {
        *self.point.y_mut() = y;
    }
}

// Utils
impl AsRef<py::Point2D<f64>> for Point2D {
    fn as_ref(&self) -> &py::Point2D<f64> {
        &self.point
    }
}

impl AsMut<py::Point2D<f64>> for Point2D {
    fn as_mut(&mut self) -> &mut py::Point2D<f64> {
        &mut self.point
    }
}

impl From<py::Point2D<f64>> for Point2D {
    fn from(point: py::Point2D<f64>) -> Self {
        Point2D { point }
    }
}
