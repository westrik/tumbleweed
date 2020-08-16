extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;

mod diagnostic;
mod entity;
mod schemas;

/// Implements `Model`
///
/// This trait is automatically derived...
///
/// ```ignore
/// #[derive(Entity)]
/// struct CustomEntity;
/// ```
///
/// This should be a function-like proc macro, not a derive macro.
///
///  entity! {
///      User {
///         id -> BigInt,
///         api_id -> String,
///         created_at -> UtcTimestamp,
///         updated_at -> UtcTimestamp,
///         name -> String,
///         password_hash -> HashedPassword,
///      }
///  }
///
///  Alternatively, entity definitions could be loaded from a config file (e.g. TOML)
///
/// load_entity_definitions!("./schema/entities.toml")
///
#[proc_macro_derive(Entity)]
pub fn derive_model(input: TokenStream) -> TokenStream {
    expand_proc_macro(input, entity::derive)
}

fn expand_proc_macro<T: syn::parse::Parse>(
    input: TokenStream,
    f: fn(T) -> Result<proc_macro2::TokenStream, diagnostic::Diagnostic>,
) -> TokenStream {
    let item = syn::parse(input).unwrap();
    match f(item) {
        Ok(x) => x.into(),
        Err(e) => {
            e.emit();
            "".parse().unwrap()
        }
    }
}
