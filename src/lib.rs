pub mod base_types;
pub mod console;
pub mod error;
pub mod strand;

#[cfg(test)]
mod tests {
    use super::*;

    use parsr::{
        matcher::{MatchContainer, MatcherStart},
        parse::{Parse, ParsePair},
    };

    use base_types::EmptyState;
    use console::run_console;
    use error::RopedError;
    use strand::Strand;

    use strand_derive::Strand;

    struct ManualImplStrand;
    impl<'a> Strand<'a> for ManualImplStrand {
        type State = EmptyState;
        type Input = &'a str;
        type Err = String;

        fn run(
            state: &mut Self::State,
            input: Self::Input,
            ws_chars: MatchContainer<Self::Input, <Self::Input as MatcherStart>::Item>,
        ) -> Result<(), RopedError<Self::Err>> {
            let ParsePair { parsed, excess } = input.parse_one_arg(ws_chars);
            match excess {
                Some(v) => println!("{} + {}", parsed, v),
                None => println!("{}", parsed),
            }
            Ok(())
        }
    }

    #[test]
    fn manual_bundle_instance() {
        run_console::<ManualImplStrand, EmptyState>(
            &mut EmptyState,
            None,
            None,
            None,
            MatchContainer::ItemList(&[' ']),
            MatchContainer::ItemList(&['\n', ';']),
        );
    }

    // #[allow(dead_code)]
    // #[derive(Debug, Bundle)]
    // #[bundle(state = "EmptyState")]
    // enum BundleInstance {
    //     #[bundle(prefix = ":")]
    //     Quit(Quit),
    //     #[bundle(name = "scope")]
    //     StrandInstance(StrandInstance),
    //     #[bundle(other)]
    //     Other(OtherInstance),
    // }

    // #[allow(dead_code)]
    // #[derive(Debug, Strand)]
    // #[strand(state = "EmptyState", action = "action")]
    // struct StrandInstance {
    //     a: i32,
    //     b: String,
    //     #[strand(flag = "flag")]
    //     c: bool,
    // }

    // impl StrandInstance {
    //     fn action(&self, _: &mut EmptyState) -> Result<(), String> {
    //         println!("{:?}", self);
    //         Ok(())
    //     }
    // }

    // #[allow(dead_code)]
    // #[derive(Debug)]
    // struct OtherInstance;

    // impl Strand for OtherInstance {
    //     type State = EmptyState;

    //     fn run(_: &mut Self::State, input: &str, _: &[char]) -> Result<(), String> {
    //         println!("You sent: {}", input);
    //         Ok(())
    //     }
    // }

    // #[allow(dead_code)]
    // #[derive(Debug)]
    // struct Quit;

    // impl Strand for Quit {
    //     type State = EmptyState;

    //     fn run(_: &mut Self::State, _: &str, _: &[char]) -> Result<(), String> {
    //         std::process::exit(0);
    //     }
    // }

    // #[test]
    // fn strand_instance() {
    //     StrandInstance::run(&mut EmptyState, "21 --flag word", &[' ']).unwrap();
    // }

    // #[test]
    // fn bundle_instance() {
    //     BundleInstance::run(&mut EmptyState, "scope 21 word", &[' ']).unwrap();
    // }

    // #[test]
    // fn bundle_instance_other() {
    //     BundleInstance::run(&mut EmptyState, "seperated by spaces", &[' ']).unwrap();
    // }

    // #[test]
    // fn parse_multiline() {
    //     parse_input::<BundleInstance, EmptyState>(
    //         &mut EmptyState,
    //         "scope 21 word --flag; seperated by spaces",
    //         ". ",
    //         "!",
    //         &[' '],
    //         &[';'],
    //     )
    // }

    // #[test]
    // fn parse_empty() {
    //     parse_input::<BundleInstance, EmptyState>(&mut EmptyState, "", ". ", "!", &[' '], &[';'])
    // }

    // #[test]
    // fn parse_semi_colons() {
    //     parse_input::<BundleInstance, EmptyState>(
    //         &mut EmptyState,
    //         ";;  ; ;; ; ",
    //         ". ",
    //         "!",
    //         &[' '],
    //         &[';'],
    //     )
    // }

    // #[test]
    // fn console() {
    //     loop {
    //         run_console::<BundleInstance, EmptyState>(
    //             &mut EmptyState,
    //             "> ".into(),
    //             ". ".into(),
    //             None,
    //             &[' '],
    //             &[';'],
    //         )
    //     }
    // }
}
