pub trait Bundle {
    type State;
    fn run<'a>(state: &mut Self::State, args: impl Iterator<Item = &'a str>) -> Result<(), String>;
}
