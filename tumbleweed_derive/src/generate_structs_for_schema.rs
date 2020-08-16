use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use crate::diagnostic::Diagnostic;
use crate::schemas::entity_schema::{EntitySchema, FieldType};

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
    let entity_structs: Vec<proc_macro2::TokenStream> = schema
        .entities
        .iter()
        .map(|entity| {
            let entity_name = format_ident!("{}", entity.name);
            let fields: Vec<proc_macro2::TokenStream> = entity
                .fields
                .iter()
                .map(|field| {
                    // TODO: field_name should be UpperCased, not snake_cased
                    let field_name = format_ident!("{}", field.field_name);
                    let field_type = format_ident!(
                        "{}",
                        match &field.field_type {
                            FieldType::BigInt => "i64",
                            FieldType::Int => "i32",
                            FieldType::JsonBlob => "serde_json::Value",
                            // TODO: generate conversion impls for password hashing
                            FieldType::PasswordHash => "String",
                            FieldType::String => "String",
                            // TODO: generate conversion impls for datetimes
                            FieldType::UtcTimestamp => "String",
                        }
                    );
                    quote!(pub #field_name: #field_type)
                })
                .collect();
            quote! {
                #[derive(Debug, Deserialize, PartialEq, Serialize)]
                pub struct #entity_name {
                    #(#fields),*
                }
            }
        })
        .collect();
    Ok(quote! {
        pub mod entities {
            extern crate serde;
            extern crate serde_derive;

            use serde_derive::{Deserialize, Serialize};

            #(#entity_structs)*
        }
    })
}
