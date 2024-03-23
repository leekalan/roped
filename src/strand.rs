use std::borrow::Borrow;

use parsr::{parser::{search::Search, Parser}, parser_matcher::Matcher};

use crate::error::Error;

pub struct RopedInfo {}

pub trait Strand<'a> {
    type State;
    type Input: ?Sized + Parser;
    type Err;

    fn run(
        state: &mut Self::State,
        input: &Self::Input,
        ws: &'a impl Borrow<Matcher<'a, Self::Input, <Self::Input as Search>::Item>>,
        index: usize,
    ) -> Result<(), Error<&'a Self::Input, Self::Err>>;
}
