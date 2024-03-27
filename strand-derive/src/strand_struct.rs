use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Type;

use crate::{meta_map::collect_meta_map, search_meta::search_meta};

pub fn strand_derive_struct(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let name = &input.ident;

    let fields = get_fields(&input)?;

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

enum Extras<'a> {
    None,
    Optional(Vec<DefaultField<'a>>),
    Flags(Vec<Flag<'a>>),
}

#[derive(Clone)]
struct DefaultField<'a> {
    field: Field<'a>,
    default: syn::Expr,
}

#[derive(Clone, Copy)]
struct Flag<'a> {
    ident: &'a syn::Ident,
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
                Extras::None => extras = Extras::Optional(vec![default_object]),
                Extras::Optional(list) => list.push(default_object),
                Extras::Flags(_) => return Err(syn::Error::new_spanned(
                    meta,
                    "both defaults and flags on a strand are not supported",
                )),
            }
        } else if let Some(meta) = meta_map.get("flag") {
            let flag_type: Option<Type> = match meta {
                syn::Meta::NameValue(n) => Some(syn::parse(n.value.to_token_stream().into())?),
                syn::Meta::Path(_) => None,
                _ => {
                    return Err(syn::Error::new_spanned(
                        meta,
                        "expected type, \"state = <type>\"",
                    ))
                }
            };

            todo!();
        } else {
            return Err(syn::Error::new_spanned(
                strand_meta,
                "expected attribute, \"#[strand(optional / flag / flag = <type>)]\"",
            ))
        }
    }

    todo!()
}
