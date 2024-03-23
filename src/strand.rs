use parsr::{
    parser::{search::Search, Parser},
    parser_matcher::Matcher,
};

use crate::error::Error;

pub struct RopedInfo {}

pub trait Strand {
    type State;
    type Input: ?Sized + Parser;
    type Err;

    fn run<'a>(
        state: &mut Self::State,
        input: &Self::Input,
        ws: &Matcher<Self::Input, <Self::Input as Search>::Item>,
        index: usize,
    ) -> Result<(), Error<&'a Self::Input, Self::Err>>;
}
