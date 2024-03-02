use std::io::{self, Write};

use parsr::{
    matcher::MatchContainer,
    parse::{Parse, ParsePair},
};

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

    let mut input = String::new();

    if io::stdin().read_line(&mut input).is_err() {
        println!("{}failed to read console!", err_prefix);
        return;
    }

    let counter_suffix = counter_suffix.unwrap_or(" ");

    let mut iter = input.parse_all_args(nl_chars);
    
    for command in iter {
        todo!()
    }
}