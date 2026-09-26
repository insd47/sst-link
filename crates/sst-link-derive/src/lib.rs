use proc_macro::TokenStream;
use syn::{DeriveInput, Error, parse_macro_input};

mod links;

/// Derives `sst_link::Links` for a struct with named fields.
///
/// ```ignore
/// #[derive(Links)]
/// struct Resources {
///     key: sst_link::Secret,          // link "Key"
///     #[links(name = "RouterStorage")]
///     storage: sst_link::Bucket,
/// }
/// ```
///
/// - Each field reads the link named after it in PascalCase (`registry_key` → `RegistryKey`).
/// - `#[links(name = "…")]` overrides the name for one field.
/// - Field types only need `serde::Deserialize`.
#[proc_macro_derive(Links, attributes(links))]
pub fn derive_links(input: TokenStream) -> TokenStream {
    links::derive(parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
