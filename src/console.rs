use std::io::{self, Write};

use parsr::{matcher::MatchContainer, parse::Parse, trim::Trim};

use crate::{error::RopedError, strand::Strand};

pub fn run_console<R, S>(
    state: &mut S,
    prompt: Option<&str>,
    counter_suffix: Option<&str>,
    err_prefix: Option<&str>,
    ws_chars: MatchContainer<&str, char>,
    nl_chars: MatchContainer<&str, char>,
) where
    R: for<'a> Strand<'a, State = S, Input = &'a str, Err = String>,
{
    if let Some(prompt) = prompt {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
    }

    let err_prefix = err_prefix.unwrap_or("!");

    let mut read_input = String::new();

    if io::stdin().read_line(&mut read_input).is_err() {
        println!("{}failed to read console!", err_prefix);
        return;
    }

    let input: &str =
        match Trim::trim_end(read_input.as_ref(), MatchContainer::ItemList(&['\r', '\n'])) {
            Some(v) => v,
            None => return,
        };

    let counter_suffix = counter_suffix.unwrap_or(" ");

    let mut iter = input.parse_all_args(nl_chars);

    let mut index = 1usize;
    while let Some(command) = iter.next().map(|s| Trim::trim_start(s, ws_chars)) {
        let command = match command {
            Some(v) => v,
            None => continue,
        };

        if !iter.is_empty() || index != 1 {
            print!("{}{}", index, counter_suffix);
            index += 1;
        }

        if let Err(err) = R::run(state, command, ws_chars, 1) {
            match err {
                RopedError::Internal(_err) => (),
                RopedError::Err(err) => println!("{}{}", err_prefix, err),
            }
        }
    }
}
