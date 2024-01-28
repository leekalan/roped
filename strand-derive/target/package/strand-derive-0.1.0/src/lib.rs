use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Ident, Lit, Meta};

#[proc_macro_derive(Strand, attributes(strand))]
pub fn strand_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let action = extract_attr_ident(&input, "action");

    let state = extract_attr_ident(&input, "state");

    let fields: Vec<_> = if let Data::Struct(DataStruct { fields, .. }) = &input.data {
        fields.iter().map(|f| (&f.ident, &f.ty)).collect()
    } else {
        panic!("Strand derive only supports structs");
    };

    let parse_args = fields.iter().map(|(field_name, field_type)| {
        quote! {
            let #field_name = args_iter.next().expect("Not enough arguments")
                .parse::<#field_type>().expect("Failed to parse argument");
        }
    });

    let list_fields = fields.iter().map(|(field_name, _)| {
        quote! {
            #field_name
        }
    });

    let gen = quote! {
        impl Strand for #name {
            type State = #state;

            fn run<'a>(state: &mut Self::State, args: impl Iterator<Item = &'a str>) -> Result<(), String> {
                let mut args_iter = args;
                #(#parse_args)*

                let instance = Self { #( #list_fields, )* };

                return instance.#action(state)
            }
        }
    };

    gen.into()
}

fn extract_attr_ident(input: &DeriveInput, ident: &str) -> Ident {
    let mut action_function_name = input.attrs.iter().filter_map(|attr| {
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
        .expect(&format!("Couldn't find attribute: {}", ident))
}