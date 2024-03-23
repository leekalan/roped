use std::collections::HashMap;

use syn::Meta;

use crate::build_error::BuildError;

pub fn collect_meta_map<'a, 'b, M>(
    meta_list: M,
    slice: &[(&'b str, bool)],
) -> syn::Result<HashMap<&'b str, Meta>>
where
    M: Clone + quote::ToTokens + IntoIterator<Item = Meta>,
{
    let mut map: HashMap<&str, Meta> = HashMap::new();

    let mut err_builder: Option<syn::Error> = None;

    for meta in meta_list.clone() {
        if let Some((name, _)) = slice.iter().find(|(n, _)| meta.path().is_ident(n)) {
            if map.contains_key(name) {
                err_builder.build_error(syn::Error::new_spanned(meta, "path already exists"));
            } else {
                map.insert(name, meta);
            }
        } else {
            err_builder.build_error(syn::Error::new_spanned(meta, "path does not exist"))
        }
    }

    if let Some(err) = err_builder {
        return Err(err);
    }

    Ok(map)
}
