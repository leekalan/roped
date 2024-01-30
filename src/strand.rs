pub trait Strand {
    type State;
    fn run(state: &mut Self::State, input: &str, ws_chars: &[char]) -> Result<(), String>;
}
