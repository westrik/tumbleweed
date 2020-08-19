use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// TODO: generate structs from schema

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct TestData {
    message: Option<String>,
}

impl TestData {
    pub fn new() -> Self {
        TestData { message: None }
    }
}

#[wasm_bindgen]
impl TestData {
    #[wasm_bindgen(getter)]
    pub fn message(&self) -> Option<String> {
        self.message.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_message(&mut self, message: String) {
        self.message = Some(message);
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
