use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DataStruct, DeriveInput, Ident, Lit, Meta, Type};

enum FieldInfo<'a> {
    Field(Ident, &'a Type),
    Flag(Ident, String),
}

#[proc_macro_derive(Strand, attributes(strand))]
pub fn strand_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let action = extract_attr_ident(&input.attrs, "action");

    let state = extract_attr_ident(&input.attrs, "state");

    let fields: Vec<_> = if let Data::Struct(DataStruct { fields, .. }) = &input.data {
        fields.iter().map(|f| {
            if let Some(flag) = extract_attr_name(&f.attrs, "flag") {
                return FieldInfo::Flag(f.ident.clone().expect("Invalid Identifier"), flag)
            }
            
            FieldInfo::Field(f.ident.clone().expect("Invalid Identifier"), &f.ty)
        }).collect()
    } else {
        panic!("Strand derive only supports structs");
    };

    let parse_args = fields.iter().map(|field_info| {
        match field_info {
            FieldInfo::Field(field_name, field_type) => {
                quote! {
                    let (residue, arg) = split_at_char(residue, ws_chars);
                    if arg == "" {
                        return Err("Not enough arguments".into())
                    }
        
                    let parsed = match arg.parse::<#field_type>() {
                        Ok(val) => val,
                        Err(_) => return Err(format!("Invalid argument")),
                    };
        
                    let #field_name = parsed;
                }
            },
            FieldInfo::Flag(field_name, flag_name) => {
                let flag_match = format!("--{}", flag_name);

                let flag_char = match flag_name.chars().next() {
                    Some(val) => val.to_string(),
                    None => "".into(),
                };
                let flag_small = format!("-{}", flag_char);

                quote! {
                    let (temp, arg) = split_at_char(residue, ws_chars);

                    let flagged = if arg != "" {
                        arg == #flag_match || arg == #flag_small
                    } else {
                        false
                    };

                    let #field_name = flagged;
        
                    if flagged {
                        let residue = temp;
                    }
                }
            },
        }
    });

    let list_fields = fields.iter().map(|field_info| {
        let field_name = match field_info {
            FieldInfo::Field(n, _) => n,
            FieldInfo::Flag(n, _) => n,
        };
        quote! {
            #field_name,
        }
    });

    let gen = quote! {
        impl Strand for #name {
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

                let mut residue = input;
                let mut arg = String::new();

                #(#parse_args)*

                let instance = Self { #( #list_fields )* };

                return instance.#action(state)
            }
        }
    };

    gen.into()
}

fn extract_attr_name(attrs: &Vec<Attribute>, ident: &str) -> Option<String> {
    let mut action_function_name = attrs.iter().filter_map(|attr| {
        if attr.path.is_ident("strand") {
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

fn extract_attr_ident(attrs: &Vec<Attribute>, ident: &str) -> Option<Ident> {
    let mut action_function_name = attrs.iter().filter_map(|attr| {
        if attr.path.is_ident("strand") {
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
}