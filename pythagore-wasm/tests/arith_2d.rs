#![cfg(target_arch = "wasm32")]

extern crate pythagore_wasm;
extern crate wasm_bindgen_test;

use pythagore::{force, point};
use pythagore_wasm::force_2d::Force2D;
use pythagore_wasm::point_2d::Point2D;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn test_force_add_force() {
    let u = Force2D::new(1.0, 2.0);
    let v = Force2D::new(3.0, 4.0);

    assert_eq!(u.add(&v), force![4.0, 6.0].into());
}

#[wasm_bindgen_test]
pub fn test_point_add_force() {
    let a = Point2D::new(1.0, 2.0);
    let u = Force2D::new(3.0, 4.0);

    assert_eq!(a.add_force(&u), point![4.0, 6.0].into());
}

#[wasm_bindgen_test]
pub fn test_force_sub_force() {
    let u = Force2D::new(1.0, 2.0);
    let v = Force2D::new(3.0, 4.0);

    assert_eq!(u.sub(&v), force![-2.0, -2.0].into());
}

#[wasm_bindgen_test]
pub fn test_point_sub_force() {
    let a = Point2D::new(1.0, 2.0);
    let u = Force2D::new(3.0, 4.0);

    assert_eq!(a.sub_force(&u), point![-2.0, -2.0].into());
}

#[wasm_bindgen_test]
pub fn test_point_sub_point() {
    let a = Point2D::new(1.0, 2.0);
    let b = Point2D::new(3.0, 4.0);

    assert_eq!(a.sub(&b), force![-2.0, -2.0].into());
}

#[wasm_bindgen_test]
pub fn test_force_dot_force() {
    let u = Force2D::new(1.0, 2.0);
    let v = Force2D::new(3.0, 4.0);

    assert_eq!(u.dot(&v), 11.0);
}

#[wasm_bindgen_test]
pub fn test_force_dot_scalar() {
    let u = Force2D::new(1.0, 2.0);

    assert_eq!(u.dot_scalar(2.0), force![2.0, 4.0].into());
}

#[wasm_bindgen_test]
pub fn test_force_div_scalar() {
    let u = Force2D::new(1.0, 2.0);

    assert_eq!(u.div_scalar(2.0), force![0.5, 1.0].into());
}