use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

// TODO: generate structs from schema

// #[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct TestData {
    pub message: String,
    pub data: HashMap<String, String>,
}

// impl TestData {
//     pub fn new() -> Self {
//         TestData {
//             message: None,
//             data: None,
//         }
//     }
// }

// #[wasm_bindgen]
// impl TestData {
//     #[wasm_bindgen(getter)]
//     pub fn message(&self) -> Option<String> {
//         self.message.clone()
//     }
//
//     #[wasm_bindgen(setter)]
//     pub fn set_message(&mut self, message: String) {
//         self.message = Some(message);
//     }
//
//     #[wasm_bindgen(getter)]
//     pub fn data(&self) -> Option<HashMap<String, String>> {
//         self.data.clone()
//     }
//
//     #[wasm_bindgen(setter)]
//     pub fn set_data(&mut self, data: HashMap<String, String>) {
//         self.data = Some(data);
//     }
// }

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
