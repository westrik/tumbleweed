use proc_macro2;
use syn;

use crate::diagnostic::Diagnostic;

pub fn derive(item: syn::DeriveInput) -> Result<proc_macro2::TokenStream, Diagnostic> {
    Ok(quote!())
}
