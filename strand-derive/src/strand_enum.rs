use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Meta, Type};

use crate::{meta_map::collect_meta_map, search_meta::search_meta};

pub fn strand_derive_enum(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let name = &input.ident;

    let config = get_config(&input)?;

    let (prefixes, names, other) = get_variants(&input)?;

    let captures = construct_fields(prefixes, names, other);

    let Config { state, error } = config;

    let gen = quote::quote! {
        impl ::roped::strand::Strand for #name {
            type State = #state;
            type Err = #error;

            fn run(
                state: &mut Self::State,
                raw_input: Option<::roped::parsr::parser::safe_str::SafeStr>,
                index: usize,
            ) -> Result<(), ::roped::error::Error<Self::Err>> {
                #captures
            }
        }
    };

    Ok(gen)
}

#[derive(Clone)]
struct Config {
    state: Type,
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
        None => syn::parse_quote! { () },
    };

    Ok(Config { state, error })
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

    let e = match &input.data {
        syn::Data::Enum(v) => v,
        _ => panic!("internal error, this should not happen"),
    };

    for variant in &e.variants {
        let variant_type = match &variant.fields {
            syn::Fields::Unnamed(v) if v.unnamed.len() == 1 => &v.unnamed[0].ty,
            _ => {
                return Err(syn::Error::new_spanned(
                    &variant,
                    "expected tuple, \"<name>(<type>)\"",
                ))
            }
        };

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

        let meta_map = collect_meta_map(meta_list, &["name", "prefix", "other"])?;

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

            names.push(Name(string, variant_type));
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

            prefixes.push(Prefix(string, variant_type));
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
        }
    }

    Ok((prefixes, names, other))
}

fn construct_fields(
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
                let parse_pair = input.safe_parse_once();

                match parse_pair.arg.as_str() {
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
                    input.as_str(), ::roped::parsr::parser_matcher::Matcher::ident(&#s)
                ) {
                    #t::run(state, ::roped::parsr::parser::safe_str::SafeStr::new(trail, input.get_matcher()), index)
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
                #s => #t::run(state, parse_pair.trail, index),
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
            panic!("Invalid scope")
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
            panic!("Missing scope")
        }
    }
}
