use parsr::{
    matcher::{MatchContainer, MatcherStart},
    parse::Parse,
};

pub trait Strand {
    type State;
    type Input: Parse;
    fn run(
        state: &mut Self::State,
        input: Self::Input,
        ws_chars: MatchContainer<Self::Input, <Self::Input as MatcherStart>::Item>,
    ) -> Result<(), String>;
}
