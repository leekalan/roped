pub mod base_types;
pub mod bundle;
pub mod parse;
pub mod strand;

pub use bundle::Bundle;
pub use strand::Strand;

pub use base_types::EmptyState;

pub use bundle_derive::Bundle;
pub use strand_derive::Strand;

pub use parse::{parse, run_console};

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    #[derive(Debug, Bundle)]
    #[bundle(state = "EmptyState")]
    enum BundleInstance {
        #[bundle(name = "scope")]
        StrandInstance(StrandInstance),
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
    fn parse_string() {
        let parsed_string = parse("scope 21 bob true");

        BundleInstance::run(&mut EmptyState, parsed_string).unwrap();
    }
}
