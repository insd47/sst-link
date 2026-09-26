use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

/// Implements `sst_link::Links` for a `serde::Deserialize` struct.
///
/// ```ignore
/// #[derive(Deserialize, Links)]
/// #[serde(rename_all = "PascalCase")]
/// struct Resources {
///     key: sst_link::Secret,
/// }
/// ```
#[proc_macro_derive(Links)]
pub fn derive_links(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;
    let (implementation, arguments, clause) = input.generics.split_for_impl();

    quote! {
        impl #implementation ::sst_link::Links for #ident #arguments #clause {}
    }
    .into()
}
