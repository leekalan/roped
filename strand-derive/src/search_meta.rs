use syn::Meta;

pub fn search_meta<'a, M>(mut meta_list: M, ident: &str) -> Option<&'a Meta>
where
    M: Iterator<Item = &'a Meta>,
{
    meta_list.find(|meta| meta.path().is_ident(ident))
}
