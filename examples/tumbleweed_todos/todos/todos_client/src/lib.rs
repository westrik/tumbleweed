use tumbleweed_todos_core::*;
use wasm_bindgen::prelude::*;

// OQ: how should this be organized? We'll need wrappers for WASM and native
// - add proc macro to wrap with correct fn & type depending on arch?

#[wasm_bindgen]
pub fn get_value() -> JsValue {
    let test_data = TestData {
        message: "hehe".to_string(),
    };
    JsValue::from_serde(&test_data).unwrap()
}

// #[wasm_bindgen]
// pub fn transform_value(value: &JsValue) -> JsValue {
//
// }

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
