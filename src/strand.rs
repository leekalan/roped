use parsr::parser::trimmed::Trimmed;

use crate::error::Error;

/// A Strand is an object that acts on an input, primarily in a console setting
///
/// # Overview
///
/// A Strand is responsible for taking input from a user and executing some
/// action. In a console application, you will have multiple Strands, each
/// responsible for a different "scope" or "command"
///
/// # Run
///
/// A Strand is given a mutable reference to the current state, a reference to an input string to
/// parse, a reference to a matcher of whitespace characters, and an index of the current scope
///
/// ```rust
/// fn run(
///     state: &mut Self::State,
///     input: Option<Trimmed<str>>,
///     index: usize,
/// ) -> Result<(), Error<Self::Err>>;
/// ```
///
/// # Examples
///
/// ```
/// use parsr::parser_matcher::Matcher;
///
/// use roped::{Strand, Error, EmptyState, Matcher};
///
/// struct StrandExample;
///
/// impl Strand for StrandExample {
///     type State = EmptyState;
///     type Err = ();
///
///     fn run(
///         _state: &mut Self::State,
///         input: Option<Trimmed<str>>,
///         _index: usize,
///     ) -> Result<(), Error<Self::Err>> {
///         for command in input.parse_all(ws) {
///             println!("{},", command.get_arg());
///         }
///         Ok(())
///     }
/// }
/// ```
///
/// # Purpose
///
/// The value of using a Strand is that it intergrates seamlessly with other
/// Strand variants construct using the [`#[derive(Strand)]`](trait@Strand)
/// macro.
///
pub trait Strand {
    /// The type of the current state.
    type State: ?Sized;
    /// The type of error the Strand can return.
    type Err;

    /// The function that will be called by the console application to execute the Strand.
    ///
    /// A Strand is given a mutable reference to the current state, a reference to an input string to
    /// parse, a reference to a matcher of whitespace characters, and an index of the current scope.
    ///
    /// ```
    /// fn run(
    ///     state: &mut Self::State,
    ///     input: Option<Trimmed<str>>,
    ///     index: usize,
    /// ) -> Result<(), Error<Self::Err>>;
    /// ```
    fn run(
        state: &mut Self::State,
        input: Option<Trimmed<str>>,
        index: usize,
    ) -> Result<(), Error<Self::Err>>;
}
