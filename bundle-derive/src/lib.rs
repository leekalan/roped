use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DataEnum, DeriveInput, Fields, Ident, Lit, Meta};

#[proc_macro_derive(Bundle, attributes(bundle))]
pub fn strand_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let state = extract_attr_ident(&input, "state");

    let variants: Vec<_> = if let Data::Enum(DataEnum { variants, .. }) = &input.data {
        variants.iter().filter_map(|variant| {
            match &variant.fields {
                Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                    // Tuple variant with a single field
                    let field_ty = &fields.unnamed[0].ty;

                    let scope_name = extract_attr_name(&variant.attrs, "name")?;

                    Some((scope_name, field_ty))
                }
                _ => panic!("Strand derive only supports enums with tuple variants containing a single field"),
            }
        }).collect()
    } else {
        panic!("Strand derive only supports enums");
    };

    let parse_variants = variants.iter().map(|(scope_name, field_ty)| {
        quote! {
            #scope_name => return #field_ty::run(state, args_iter),
        }
    });

    let gen = quote! {
        impl Bundle for #name {
            type State = #state;

            fn run<'a>(state: &mut Self::State, args: impl Iterator<Item = &'a str>) -> Result<(), String> {
                let mut args_iter = args;
                let scope_name = args_iter.next().expect("Not enough arguments");

                match scope_name {
                    #( #parse_variants )*
                    _ => panic!("Invalid scope: {}", scope_name),
                }
            }
        }
    };

    gen.into()
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