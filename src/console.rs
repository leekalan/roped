use std::io::{self, Write};

use parsr::{
    matcher::MatchContainer,
    parse::{Parse, ParsePair},
};

use crate::Bundle;

pub fn run_console<'a, B, S>(
    state: &mut S,
    prompt: Option<&str>,
    counter_suffix: Option<&str>,
    err_prefix: Option<&str>,
    ws_chars: MatchContainer<&str, char>,
    nl_chars: MatchContainer<&str, char>,
) where
    B: Bundle<State = S, Input = &'a str>,
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

    parse_input::<B, S>(
        state,
        &input,
        counter_suffix,
        err_prefix,
        ws_chars,
        nl_chars,
    );
}

pub fn parse_input<'a, B, S>(
    state: &mut S,
    input: &'a str,
    counter_suffix: &str,
    err_prefix: &str,
    ws_chars: MatchContainer<&'a str, char>,
    nl_chars: MatchContainer<&str, char>,
) where
    B: Bundle<State = S, Input = &'a str>,
{
    parse_input_p::<B, S>(
        state,
        &input,
        counter_suffix,
        err_prefix,
        ws_chars,
        nl_chars,
        1,
    )
}

/// INPUT FIELD MUST BE TRIMMED AT THE START AND NOT EMPTY
fn parse_input_p<'a, B, S>(
    state: &mut S,
    input: &'a str,
    counter_suffix: &str,
    err_prefix: &str,
    ws_chars: MatchContainer<&'a str, char>,
    nl_chars: MatchContainer<&str, char>,
    index: u8,
) where
    B: Bundle<State = S, Input = &'a str>,
{
    let ParsePair { parsed, excess } = input.parse_one_arg(nl_chars);

    if index != 1 || excess.is_some() {
        print!("{}{}", index, counter_suffix);
    }

    let result = B::run(state, input, ws_chars);

    // let (residue, input_ws) = split_at_char(input, nl_chars);

    // let input = trim_chars(trim_chars(input_ws, &['\r', '\n']), ws_chars);

    // if input.is_empty() {
    //     if residue.is_empty() {
    //         return;
    //     } else {
    //         parse_input_p::<B, S>(
    //             state,
    //             residue,
    //             counter_suffix,
    //             err_prefix,
    //             ws_chars,
    //             nl_chars,
    //             index,
    //         );
    //         return;
    //     }
    // }

    // if index != 1 || !residue.is_empty() {
    //     print!("{}{}", index, counter_suffix);
    // }

    // let result = B::run(state, input, ws_chars);

    // if let Err(err) = result {
    //     println!("{}{}", err_prefix, err);
    // }

    // if !residue.is_empty() {
    //     parse_input_p::<B, S>(
    //         state,
    //         residue,
    //         counter_suffix,
    //         err_prefix,
    //         ws_chars,
    //         nl_chars,
    //         index + 1,
    //     );
    // }
}
