pub mod base_types;
pub mod console;
pub mod error;
pub mod strand;

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use super::*;

    use base_types::EmptyState;
    use console::run_console;
    use parsr::{parser::{search::Search, ParsePair, Parser}, parser_matcher::Matcher};
    use strand::Strand;

    use strand_derive::Strand;

    use crate as roped;

    struct ManualImplStrand;
    impl<'a> Strand<'a> for ManualImplStrand {
        type State = EmptyState;
        type Input = str;
        type Err = String;
        
        fn run(
            state: &mut Self::State,
            input: &Self::Input,
            ws: &'a impl Borrow<Matcher<'a, Self::Input, <Self::Input as Search>::Item>>,
            index: usize,
        ) -> Result<(), error::Error<&'a Self::Input, Self::Err>> {
            if let Some(pair) = input.parse_once(ws.borrow()) {
                match pair.get_trail() {
                    Some(v) => println!("{} + {}", pair.get_arg(), v),
                    None => println!("{}", pair.get_arg()),
                }
    
                Ok(())
            } else {
                todo!()
            }
        }
    }

    #[test]
    fn manual_bundle_instance() {
        run_console::<ManualImplStrand, EmptyState>(
            &mut EmptyState,
            "> ".into(),
            ". ".into(),
            "!".into(),
            MatchContainer::ItemList(&[' ']),
            MatchContainer::ItemList(&['\n', ';']),
        );
    }

    #[derive(Strand)]
    #[strand()]
    enum ImplStrand {
        #[strand()]
        A,
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
