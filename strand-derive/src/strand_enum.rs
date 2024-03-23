use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{spanned::Spanned, Meta, Type};

use crate::{
    meta_map::collect_meta_map,
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

enum Trigger {
    None,
    Name(String),
    Prefix(String),
    NamePrefix(String, String),
}
impl Trigger {
    pub fn build(self, trigger: Trigger, span: proc_macro2::Span) -> syn::Result<Self> {
        match (self, trigger) {
            (Trigger::None, v) => Ok(v),
            (v, Trigger::None) => Ok(v),
            (Trigger::Name(n), Trigger::Prefix(p)) => Ok(Trigger::NamePrefix(n, p)),
            (Trigger::Prefix(p), Trigger::Name(n)) => Ok(Trigger::NamePrefix(n, p)),
            (Trigger::Name(n), _) => Err(syn::Error::new(span, format!("\"{n}\" already exists"))),
            (Trigger::Prefix(n), _) => Err(syn::Error::new(span, format!("prefix \"{n}\" already exists"))),
            (Trigger::NamePrefix(n, _), _) => Err(syn::Error::new(span, format!("\"{n}\" already exists"))),
        }
    }
}

fn get_variants(input: &syn::DeriveInput) -> syn::Result<(HashMap<Trigger, Type>, Option<Type>)> {
    let map: HashMap<Trigger, Type> = HashMap::new();

    let other: Option<Type> = None;

    let e = match &input.data {
        syn::Data::Enum(v) => v,
        _ => panic!("internal error, this should not happen"),
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
            &[("name", false), ("prefix", false)],
        )?;
        
        let mut trigger = Trigger::None;

        for (path, meta) in meta_map {
            let string = match meta {
                Meta::NameValue(nv) => {
                    let temp: proc_macro::TokenStream = nv.value.to_token_stream().into();
                    let lit: syn::LitStr = syn::parse(temp)?;
                    lit.value()
                },
                _ => return Err(syn::Error::new_spanned(meta, "expected string \"<attr> = <string>\"")),
            };

            match path {
                "name" => trigger = trigger.build(Trigger::Name(string), path.span())?,
                "prefix" => trigger = trigger.build(Trigger::Prefix(string), path.span())?,
                _ => panic!("internal error, this should not happen")
            }
        }
    }

    todo!()
}

fn construct_fields(
    triggers: (HashMap<Trigger, Type>, Option<Type>),
) -> syn::Result<proc_macro2::TokenStream> {
    todo!()
}
