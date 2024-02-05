use std::io::{self, Write};

use crate::Bundle;

pub fn run_console<B, S>(
    state: &mut S,
    prompt: Option<&str>,
    counter_suffix: Option<&str>,
    err_prefix: Option<&str>,
    ws_chars: &[char],
    nl_chars: &[char],
) where
    B: Bundle<State = S>,
{
    if let Some(prompt) = prompt {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
    }

    let err_prefix = match err_prefix {
        Some(v) => v,
        None => "!",
    };

    let mut input = String::new();

    if io::stdin().read_line(&mut input).is_err() {
        println!("{}failed to read console!", err_prefix);
        return;
    }

    let counter_suffix = match counter_suffix {
        Some(v) => v,
        None => " ",
    };

    parse_input::<B, S>(state, &input, counter_suffix, err_prefix, ws_chars, nl_chars);
}

pub fn parse_input<B, S>(
    state: &mut S,
    input: &str,
    counter_suffix: &str,
    err_prefix: &str,
    ws_chars: &[char],
    nl_chars: &[char],
) where
    B: Bundle<State = S>,
{
    parse_input_p::<B, S>(state, input, counter_suffix, err_prefix, ws_chars, nl_chars, 1)
}

fn parse_input_p<B, S>(
    state: &mut S,
    input: &str,
    counter_suffix: &str,
    err_prefix: &str,
    ws_chars: &[char],
    nl_chars: &[char],
    index: u8,
) where
    B: Bundle<State = S>,
{
    let (residue, input_ws) = split_at_char(input, nl_chars);

    let input = trim_chars(trim_chars(input_ws, &['\r', '\n']), ws_chars);

    if input.is_empty() {
        if residue.is_empty() {
            return;
        } else {
            parse_input_p::<B, S>(state, residue, counter_suffix, err_prefix, ws_chars, nl_chars, index);
            return;
        }
    }

    if index != 1 || !residue.is_empty() {
        print!("{}{}", index, counter_suffix);
    }

    let result = B::run(state, input, ws_chars);

    if let Err(err) = result {
        println!("{}{}", err_prefix, err);
    }

    if !residue.is_empty() {
        parse_input_p::<B, S>(
            state,
            residue,
            counter_suffix,
            err_prefix,
            ws_chars,
            nl_chars,
            index + 1,
        );
    }
}

fn split_at_char<'a>(input_raw: &'a str, splits: &[char]) -> (&'a str, &'a str) {
    let input = trim_chars(input_raw, splits);

    let mut out = ("", input);

    for (index, char) in input.char_indices() {
        if splits.contains(&char) {
            let (a, b) = input.split_at(index);
            out = (b, trim_chars(a, splits));
            break;
        }
    }

    out
}

fn trim_chars<'a>(input: &'a str, splits: &[char]) -> &'a str {
    let start_trimmed = input.trim_start_matches(|c| splits.contains(&c));
    let trimmed = start_trimmed.trim_end_matches(|c| splits.contains(&c));
    trimmed
}
