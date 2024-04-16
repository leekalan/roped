pub mod base_types;
pub mod command;
pub mod console;
pub mod error;
pub mod strand;

#[allow(unused)]
pub use base_types::EmptyState;
#[allow(unused)]
pub use base_types::Trigger;
#[allow(unused)]
pub use parsr::parser_matcher::Matcher;
#[allow(unused)]
pub use strand::Strand;
#[allow(unused)]
pub use strand_derive::Strand;

#[allow(unused)]
pub use error::Error;

#[allow(clippy::single_component_path_imports)]
pub use parsr;

pub extern crate self as roped;

#[cfg(test)]
mod tests {
    use self::command::Command;

    use super::*;

    use base_types::EmptyState;
    use console::run_console;
    use parsr::{
        parser::trimmed::Trimmed,
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
            input: Option<Trimmed<str>>,
            _index: usize,
        ) -> Result<(), error::Error<Self::Err>> {
            let input = match input {
                Some(v) => v,
                None => return Err(error::Error::Err("Recieved no input".to_string())),
            };

            let pair = input.parse_once();

            match pair.trail {
                Some(v) => println!("{} + {}", pair.arg.get_internal(), v.get_internal()),
                None => println!("{}", pair.arg.get_internal()),
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

            let trail = Trimmed::<str>::new(&self.trail, &matcher);

            print!("number: {}", self.num);
            if let Some(trail) = trail {
                let pair = trail.parse_once();
                print!(" + trail args: {}", pair.arg.get_internal());

                if let Some(trail) = pair.trail {
                    for arg in trail.parse_all() {
                        print!(", {}", arg.get_internal());
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
        f2: Option<String>,
    }

    impl Command for FlagStrand {
        type State = EmptyState;

        type Err = String;

        fn action(self, _state: &mut Self::State) -> Result<(), Self::Err> {
            println!("num: {}, f1: {:?}, f2: {:?}", self.num, self.f1, self.f2);

            Ok(())
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
