mod search_meta;
mod strand_enum;
mod strand_struct;
mod meta_map;
mod build_error;

use strand_enum::strand_derive_enum;
use strand_struct::strand_derive_struct;

use proc_macro::TokenStream;

#[proc_macro_derive(Strand, attributes(strand))]
pub fn strand_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    match &input.data {
        syn::Data::Struct(_) => strand_derive_struct(input)
            .unwrap_or_else(syn::Error::into_compile_error)
            .into(),
        syn::Data::Enum(_) => strand_derive_enum(input)
            .unwrap_or_else(syn::Error::into_compile_error)
            .into(),
        syn::Data::Union(_) => syn::Error::new_spanned(input, "expected list, \"#[strand(..)]\"")
            .to_compile_error()
            .into(),
    }
}
