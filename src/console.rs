use std::{
    borrow::Borrow,
    io::{self, Write},
};

use parsr::{
    parser::{safe_str::SafeStr, Parser},
    parser_matcher::Matcher,
};

use crate::strand::Strand;

/// Runs a console using the provided Strand
///
/// Takes in a mutable reference to a State and various configuration options
///
/// `prompt`, `counter_suffix`, and `error_prefix` are all optional and will be used if provided,
/// otherwise they will be left as default values
///
/// `ws_chars` and `nl_chars` are used to determine the argument separators and newline separators
/// for parsing the input into individual commands
pub fn run_console<'a, R: Strand<Err = String>>(
    state: &mut R::State,
    prompt: Option<&str>,
    counter_suffix: Option<&str>,
    err_prefix: Option<&str>,
    ws_chars: impl Borrow<Matcher<'a, str, char>>,
    nl_chars: impl Borrow<Matcher<'a, str, char>>,
) -> Result<(), io::Error> {
    // Prints the prompt if one was provided
    if let Some(prompt) = prompt {
        print!("{}", prompt);
        io::stdout().flush()?;
    }

    // Reads the input from stdin until a newline is encountered
    let mut read_input = String::new();
    io::stdin().read_line(&mut read_input)?;

    // Sets the `counter_suffix` and `error_prefix` to a default value if one wasn't provided
    let counter_suffix = counter_suffix.unwrap_or(" ");
    let err_prefix = err_prefix.unwrap_or("!");

    // Trims the trailing whitespace from the input
    let input: &str = read_input.trim_end_matches(['\n', '\r']);

    // Creates an iterator over the input, separated by newline characters
    let mut iter = input.parse_all(nl_chars.borrow());

    // Index to keep track of which command is being run
    let mut index = 1usize;

    // Loops over each command in the input
    while let Some(command) = iter.next() {
        // Trims whitespace from the command
        let command = SafeStr::new(command, ws_chars.borrow());

        if command.is_none() {
            continue;
        }

        // Prints the index if it's not the first command or there are more commands
        if iter.get_internal().is_some() || index != 1 {
            print!("{}{}", index, counter_suffix);
            index += 1;
        }

        // Runs the command and prints the error if it fails
        if let Err(err) = R::run(state, command, 1) {
            println!("{}{}", err_prefix, err);
        }
    }

    Ok(())
}
