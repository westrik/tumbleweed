extern crate wee_alloc;

use std::collections::HashMap;
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

// #[wasm_bindgen]
// pub fn get_value() -> TestData {
//     let mut data_mapping = Map::new();
//     data_mapping.insert("Hello", "World");
//     let mut test_data = TestData::new();
//     test_data.set_message("HEHE".to_string());
//     test_data.set_data(data_mapping);
//     test_data
// }

// - `wasm-bindgen` serializes Rust structs to JS classes.
//
// - `serde-wasm-bindgen` serializes structs to POJOs, which seems more ergonomic.
//   It also handles conversions between Rust HashMap<> and JS Map<>, etc.
//
// - The main downside to using `serde-wasm-bindgen` is that we need to return `JsValue` instead of
//   the type to be serialized. This makes the generated types for the wasm bundle practically
//   useless, since all serialized structs are casted to `any`.
//
// I think I like `serde-wasm-bindgen` because it supports modern JS data structures.
// I may need to generate TypeScript types separately. (This may be complex).

#[wasm_bindgen]
pub fn get_value() -> JsValue {
    let mut data = HashMap::new();
    data.insert("Hello".to_string(), "World".to_string());
    let test_data = TestData {
        message: "HEHE".to_string(),
        data,
    };
    serde_wasm_bindgen::to_value(&test_data).unwrap()
}
