#![allow(unused)]

pub mod base_types;
pub mod console;
pub mod error;
pub mod strand;
pub mod command;

use base_types::EmptyState;
use parsr::parser_matcher::Matcher;
use strand::Strand;
use strand_derive::Strand;

use error::Error;

#[allow(clippy::single_component_path_imports)]
use parsr;

extern crate self as roped;

#[cfg(test)]
mod tests {
    use std::{borrow::Borrow, process::CommandArgs};

    use self::command::Command;

    use super::*;

    use base_types::EmptyState;
    use console::run_console;
    use parsr::{
        parser::{safe_str::SafeStr, search::Search, ParsePair, Parser},
        parser_matcher::{Matcher, MatcherSingle},
    };
    use strand::Strand;

    use strand_derive::Strand;

    use crate as roped;

    struct ManualImplStrand;
    impl Strand for ManualImplStrand {
        type State = EmptyState;
        type Err = String;

        fn run(
            _state: &mut Self::State,
            input: Option<SafeStr>,
            _index: usize,
        ) -> Result<(), error::Error<Self::Err>> {
            let input = match input {
                Some(v) => v,
                None => return Err(error::Error::Err("Recieved no input".to_string())),
            };

            let pair = input.safe_parse_once();

            match pair.trail {
                Some(v) => println!("{} + {}", pair.arg.as_str(), v.as_str()),
                None => println!("{}", pair.arg.as_str()),
            }

            Ok(())
        }
    }

    #[test]
    fn manual_strand_instance() {
        run_console::<ManualImplStrand>(
            &mut EmptyState,
            "> ".into(),
            ". ".into(),
            "!".into(),
            Matcher::Single(MatcherSingle::Item(' ')),
            Matcher::List(&[MatcherSingle::Item('\n'), MatcherSingle::Item(';')]),
        );
    }

    // #[derive(Strand)]
    // struct CommandStrand {
    //     num: usize,
    //     string: String,
    // }

    // impl Command for CommandStrand {
    //     type State = EmptyState;
    
    //     type Err = String;
    
    //     fn action(self, state: &mut Self::State) -> Result<(), Self::Err> {
    //         todo!()
    //     }
    // }

    #[derive(Strand)]
    #[strand(state = EmptyState, error = String)]
    enum ScopeStrand {
        #[strand(prefix = "$")]
        A(ManualImplStrand),
        #[strand(name = "command")]
        B(ManualImplStrand),
        #[strand(other)]
        C(ManualImplStrand),
    }

    #[test]
    fn strand_instance() {
        run_console::<ScopeStrand>(
            &mut EmptyState,
            "> ".into(),
            ". ".into(),
            "!".into(),
            Matcher::Single(MatcherSingle::Item(' ')),
            Matcher::List(&[MatcherSingle::Item('\n'), MatcherSingle::Item(';')]),
        );
    }
}
