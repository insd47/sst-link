use heck::ToUpperCamelCase;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Error, Fields, LitStr, Result};

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
    let Data::Struct(data) = &input.data else {
        return Err(Error::new_spanned(
            &input.ident,
            "`Links` can only be derived for structs",
        ));
    };

    let Fields::Named(fields) = &data.fields else {
        return Err(Error::new_spanned(
            &input.ident,
            "`Links` needs a struct with named fields",
        ));
    };

    let reads = fields
        .named
        .iter()
        .map(|field| {
            let ident = field.ident.as_ref().expect("named fields have identifiers");
            let name = name(field)?.unwrap_or_else(|| ident.to_string().trim_start_matches("r#").to_upper_camel_case());

            Ok(quote! { #ident: source.get(#name)? })
        })
        .collect::<Result<Vec<_>>>()?;

    let ident = &input.ident;
    let (implementation, arguments, clause) = input.generics.split_for_impl();

    Ok(quote! {
        impl #implementation ::sst_link::Links for #ident #arguments #clause {
            fn load() -> ::core::result::Result<Self, ::sst_link::Error> {
                let source = ::sst_link::__private::Source::init()?;

                ::core::result::Result::Ok(Self { #(#reads),* })
            }
        }
    })
}

fn name(field: &syn::Field) -> Result<Option<String>> {
    let mut name = None;

    for attribute in field
        .attrs
        .iter()
        .filter(|attribute| attribute.path().is_ident("links"))
    {
        attribute.parse_nested_meta(|meta| {
            if !meta.path.is_ident("name") {
                return Err(meta.error("expected `name = \"…\"`"));
            }

            name = Some(meta.value()?.parse::<LitStr>()?.value());

            Ok(())
        })?;
    }

    Ok(name)
}
