pub mod base_types;
pub mod bundle;
pub mod strand;
pub mod branch;
pub mod parse;

pub use bundle::Bundle;
pub use strand::Strand;
pub use branch::Branch;

pub use base_types::EmptyState;

pub use bundle_derive::Bundle;
pub use strand_derive::Strand;

pub use parse::{parse_whitespace, run_console};

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    #[derive(Debug, Bundle)]
    #[bundle(state = "EmptyState")]
    enum BundleInstance {
        #[bundle(name = "scope")]
        StrandInstance(StrandInstance),
        #[bundle(other)]
        Other(BranchInstance),
    }

    #[allow(dead_code)]
    #[derive(Debug, Strand)]
    #[strand(state = "EmptyState", action = "action")]
    struct StrandInstance {
        a: i32,
        b: String,
        c: bool,
    }

    impl StrandInstance {
        fn action(&self, _: &mut EmptyState) -> Result<(), String> {
            println!("{:?}", self);
            Ok(())
        }
    }

    #[derive(Debug)]
    struct BranchInstance;

    impl Branch for BranchInstance {
        type State = EmptyState;

        fn run<'a>(_: &mut Self::State, arg: &'a str, _: impl Iterator<Item = &'a str>) -> Result<(), String> {
            println!("You said {}", arg);
            Ok(())
        }
    }

    #[test]
    fn strand_instance() {
        StrandInstance::run(&mut EmptyState, vec!["21", "bob", "true"].into_iter()).unwrap();
    }

    #[test]
    fn bundle_instance() {
        BundleInstance::run(
            &mut EmptyState,
            vec!["scope", "21", "bob", "true"].into_iter(),
        )
        .unwrap();
    }

    #[test]
    fn bundle_instance_other() {
        BundleInstance::run(
            &mut EmptyState,
            vec!["test 1 2 3"].into_iter(),
        )
        .unwrap();
    }

    #[test]
    fn parse_string() {
        let parsed_string = parse_whitespace("scope 21 bob true");

        BundleInstance::run(&mut EmptyState, parsed_string).unwrap();
    }
}
