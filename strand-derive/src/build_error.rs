pub trait BuildError {
    fn build_error(&mut self, err: syn::Error);
}

impl BuildError for Option<syn::Error> {
    fn build_error(&mut self, err: syn::Error) {
        match self {
            Some(s) => s.combine(err),
            None => *self = Some(err),
        }
    }
}