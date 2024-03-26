pub trait Scope {
    type State: ?Sized;
    type Err;
}