use parsr::{
    matcher::{MatchContainer, MatcherStart},
    parse::Parse,
};

use crate::error::Error;

pub struct RopedInfo {}

pub trait Strand<'a> {
    type State;
    type Input: Parse;
    type Err;

    fn run(
        state: &mut Self::State,
        input: Self::Input,
        ws_chars: MatchContainer<Self::Input, <Self::Input as MatcherStart>::Item>,
        index: usize,
    ) -> Result<(), Error<Self::Input, Self::Err>>;
}
