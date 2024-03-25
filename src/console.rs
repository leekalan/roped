use std::{
    borrow::Borrow,
    io::{self, Write},
};

use parsr::{
    parser::{trim::Trim, Parser},
    parser_matcher::Matcher,
};

use crate::strand::Strand;

pub fn run_console<'a, R: Strand<Input = str, Err = String>>(
    state: &mut R::State,
    prompt: Option<&str>,
    counter_suffix: Option<&str>,
    err_prefix: Option<&str>,
    ws_chars: impl Borrow<Matcher<'a, str, char>>,
    nl_chars: impl Borrow<Matcher<'a, str, char>>,
) -> Result<(), io::Error> {
    if let Some(prompt) = prompt {
        print!("{}", prompt);
        io::stdout().flush()?;
    }

    let mut read_input = String::new();

    io::stdin().read_line(&mut read_input)?;

    let err_prefix = err_prefix.unwrap_or("!");

    let counter_suffix = counter_suffix.unwrap_or(" ");

    let input: &str = read_input.trim_end_matches(['\n', '\r']);

    let mut iter = input.parse_all(nl_chars.borrow());

    let mut index = 1usize;
    while let Some(command) = iter.next() {
        let command = command.trim_all(ws_chars.borrow());

        if command.is_empty() {
            continue;
        }

        if iter.get_internal().is_some() || index != 1 {
            print!("{}{}", index, counter_suffix);
            index += 1;
        }

        if let Err(err) = R::run(state, command, ws_chars.borrow(), 1) {
            println!("{}{}", err_prefix, err);
        }
    }

    Ok(())
}
