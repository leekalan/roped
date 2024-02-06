use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DataEnum, DeriveInput, Fields, Ident, Lit, Meta};

#[proc_macro_derive(Bundle, attributes(bundle))]
pub fn strand_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let state = extract_attr_ident(&input, "state");

    let mut alternative = None;

    let mut prefixes: Vec<_> = Vec::new();

    let variants: Vec<_> = if let Data::Enum(DataEnum { variants, .. }) = &input.data {
        variants.iter().filter_map(|variant| {
            match &variant.fields {
                Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                    let field_ty = &fields.unnamed[0].ty;

                    if attr_exist(&variant.attrs, "other") {
                        alternative = Some(field_ty);
                        return None
                    }

                    if let Some(prefix) = extract_attr_name(&variant.attrs, "prefix") {
                        prefixes.push((prefix, field_ty));
                    }

                    let scope_name = extract_attr_name(&variant.attrs, "name")?;

                    Some((scope_name, field_ty))
                }
                _ => panic!("Strand derive only supports enums with tuple variants containing a single field"),
            }
        }).collect()
    } else {
        panic!("Strand derive only supports enums");
    };
    
    let prefix_variants = prefixes.iter().map(|(prefix, field_ty)| {
        quote! {
            s if s.starts_with(#prefix) => return #field_ty::run(state, input.strip_prefix(#prefix).unwrap(), ws_chars),
        }
    });

    let parse_variants = variants.iter().map(|(scope_name, field_ty)| {
        quote! {
            #scope_name => return #field_ty::run(state, residue, ws_chars),
        }
    });

    let match_other = if alternative.is_none() {
        quote!(
            _ => return Err(format!("Invalid scope: {}", arg)),
        )
    } else {
        let other = alternative.unwrap();
        quote!(
            _ => return #other::run(state, input, ws_chars),
        )
    };

    let gen = quote! {
        impl Bundle for #name {
            type State = #state;

            fn run(state: &mut Self::State, input: &str, ws_chars: &[char]) -> Result<(), String> {
                fn split_at_char<'a>(input_raw: &'a str, splits: &[char]) -> (&'a str, &'a str) {
                    fn trim_chars<'a>(input: &'a str, splits: &[char]) -> &'a str {
                        let start_trimmed = input.trim_start_matches(|c| splits.contains(&c));
                        let trimmed = start_trimmed.trim_end_matches(|c| splits.contains(&c));
                        trimmed
                    }

                    let input = trim_chars(input_raw, splits);
                
                    let mut out = ("", input);
                
                    for (index, char) in input.char_indices() {
                        if splits.contains(&char) {
                            let (a, b) = input.split_at(index);
                            out = (b, trim_chars(a, splits));
                            break;
                        }
                    }
                
                    out
                }

                let (residue, arg) = split_at_char(input, ws_chars);
                if arg == "" {
                    return Err("Not enough arguments".into())
                }

                match arg {
                    #( #parse_variants )*
                    #( #prefix_variants)*
                    #match_other
                }
            }
        }
    };

    gen.into()
}

fn attr_exist(attrs: &Vec<Attribute>, ident: &str) -> bool {
    for attr in attrs {
        if attr.path.is_ident("bundle") {
            if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                for nested_meta in meta_list.nested {
                    if let syn::NestedMeta::Meta(meta) = nested_meta {
                        if let Meta::Path(path) = meta {
                            if path.is_ident(ident) {
                                return true
                            }
                        }
                    }
                }
            }
        }
    }
    
    false
}

fn extract_attr_name(attrs: &Vec<Attribute>, ident: &str) -> Option<String> {
    let mut action_function_name = attrs.iter().filter_map(|attr| {
        if attr.path.is_ident("bundle") {
            if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                for nested_meta in meta_list.nested {
                    if let syn::NestedMeta::Meta(meta) = nested_meta {
                        if let Meta::NameValue(name_value) = meta {
                            if name_value.path.is_ident(ident) {
                                if let Lit::Str(s) = name_value.lit {
                                    return Some(s.value());
                                }
                            }
                        }
                    }
                }
            }
        }
        None
    });

    action_function_name
        .next()
}

fn extract_attr_ident(input: &DeriveInput, ident: &str) -> Ident {
    let mut action_function_name = input.attrs.iter().filter_map(|attr| {
        if attr.path.is_ident("bundle") {
            if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                for nested_meta in meta_list.nested {
                    if let syn::NestedMeta::Meta(meta) = nested_meta {
                        if let Meta::NameValue(name_value) = meta {
                            if name_value.path.is_ident(ident) {
                                if let Lit::Str(s) = name_value.lit {
                                    return Some(Ident::new(&s.value(), s.span()));
                                }
                            }
                        }
                    }
                }
            }
        }
        None
    });

    action_function_name
        .next()
        .expect(&format!("Couldn't find attribute: {}", ident))
}