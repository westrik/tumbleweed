use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use crate::diagnostic::Diagnostic;
use crate::schemas::entity_schema::EntitySchema;

fn load_schema_from_path(path: String) -> Result<EntitySchema, Diagnostic> {
    let schema_path = if path.is_empty() {
        Err(Diagnostic::error("Path for schema file not specified"))
    } else {
        Ok(path.replace("\"", ""))
    }?;
    let file = File::open(schema_path)
        .map_err(|err| Diagnostic::error(format!("Failed to open schema file: {}", err)))?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader
        .read_to_string(&mut contents)
        .map_err(|err| Diagnostic::error(format!("Failed to read schema file: {}", err)))?;

    Ok(EntitySchema::from_str(&contents)?)
}

pub fn expand(path: String) -> Result<proc_macro2::TokenStream, Diagnostic> {
    let schema = load_schema_from_path(path)?;
    println!("{:#?}", schema);
    // TODO: parse `contents` as an `EntitySchema`
    //   then emit all the structs
    Ok(quote!())
}
