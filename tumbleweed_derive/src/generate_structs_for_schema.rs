use proc_macro2;
use quote::ToTokens;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use syn;

use crate::diagnostic::Diagnostic;
use crate::schemas::entity_schema::{Entity, EntitySchema, Field, FieldType};

fn load_schema_from_path(path: String) -> Result<EntitySchema, Diagnostic> {
    let schema_path = if path.is_empty() {
        Err(Diagnostic::error("Path for schema file not specified"))
    } else {
        Ok(path.replace("\"", ""))
    }?;
    let file =
        File::open(schema_path).map_err(|_| Diagnostic::error("Failed to open schema file"))?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader
        .read_to_string(&mut contents)
        .map_err(|_| Diagnostic::error("Failed to read schema file"))?;

    Ok(EntitySchema {
        authenticating_entities: None,
        entities: vec![Entity {
            name: "".to_string(),
            api_id_prefix: None,
            fields: vec![Field {
                field_name: "fake".to_string(),
                field_type: FieldType::BigInt,
                required: false,
            }],
        }],
    })
}

pub fn expand(path: String) -> Result<proc_macro2::TokenStream, Diagnostic> {
    let schema = load_schema_from_path(path)?;
    println!("{:#?}", schema);
    // TODO: parse `contents` as an `EntitySchema`
    //   then emit all the structs
    Ok(quote!())
}
