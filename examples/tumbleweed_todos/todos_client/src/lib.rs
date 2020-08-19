extern crate wee_alloc;

use tumbleweed_todos::*;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// TODO: reduce bundle size by:
//  - removing panic infrastructure
//  - wasm-opt
//  - serde-wasm-bindgen
//  - ...?

// OQ: how should this be organized? We'll need wrappers for WASM and native
// - add proc macro to wrap with correct fn & type depending on arch?

// TODO: add #[wasm_bindgen] to all exported structs

#[wasm_bindgen]
pub fn get_value() -> TestData {
    let mut test_data = TestData::new();
    test_data.set_message("HEHE".to_string());
    test_data
}
