pub trait Command {
    type State: ?Sized;
    type Err;
    fn action(self, state: &mut Self::State) -> Result<(), Self::Err>;
}
