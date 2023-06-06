#![cfg(target_arch = "wasm32")]

extern crate pythagore_wasm;
extern crate wasm_bindgen_test;

use pythagore_wasm::force_2d::Force2D;
use wasm_bindgen_test::*;
use pythagore::force;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn test_force_add_force() {
    let u = Force2D::new(1.0, 2.0);
    let v = Force2D::new(3.0, 4.0);

    assert_eq!(u.add(&v), force![4.0, 6.0].into());
}