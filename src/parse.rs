use std::io;

use crate::Bundle;

pub fn run_console<B, S>(prompt: Option<&str>, nl_chars: &str, state: &mut S)
where
    B: Bundle<State = S>,
{
    if let Some(prompt) = prompt {
        println!("{}", prompt);
    }

    let mut input = String::new();

    if io::stdin().read_line(&mut input).is_err() {
        println!("Err: failed to read console!");
        return;
    }

    sub_console::<B, S>(&input, nl_chars, state, 1);
}

fn sub_console<B, S>(input: &str, nl_chars: &str, state: &mut S, index: u8)
where
    B: Bundle<State = S>,
{
    let (residue_w, input) = parse_nl(input, nl_chars);

    let parsed_input = parse_whitespace(input);

    if residue_w.is_some() {
        print!("{}. ", index);
    }

    let result = B::run(state, parsed_input);

    if let Err(err) = result {
        println!("Err: {}", err);
    }

    if let Some(residue) = residue_w {
        sub_console::<B, S>(residue, nl_chars, state, index + 1);
    }
}

pub fn parse_nl<'a>(input: &'a str, nl_chars: &str) -> (Option<&'a str>, &'a str) {
    for (index, char) in input.char_indices() {
        if nl_chars.find(char).is_some() {
            let (a, b) = input.split_at(index);
            return (Some(b), a)
        }
    }
    
    (None, input)
}

pub fn parse_whitespace(input: &str) -> impl Iterator<Item = &str> {
    let trimmed_input = input.trim();
    trimmed_input.split_whitespace()
}
