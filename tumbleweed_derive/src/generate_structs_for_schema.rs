use proc_macro2;
use quote::ToTokens;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use syn;

use crate::diagnostic::Diagnostic;

pub fn expand(path: String) -> Result<proc_macro2::TokenStream, Diagnostic> {
    let schema_path = if path.is_empty() {
        panic!("Invalid path to schema file")
    } else {
        path.replace("\"", "")
    };
    let file =
        File::open(schema_path).map_err(|_| Diagnostic::error("Failed to open schema file"))?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader
        .read_to_string(&mut contents)
        .map_err(|_| Diagnostic::error("Failed to read schema file"))?;
    println!("{}", contents);
    Ok(quote!())
}
