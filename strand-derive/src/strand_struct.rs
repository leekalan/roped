use std::any::Any;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Type;

use crate::{meta_map::collect_meta_map, search_meta::search_meta};

pub fn strand_derive_struct(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let name = &input.ident;

    let (fields, extras) = get_fields(&input)?;

    // todo!("construct arguments");

    let gen = quote::quote! {
        impl ::roped::strand::Strand for #name {
            type State = <Self as ::roped::command::Command>::State;
            type Err = <Self as ::roped::command::Command>::Err;

            fn run(
                state: &mut Self::State,
                raw_input: Option<::roped::parsr::parser::safe_str::SafeStr>,
                index: usize,
            ) -> Result<(), ::roped::error::Error<Self::Err>> {
                let mut input = raw_input;

                // let this = #this_contructor;

                todo!() // this.run(state, input)
            }
        }
    };

    Ok(gen)
}

#[derive(Clone, Copy)]
struct Field<'a> {
    ident: &'a syn::Ident,
    ty: &'a Type,
}

#[derive(Clone)]
enum Extras<'a> {
    None,
    Default(Vec<DefaultField<'a>>),
    Flags(Vec<Flag<'a>>),
}

#[derive(Clone)]
struct DefaultField<'a> {
    field: Field<'a>,
    default: syn::Expr,
}

#[derive(Clone)]
struct Flag<'a> {
    ident: &'a syn::Ident,
    name: String,
    flag_type: FlagType<'a>,
}

#[derive(Clone, Copy)]
enum FlagType<'a> {
    Trigger,
    Value(&'a Type),
}

fn get_fields(input: &syn::DeriveInput) -> syn::Result<(Vec<Field>, Extras)> {
    let mut field_state = true;

    let mut fields: Vec<Field> = Vec::new();
    let mut extras: Extras = Extras::None;

    let data = match &input.data {
        syn::Data::Struct(v) => v,
        _ => panic!("internal error, this should not happen"),
    };

    for field in &data.fields {
        let ident: &syn::Ident = match &field.ident {
            Some(v) => v,
            None => panic!("If this happens you must be doing something wrong"),
        };

        let ty = &field.ty;
        
        let strand_meta = search_meta(field.attrs.iter().map(|s| &s.meta), "strand").ok_or(
            syn::Error::new_spanned(field, "expected attribute, \"#[strand(..)]\""),
        )?;

        let meta_list = match strand_meta {
            syn::Meta::List(v) => v.parse_args_with(
                syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
            )?,
            _ => {
                return Err(syn::Error::new_spanned(
                    strand_meta,
                    "expected list, \"#[strand(..)]\"",
                ))
            }
        };

        let meta_map = collect_meta_map(meta_list, &["default", "flag"])?;

        if field_state {
            if meta_map.is_empty() {
                fields.push(Field { ident, ty });
                continue
            } else {
                field_state = false
            }
        }

        if let Some(meta) = meta_map.get("default") {
            let default: syn::Expr = match meta {
                syn::Meta::NameValue(nv) => {
                    nv.value.clone()
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        meta,
                        "expected default value \"default = <expr>\"",
                    ))
                }
            };

            let default_object = DefaultField { field: Field { ident, ty }, default };

            match &mut extras {
                Extras::None => extras = Extras::Default(vec![default_object]),
                Extras::Default(list) => list.push(default_object),
                Extras::Flags(_) => return Err(syn::Error::new_spanned(
                    meta,
                    "both defaults and flags on a strand are not supported",
                )),
            }
        } else if let Some(meta) = meta_map.get("flag") {
            let flag_name: String = match meta {
                syn::Meta::NameValue(n) => {
                    let lit: syn::LitStr = syn::parse(n.value.to_token_stream().into())?;
                    lit.value()
                },
                _ => {
                    return Err(syn::Error::new_spanned(
                        meta,
                        "expected type, \"flag = <name>\"",
                    ))
                }
            };

            let flag_type = match ty {
                syn::Type::Path(syn::TypePath { qself: None, path }) => {
                    let seg = path.segments.first().unwrap();
                    if seg.ident == "Option" {
                        match &seg.arguments {
                            syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments { args, .. }) => {
                                if args.len() != 1 {
                                    return Err(syn::Error::new_spanned(
                                        ty,
                                        "expected single type argument on Option, \"Option<T>\"",
                                    ))
                                }
                                match args.first().unwrap() {
                                    syn::GenericArgument::Type(ty) => ty,
                                    _ => {
                                        return Err(syn::Error::new_spanned(
                                            ty,
                                            "expected single type argument on Option, \"Option<T>\"",
                                        ))
                                    }
                                }
                            }
                            _ => {
                                return Err(syn::Error::new_spanned(
                                    ty,
                                    "expected single type argument on Option, \"Option<T>\"",
                                ))
                            }
                        }
                    } else {
                        return Err(syn::Error::new_spanned(
                            ty,
                            "expected single type argument on Option, \"Option<T>\"",
                        ))
                    }
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        ty,
                        "expected single type argument on Option, \"Option<T>\"",
                    ))
                }
            };

            let empty: Type = syn::parse(quote::quote! { () }.into_token_stream().into())?;

            let is_unit = match flag_type {
                Type::Path(syn::TypePath { qself: None, path }) => {
                    path.segments.last().map_or(false, |path_segment| {
                        path_segment.ident == "Trigger" && path_segment.arguments.is_empty()
                    })
                },
                _ => false,
            };

            let flag_type = if !is_unit {
                FlagType::Value(flag_type)
            } else {
                FlagType::Trigger
            };

            let flag_object = Flag { ident, name: flag_name, flag_type };

            match &mut extras {
                Extras::None => extras = Extras::Flags(vec![flag_object]),
                Extras::Flags(list) => list.push(flag_object),
                Extras::Default(_) => return Err(syn::Error::new_spanned(
                    meta,
                    "both defaults and flags on a strand are not supported",
                )),
            }
        } else {
            return Err(syn::Error::new_spanned(
                strand_meta,
                "expected attribute, \"#[strand(optional / flag / flag = <type>)]\"",
            ))
        }
    }
    
    Ok((fields, extras))
}
