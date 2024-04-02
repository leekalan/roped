pub mod base_types;
pub mod command;
pub mod console;
pub mod error;
pub mod strand;

#[allow(unused)]
use base_types::EmptyState;
#[allow(unused)]
use base_types::Trigger;
#[allow(unused)]
use parsr::parser_matcher::Matcher;
#[allow(unused)]
use strand::Strand;
#[allow(unused)]
use strand_derive::Strand;

#[allow(unused)]
use error::Error;

#[allow(clippy::single_component_path_imports)]
use parsr;

extern crate self as roped;

#[cfg(test)]
mod tests {
    use self::command::Command;

    use super::*;

    use base_types::EmptyState;
    use console::run_console;
    use parsr::{
        parser::safe_str::SafeStr,
        parser_matcher::{Matcher, MatcherSingle},
    };
    use strand::Strand;

    use strand_derive::Strand;

    #[allow(unused)]
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
        )
        .unwrap();
    }

    #[derive(Strand)]
    struct TrailStrand {
        num: usize,
        #[strand(trail)]
        trail: String,
    }

    impl Command for TrailStrand {
        type State = EmptyState;

        type Err = String;

        fn action(self, _state: &mut Self::State) -> Result<(), Self::Err> {
            let matcher: Matcher<str, char> = Matcher::Single(MatcherSingle::Item(' '));

            let trail = SafeStr::new(&self.trail, &matcher);

            print!("number: {}", self.num);
            if let Some(trail) = trail {
                let pair = trail.safe_parse_once();
                print!(" + trail args: {}", pair.arg.as_str());

                if let Some(trail) = pair.trail {
                    for arg in trail.safe_parse_all() {
                        print!(", {}", arg);
                    }
                }
            }

            println!();

            Ok(())
        }
    }

    #[derive(Strand)]
    struct FlagStrand {
        num: usize,
        #[strand(flag = "f1")]
        f1: Option<Trigger>,
        #[strand(flag = "f2")]
        f2: Option<usize>,
    }

    impl Command for FlagStrand {
        type State = EmptyState;

        type Err = String;

        fn action(self, _state: &mut Self::State) -> Result<(), Self::Err> {
            todo!()
        }
    }

    #[derive(Strand)]
    struct DefaultStrand {
        num: usize,
        #[strand(default = "abc".into())]
        d1: String,
        #[strand(default = 2)]
        d2: usize,
    }

    impl Command for DefaultStrand {
        type State = EmptyState;

        type Err = String;

        fn action(self, _state: &mut Self::State) -> Result<(), Self::Err> {
            println!("{}, {}, {}", self.num, self.d1, self.d2);

            Ok(())
        }
    }

    #[allow(unused)]
    #[derive(Strand)]
    #[strand(state = EmptyState, error = String)]
    enum ScopeStrand {
        #[strand(prefix = "$")]
        A(DefaultStrand),
        #[strand(name = "flag")]
        B(FlagStrand),
        #[strand(other)]
        C(TrailStrand),
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
        )
        .unwrap();
    }
}
