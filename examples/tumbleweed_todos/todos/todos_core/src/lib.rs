use serde::{Deserialize, Serialize};

// TODO: generate structs from schema

#[derive(Serialize, Deserialize)]
pub struct TestData {
    pub message: String,
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
