use proc_macro2::TokenStream;
use syn::Type;

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
    lit: &'a syn::Lit,
    ty: &'a Type,
}

enum Extras<'a> {
    None,
    Optional(Vec<OptionalField<'a>>),
    Flags(Vec<OptionalField<'a>>),
}

#[derive(Clone, Copy)]
struct OptionalField<'a> {
    field: Field<'a>,
    default: &'a syn::Expr,
}

#[derive(Clone, Copy)]
struct Flag<'a> {
    lit: &'a syn::Lit,
    flag_type: FlagType<'a>,
}

#[derive(Clone, Copy)]
enum FlagType<'a> {
    Trigger,
    Value(&'a Type),
}

fn get_fields(input: &syn::DeriveInput) -> syn::Result<(Vec<Field>, Extras)> {
    todo!()
}