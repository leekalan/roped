use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Type;

use crate::{meta_map::collect_meta_map, search_meta::search_meta};

pub fn strand_derive_struct(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let name = &input.ident;

    let (fields, extras) = get_fields(&input)?;

    let internal = construct_internal(fields, extras);

    let gen = quote::quote! {
        impl ::roped::strand::Strand for #name {
            type State = <Self as ::roped::command::Command>::State;
            type Err = <Self as ::roped::command::Command>::Err;

            fn run(
                state: &mut Self::State,
                raw_input: Option<::roped::parsr::parser::trimmed::Trimmed<str>>,
                index: usize,
            ) -> Result<(), ::roped::error::Error<Self::Err>> {
                let mut input = raw_input;
                let mut index = index;

                #internal

                this.action(state).map_err(|err| ::roped::error::Error::Err(err))
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
    Trail(Field<'a>),
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

        if let Some(strand_meta) = search_meta(field.attrs.iter().map(|s| &s.meta), "strand") {
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

            let meta_map = collect_meta_map(meta_list, &["default", "flag", "trail"])?;

            if field_state {
                if meta_map.is_empty() {
                    fields.push(Field { ident, ty });
                    continue;
                } else {
                    field_state = false
                }
            }

            if let Some(meta) = meta_map.get("default") {
                let default: syn::Expr = match meta {
                    syn::Meta::NameValue(nv) => nv.value.clone(),
                    _ => {
                        return Err(syn::Error::new_spanned(
                            meta,
                            "expected default value \"default = <expr>\"",
                        ))
                    }
                };

                let default_object = DefaultField {
                    field: Field { ident, ty },
                    default,
                };

                match &mut extras {
                    Extras::None => extras = Extras::Default(vec![default_object]),
                    Extras::Default(list) => list.push(default_object),
                    Extras::Flags(_) => {
                        return Err(syn::Error::new_spanned(
                            meta,
                            "both defaults and flags on a strand are not supported",
                        ))
                    }
                    Extras::Trail(_) => {
                        panic!("both defaults and trails on a strand are not supported")
                    }
                }
            } else if let Some(meta) = meta_map.get("flag") {
                let flag_name: String = match meta {
                    syn::Meta::NameValue(n) => {
                        let lit: syn::LitStr = syn::parse(n.value.to_token_stream().into())?;
                        lit.value()
                    }
                    _ => {
                        return Err(syn::Error::new_spanned(
                            meta,
                            "expected name, \"flag = <name>\"",
                        ))
                    }
                };

                let flag_type = match ty {
                    syn::Type::Path(syn::TypePath { qself: None, path }) => {
                        let seg = path.segments.first().unwrap();
                        if seg.ident == "Option" {
                            match &seg.arguments {
                                syn::PathArguments::AngleBracketed(
                                    syn::AngleBracketedGenericArguments { args, .. },
                                ) => {
                                    if args.len() != 1 {
                                        return Err(syn::Error::new_spanned(
                                            ty,
                                            "expected single type argument on Option, \"Option<T>\"",
                                        ));
                                    }
                                    match args.first().unwrap() {
                                        syn::GenericArgument::Type(ty) => ty,
                                        _ => return Err(syn::Error::new_spanned(
                                            ty,
                                            "expected single type argument on Option, \"Option<T>\"",
                                        )),
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
                            ));
                        }
                    }
                    _ => {
                        return Err(syn::Error::new_spanned(
                            ty,
                            "expected single type argument on Option, \"Option<T>\"",
                        ))
                    }
                };

                let is_unit = match flag_type {
                    Type::Path(syn::TypePath { qself: None, path }) => {
                        path.segments.last().map_or(false, |path_segment| {
                            path_segment.ident == "Trigger" && path_segment.arguments.is_empty()
                        })
                    }
                    _ => false,
                };

                let flag_type = if !is_unit {
                    FlagType::Value(flag_type)
                } else {
                    FlagType::Trigger
                };

                let flag_object = Flag {
                    ident,
                    name: flag_name,
                    flag_type,
                };

                match &mut extras {
                    Extras::None => extras = Extras::Flags(vec![flag_object]),
                    Extras::Flags(list) => list.push(flag_object),
                    Extras::Default(_) => {
                        return Err(syn::Error::new_spanned(
                            meta,
                            "both defaults and flags on a strand are not supported",
                        ))
                    }
                    Extras::Trail(_) => {
                        panic!("both flags and trails on a strand are not supported")
                    }
                }
            } else if let Some(meta) = meta_map.get("trail") {
                match meta {
                    syn::Meta::Path(_) => (),
                    _ => return Err(syn::Error::new_spanned(meta, "expected, \"trail\"")),
                };

                match &mut extras {
                    Extras::None => extras = Extras::Trail(Field { ident, ty }),
                    Extras::Flags(_) => {
                        return Err(syn::Error::new_spanned(
                            meta,
                            "both flags and trails on a strand are not supported",
                        ))
                    }
                    Extras::Default(_) => {
                        return Err(syn::Error::new_spanned(
                            meta,
                            "both defaults and trails on a strand are not supported",
                        ))
                    }
                    Extras::Trail(_) => panic!("a trail can only appear once on a strand"),
                }
            } else {
                return Err(syn::Error::new_spanned(
                    strand_meta,
                    "expected attribute, \"#[strand(optional / flag / flag = <type>)]\"",
                ));
            }
        } else {
            fields.push(Field { ident, ty });
        }
    }

    Ok((fields, extras))
}

fn construct_internal(fields: Vec<Field>, extras: Extras) -> TokenStream {
    let field_constructors = construct_fields(&fields);
    let other = match &extras {
        Extras::None => quote::quote!(),
        Extras::Default(t0) => construct_defaults(t0),
        Extras::Flags(t0) => construct_flags(t0),
        Extras::Trail(t0) => construct_trail(t0),
    };
    let constructor = construct_constructor(&fields, extras);

    quote::quote! {
        #field_constructors
        #other
        let this = Self {
            #constructor
        };

        if let Some(overflow) = input {
            return Err(::roped::error::Error::Internal(::roped::error::InternalError {
                index,
                variant: ::roped::error::ErrorType::Unexpected(overflow.parse_once().arg.get_internal().to_string()),
            }))
        }
    }
}

fn construct_fields(fields: &[Field]) -> TokenStream {
    let mut field_constructors: Vec<TokenStream> = Vec::with_capacity(fields.len());

    for field in fields {
        let ident = field.ident;
        let ty = field.ty;

        let quote = quote::quote! {
            let s = match input {
                Some(v) => v,
                None => return Err(::roped::error::Error::Internal(::roped::error::InternalError {
                    index,
                    variant: ::roped::error::ErrorType::Expected(::roped::error::ArgType::Arg)
                }))
            };

            let pair = s.parse_once();

            let #ident: #ty = match std::str::FromStr::from_str(pair.arg.get_internal()) {
                Ok(v) => v,
                Err(_) => return Err(::roped::error::Error::Internal(::roped::error::InternalError {
                    index,
                    variant: ::roped::error::ErrorType::Parse(::roped::error::ParseErr {
                        arg: pair.arg.get_internal().to_string(),
                        parse_type: ::roped::error::ArgType::Arg,
                    })
                })),
            };

            index += 1;

            input = pair.trail;
        };

        field_constructors.push(quote);
    }

    quote!(#(#field_constructors)*)
}

fn construct_defaults(defaults: &[DefaultField]) -> TokenStream {
    let mut field_constructors: Vec<TokenStream> = Vec::with_capacity(defaults.len());

    for default_field in defaults {
        let ident = default_field.field.ident;
        let ty = default_field.field.ty;

        let expr = &default_field.default;

        let quote = quote::quote! {
            let #ident: #ty = match input {
                Some(v) => {
                    let pair = v.parse_once();

                    let out: #ty = match std::str::FromStr::from_str(pair.arg.get_internal()) {
                        Ok(v) => v,
                        Err(_) => return Err(::roped::error::Error::Internal(::roped::error::InternalError {
                            index,
                            variant: ::roped::error::ErrorType::Parse(::roped::error::ParseErr {
                                arg: pair.arg.get_internal().to_string(),
                                parse_type: ::roped::error::ArgType::Arg,
                            })
                        })),
                    };

                    input = pair.trail;

                    out
                },
                None => #expr,
            };

            index += 1;
        };

        field_constructors.push(quote);
    }

    quote!(#(#field_constructors)*)
}

fn construct_flags(flags: &[Flag]) -> TokenStream {
    let mut flag_setters: Vec<TokenStream> = Vec::with_capacity(flags.len());
    let mut flag_matchers: Vec<TokenStream> = Vec::with_capacity(flags.len());

    for flag in flags {
        let ident = flag.ident;
        let name = &flag.name;

        let (set_quote, match_quote) = match flag.flag_type {
            FlagType::Trigger => (
                quote!(let mut #ident: Option<Trigger> = None;),
                quote! {
                    #name => #ident = Some(Trigger),
                },
            ),
            FlagType::Value(ty) => (
                quote!(let mut #ident: Option<#ty> = None;),
                quote! {
                    #name => {
                        if let Some(s) = input {
                            let pair = s.parse_once();

                            let out: #ty = match std::str::FromStr::from_str(pair.arg.get_internal()) {
                                Ok(v) => v,
                                Err(_) => return Err(::roped::error::Error::Internal(::roped::error::InternalError {
                                    index,
                                    variant: ::roped::error::ErrorType::Parse(::roped::error::ParseErr {
                                        arg: pair.arg.get_internal().to_string(),
                                        parse_type: ::roped::error::ArgType::Arg,
                                    })
                                })),
                            };

                            input = pair.trail;

                            #ident = Some(out);
                        } else {
                            return Err(::roped::error::Error::Internal(::roped::error::InternalError {
                                index,
                                variant: ::roped::error::ErrorType::Expected(::roped::error::ArgType::Flag),
                            }))
                        }
                    },
                },
            ),
        };

        flag_setters.push(set_quote);
        flag_matchers.push(match_quote);
    }

    quote! {
        #(#flag_setters)*

        while let Some(s) = input {
            let pair = s.parse_once();

            input = pair.trail;

            if let Some(identifier) = ::roped::parsr::parser::trim::Trim::trim_once(pair.arg.get_internal(), ::roped::Matcher::Single(MatcherSingle::Ident("--"))) {
                match identifier {
                    #(#flag_matchers)*
                    _ => return Err(::roped::error::Error::Internal(::roped::error::InternalError {
                        index,
                        variant: ::roped::error::ErrorType::InvalidFlag(pair.arg.get_internal().to_string()),
                    })),
                }
            } else {
                return Err(::roped::error::Error::Internal(::roped::error::InternalError {
                    index,
                    variant: ::roped::error::ErrorType::Unexpected(pair.arg.get_internal().to_string()),
                }))
            }

            index += 1;
        }
    }
}

fn construct_trail(field: &Field) -> TokenStream {
    let ident = field.ident;
    let ty = field.ty;

    quote::quote! {
        let s = match input.take() {
            Some(v) => v.get_internal().to_string(),
            None => "".to_string(),
        };

        let #ident: #ty = match std::str::FromStr::from_str(&s) {
            Ok(v) => v,
            Err(_) => return Err(::roped::error::Error::Internal(::roped::error::InternalError {
                index,
                variant: ::roped::error::ErrorType::Parse(::roped::error::ParseErr {
                    arg: s,
                    parse_type: ::roped::error::ArgType::Arg,
                })
            })),
        };
    }
}

fn construct_constructor(fields: &[Field], extras: Extras) -> TokenStream {
    let mut field_constructors: Vec<TokenStream> = Vec::with_capacity(fields.len());

    for field in fields {
        let ident = field.ident;

        let quote = quote::quote! {
            #ident,
        };

        field_constructors.push(quote);
    }

    match extras {
        Extras::None => (),
        Extras::Default(t0) => {
            for field in t0 {
                let ident = field.field.ident;

                field_constructors.push(quote! {
                    #ident,
                })
            }
        }
        Extras::Flags(t0) => {
            for flag in t0 {
                let ident = flag.ident;

                field_constructors.push(quote! {
                    #ident,
                })
            }
        }
        Extras::Trail(t0) => field_constructors.push({
            let ident = t0.ident;

            quote! {
                #ident,
            }
        }),
    }

    quote!(#(#field_constructors)*)
}
