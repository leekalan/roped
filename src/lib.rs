pub mod base_types;
pub mod bundle;
pub mod parse;
pub mod strand;

pub use bundle::Bundle;
pub use strand::Strand;

pub use base_types::EmptyState;

pub use bundle_derive::Bundle;
pub use strand_derive::Strand;

pub use parse::{parse_input, run_console};

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    #[derive(Debug, Bundle)]
    #[bundle(state = "EmptyState")]
    enum BundleInstance {
        #[bundle(prefix = ":")]
        Quit(Quit),
        #[bundle(name = "scope")]
        StrandInstance(StrandInstance),
        #[bundle(other)]
        Other(OtherInstance),
    }

    #[allow(dead_code)]
    #[derive(Debug, Strand)]
    #[strand(state = "EmptyState", action = "action")]
    struct StrandInstance {
        a: i32,
        b: String,
        #[strand(flag = "flag")]
        c: bool,
    }

    impl StrandInstance {
        fn action(&self, _: &mut EmptyState) -> Result<(), String> {
            println!("{:?}", self);
            Ok(())
        }
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    struct OtherInstance;

    impl Strand for OtherInstance {
        type State = EmptyState;

        fn run(_: &mut Self::State, input: &str, _: &[char]) -> Result<(), String> {
            println!("You sent: {}", input);
            Ok(())
        }
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Quit;

    impl Strand for Quit {
        type State = EmptyState;

        fn run(_: &mut Self::State, _: &str, _: &[char]) -> Result<(), String> {
            std::process::exit(0);
        }
    }

    #[test]
    fn strand_instance() {
        StrandInstance::run(&mut EmptyState, "21 --flag word", &[' ']).unwrap();
    }

    #[test]
    fn bundle_instance() {
        BundleInstance::run(&mut EmptyState, "scope 21 word", &[' ']).unwrap();
    }

    #[test]
    fn bundle_instance_other() {
        BundleInstance::run(&mut EmptyState, "seperated by spaces", &[' ']).unwrap();
    }

    #[test]
    fn parse_multiline() {
        parse_input::<BundleInstance, EmptyState>(
            &mut EmptyState,
            "scope 21 word --flag; seperated by spaces",
            ". ",
            "!",
            &[' '],
            &[';'],
        )
    }

    #[test]
    fn parse_empty() {
        parse_input::<BundleInstance, EmptyState>(&mut EmptyState, "", ". ", "!", &[' '], &[';'])
    }

    #[test]
    fn parse_semi_colons() {
        parse_input::<BundleInstance, EmptyState>(
            &mut EmptyState,
            ";;  ; ;; ; ",
            ". ",
            "!",
            &[' '],
            &[';'],
        )
    }

    #[test]
    fn console() {
        run_console::<BundleInstance, EmptyState>(
            &mut EmptyState,
            "> ".into(),
            ". ".into(),
            None,
            &[' '],
            &[';'],
        )
    }
}
