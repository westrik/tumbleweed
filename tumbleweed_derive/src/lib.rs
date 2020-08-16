extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
// #[macro_use]
extern crate syn;

use proc_macro::TokenStream;

mod diagnostic;
mod generate_structs_for_schema;
mod schemas;

/// Implements the `generate_structs_for_schemas!()` procedural macro.
///
/// This automatically derives struct definitions for entities specified in a TOML schema config.
///
/// ```ignore
/// load_entity_definitions!("schema.toml")
/// ```
///
#[proc_macro]
pub fn generate_structs_for_schema(input: TokenStream) -> TokenStream {
    match generate_structs_for_schema::expand(input.to_string()) {
        Ok(x) => x.into(),
        Err(e) => {
            e.emit();
            "".parse().unwrap()
        }
    }
}

// fn expand_proc_macro<T: syn::parse::Parse>(
//     input: TokenStream,
//     f: fn(T) -> Result<proc_macro2::TokenStream, diagnostic::Diagnostic>,
// ) -> TokenStream {
//     let item = syn::parse(input).unwrap();
//     match f(item) {
//         Ok(x) => x.into(),
//         Err(e) => {
//             e.emit();
//             "".parse().unwrap()
//         }
//     }
// }
