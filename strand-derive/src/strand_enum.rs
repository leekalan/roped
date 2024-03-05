use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Meta, Type};

use crate::{build_error::BuildError, meta_map::collect_meta_map, search_meta::search_meta};

pub fn strand_derive_enum(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let name = &input.ident;

    let strand_meta = search_meta(input.attrs.iter().map(|s| &s.meta), "strand").ok_or(
        syn::Error::new_spanned(&input, "expected attribute, \"#[strand(..)]\""),
    )?;

    let meta_list = match strand_meta {
        syn::Meta::List(v) => v.parse_args_with(
            syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated,
        )?,
        _ => {
            return Err(syn::Error::new_spanned(
                strand_meta,
                "expected list, \"#[strand(..)]\"",
            ))
        }
    };

    let meta_map = collect_meta_map(meta_list, &[("state", false), ("input", false)])?;

    let state: Type = match meta_map.get("state") {
        Some(m) => match m {
            Meta::NameValue(n) => syn::parse(n.value.to_token_stream().into())?,
            _ => {
                return Err(syn::Error::new_spanned(
                    m,
                    "expected type, \"state(<type>)\"",
                ))
            }
        },
        None => syn::parse(quote::quote!(roped::base_types::EmptyState).into())?,
    };

    let input: Type = match meta_map.get("input") {
        Some(m) => match m {
            Meta::NameValue(n) => syn::parse(n.value.to_token_stream().into())?,
            _ => {
                return Err(syn::Error::new_spanned(
                    m,
                    "expected type, \"input(<type>)\"",
                ))
            }
        },
        None => syn::parse(quote::quote!(&str).into())?,
    };

    let gen = quote::quote! {
        pub fn #name() -> #state where #input : parsr::parse::Parse {
            todo!()
        }
    };

    Ok(gen)
}
