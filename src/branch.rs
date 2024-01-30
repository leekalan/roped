pub trait Branch {
    type State;
    fn run<'a>(state: &mut Self::State, arg: &'a str, args: impl Iterator<Item = &'a str>) -> Result<(), String>;
}
