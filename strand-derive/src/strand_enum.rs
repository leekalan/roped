use proc_macro2::TokenStream;
use syn::Meta;

use crate::{meta_map::collect_meta_map, search_meta::search_meta};

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

    let meta_map = collect_meta_map(meta_list, &["state", "input"])?;

    let gen = quote::quote! {
        //Empty
    };

    Ok(gen)
}
