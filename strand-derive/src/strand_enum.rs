use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Meta, Type};

use crate::{meta_map::collect_meta_map, search_meta::search_meta};

pub fn strand_derive_enum(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let name = &input.ident;

    let config = get_config(&input)?;

    let (prefixes, names, other) = get_variants(&input)?;

    let captures = construct_internal(prefixes, names, other);

    let Config { state, error } = config;

    let gen = quote::quote! {
        impl ::roped::strand::Strand for #name {
            type State = #state;
            type Err = #error;

            fn run(
                state: &mut Self::State,
                raw_input: Option<::roped::parsr::parser::trimmed::Trimmed<str>>,
                index: usize,
            ) -> Result<(), ::roped::error::Error<Self::Err>> {
                #captures
            }
        }
    };

    Ok(gen)
}

#[derive(Clone)]
pub struct Config {
    pub state: Type,
    pub error: Type,
}

pub fn get_config(input: &syn::DeriveInput) -> syn::Result<Config> {
    if let Some(strand_meta) = search_meta(input.attrs.iter().map(|s| &s.meta), "strand") {
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

        let meta_map = collect_meta_map(meta_list, &["state", "input", "error"])?;

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
            None => syn::parse_quote! { String },
        };

        Ok(Config { state, error })
    } else {
        Ok(Config {
            state: syn::parse_quote! { roped::base_types::EmptyState },
            error: syn::parse_quote! { String },
        })
    }
}

#[derive(Clone)]
struct Prefix<'a>(String, &'a Type);
#[derive(Clone)]
struct Name<'a>(String, &'a Type);
#[derive(Clone, Copy)]
struct Other<'a>(&'a Type);

fn get_variants(input: &syn::DeriveInput) -> syn::Result<(Vec<Prefix>, Vec<Name>, Option<Other>)> {
    let mut prefixes: Vec<Prefix> = Vec::new();
    let mut names: Vec<Name> = Vec::new();

    let mut other: Option<Other> = None;

    let data = match &input.data {
        syn::Data::Enum(v) => v,
        _ => panic!("internal error, this should not happen"),
    };

    for variant in &data.variants {
        let variant_type = match &variant.fields {
            syn::Fields::Unnamed(v) if v.unnamed.len() == 1 => &v.unnamed[0].ty,
            _ => {
                return Err(syn::Error::new_spanned(
                    variant,
                    "expected tuple, \"<name>(<type>)\"",
                ))
            }
        };

        let strand_meta = search_meta(variant.attrs.iter().map(|s| &s.meta), "strand").ok_or(
            syn::Error::new_spanned(variant, "expected attribute, \"#[strand(..)]\""),
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

        let meta_map = collect_meta_map(meta_list, &["name", "prefix", "other"])?;

        let mut no_reference = false;

        if let Some(meta) = meta_map.get("name") {
            let string: String = match meta {
                Meta::NameValue(nv) => {
                    let temp: proc_macro::TokenStream = nv.value.to_token_stream().into();
                    let lit: syn::LitStr = syn::parse(temp)?;
                    lit.value()
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        meta,
                        "expected string \"<attr> = <string>\"",
                    ))
                }
            };

            if prefixes.iter().any(|Prefix(s, _)| s == &string) {
                return Err(syn::Error::new_spanned(meta, "prefix/name already exists"));
            }

            if names.iter().any(|Name(s, _)| s == &string) {
                return Err(syn::Error::new_spanned(meta, "prefix/name already exists"));
            }

            if string.is_empty() {
                return Err(syn::Error::new_spanned(meta, "expected non-empty string"));
            }

            names.push(Name(string, variant_type));

            no_reference = true;
        }

        if let Some(meta) = meta_map.get("prefix") {
            let string: String = match meta {
                Meta::NameValue(nv) => {
                    let temp: proc_macro::TokenStream = nv.value.to_token_stream().into();
                    let lit: syn::LitStr = syn::parse(temp)?;
                    lit.value()
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        meta,
                        "expected string \"<attr> = <string>\"",
                    ))
                }
            };

            if prefixes.iter().any(|Prefix(s, _)| s == &string) {
                return Err(syn::Error::new_spanned(meta, "prefix/name already exists"));
            }

            if names.iter().any(|Name(s, _)| s == &string) {
                return Err(syn::Error::new_spanned(meta, "prefix/name already exists"));
            }

            if string.is_empty() {
                return Err(syn::Error::new_spanned(meta, "expected non-empty string"));
            }

            prefixes.push(Prefix(string, variant_type));

            no_reference = true;
        }

        if let Some(meta) = meta_map.get("other") {
            match meta {
                Meta::Path(_) => {
                    if other.is_none() {
                        other = Some(Other(variant_type))
                    } else {
                        return Err(syn::Error::new_spanned(meta, "\"other\" already exists"));
                    }
                }
                _ => return Err(syn::Error::new_spanned(meta, "expected string \"<attr>\"")),
            };

            no_reference = true;
        }

        if !no_reference {
            return Err(syn::Error::new_spanned(
                strand_meta,
                "expected a trigger to run the Strand, example: \"#[strand(name = \"command\")\"]",
            ));
        }
    }

    Ok((prefixes, names, other))
}

fn construct_internal(
    prefixes: Vec<Prefix>,
    names: Vec<Name>,
    other: Option<Other>,
) -> proc_macro2::TokenStream {
    let prefix_quote = prefix_matchers(prefixes);
    let name_quote = name_matchers(names);
    let other_quote = other_matcher(other);
    let no_input = no_input(other);

    quote::quote!(
        if let Some(input) = raw_input {
            if false { unsafe { std::hint::unreachable_unchecked() } }
            #prefix_quote
            else {
                let parse_pair = input.parse_once();

                match parse_pair.arg.get_internal() {
                    #name_quote
                    #other_quote
                }
            }
        } else {
            #no_input
        }
    )
}

fn prefix_matchers(prefixes: Vec<Prefix>) -> proc_macro2::TokenStream {
    let matchers: Vec<_> = prefixes
        .into_iter()
        .map(|Prefix(s, t)| {
            quote::quote! {
                else if let Some(trail) = ::roped::parsr::parser::trim::Trim::trim_once(
                    input.get_internal(), ::roped::parsr::parser_matcher::Matcher::ident(&#s)
                ) {
                    #t::run(state, ::roped::parsr::parser::trimmed::Trimmed::<str>::new(trail, input.get_matcher()), index)
                }
            }
        })
        .collect();

    quote::quote! {
        #(#matchers)*
    }
}

fn name_matchers(names: Vec<Name>) -> proc_macro2::TokenStream {
    let matchers: Vec<_> = names
        .into_iter()
        .map(|Name(s, t)| {
            quote::quote! {
                #s => #t::run(state, parse_pair.trail, index + 1),
            }
        })
        .collect();

    quote::quote!(
        #(#matchers)*
    )
}

fn other_matcher(other: Option<Other>) -> proc_macro2::TokenStream {
    match other {
        Some(Other(t)) => {
            quote::quote! {
                _ => #t::run(state, raw_input, index),
            }
        }
        None => {
            quote::quote! {
                "" => Err(::roped::error::Error::Internal(::roped::error::InternalError{
                    index,
                    variant: ::roped::error::ErrorType::Expected(::roped::error::ArgType::Scope)
                })),
                f => Err(::roped::error::Error::Internal(::roped::error::InternalError {
                    index,
                    variant: ::roped::error::ErrorType::Parse(::roped::error::ParseErr {
                        arg: f.to_string(),
                        parse_type: ::roped::error::ArgType::Scope,
                    })
                })),
            }
        }
    }
}

fn no_input(other: Option<Other>) -> proc_macro2::TokenStream {
    match other {
        Some(Other(t)) => {
            quote::quote! {
                #t::run(state, None, index)
            }
        }
        None => {
            quote::quote! {
                Err(::roped::Error::Internal(::roped::error::InternalError{
                    index,
                    variant: ::roped::error::ErrorType::Expected(::roped::error::ArgType::Scope)
                }))
            }
        }
    }
}
