use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{token::Token, Meta, Type};

use crate::{
    build_error::BuildError,
    meta_map::{self, collect_meta_map},
    search_meta::search_meta,
};

pub fn strand_derive_enum(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let name = &input.ident;

    let config = get_config(&input)?;

    let triggers = get_variants(&input)?;

    let catpures = construct_fields(triggers)?;

    let Config {
        state,
        input,
        error,
    } = config;

    let gen = quote::quote! {
        impl<'a> Strand<'a> for #name {
            type State = #state;
            type Input = #input;
            type Err = #error;

            fn run(
                state: &mut Self::State,
                input: Self::Input,
                ws_chars: MatchContainer<Self::Input, <Self::Input as MatcherStart>::Item>,
                index: usize,
            ) -> Result<(), roped::error::Error<Self::Input, Self::Err>> {
                #catpures
            }
        }
    };

    Ok(gen)
}

struct Config {
    state: Type,
    input: Type,
    error: Type,
}

fn get_config(input: &syn::DeriveInput) -> syn::Result<Config> {
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

    let meta_map = collect_meta_map(
        meta_list,
        &[("state", false), ("input", false), ("error", false)],
    )?;

    let state: Type = match meta_map.get("state") {
        Some(m) => match m {
            Meta::NameValue(n) => syn::parse(n.value.to_token_stream().into())?,
            _ => {
                return Err(syn::Error::new_spanned(
                    m,
                    "expected type, \"state = <type>\"",
                ))
            }
        },
        None => syn::parse_quote! { roped::base_types::EmptyState },
    };

    let input_raw: Type = match meta_map.get("input") {
        Some(m) => match m {
            Meta::NameValue(n) => syn::parse(n.value.to_token_stream().into())?,
            _ => {
                return Err(syn::Error::new_spanned(
                    m,
                    "expected type, \"input = <type>\"",
                ))
            }
        },
        None => syn::parse(quote::quote!(&str).into())?,
    };

    let input = match input_raw {
        Type::Reference(i) => {
            let ty = *i.elem;
            syn::parse_quote! { &'a #ty }
        }
        v => v,
    };

    let error: Type = match meta_map.get("error") {
        Some(m) => match m {
            Meta::NameValue(n) => syn::parse(n.value.to_token_stream().into())?,
            _ => {
                return Err(syn::Error::new_spanned(
                    m,
                    "expected type, \"input = <type>\"",
                ))
            }
        },
        None => syn::parse(quote::quote!(()).into())?,
    };

    Ok(Config {
        state,
        input,
        error,
    })
}

enum Triggers {
    Name(String),
    Prefix(String),
    NamePrefix(String, String),
}

fn get_variants(input: &syn::DeriveInput) -> syn::Result<(HashMap<Triggers, Type>, Option<Type>)> {
    let map: HashMap<Triggers, Type> = HashMap::new();

    let other: Option<Type> = None;

    let e = match &input.data {
        syn::Data::Enum(v) => v,
        _ => panic!("Internal Error, this is not supposed to happen"),
    };

    for variant in &e.variants {
        let strand_meta = search_meta(variant.attrs.iter().map(|s| &s.meta), "strand").ok_or(
            syn::Error::new_spanned(&variant, "expected attribute, \"#[strand(..)]\""),
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
    
        let meta_map = collect_meta_map(
            meta_list,
            &[("name", false), ("prefix", false), ("flag", false)],
        )?;

        
    }

    todo!()
}

fn construct_fields(
    triggers: (HashMap<Triggers, Type>, Option<Type>),
) -> syn::Result<proc_macro2::TokenStream> {
    todo!()
}
