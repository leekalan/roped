use std::io;

use crate::Bundle;

pub fn run_console<B, S>(prompt: Option<&str>, state: &mut S)
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

    let parsed_input = parse(&input);

    let result = B::run(state, parsed_input);

    if let Err(err) = result {
        println!("Err: {}", err);
    }
}

pub fn parse(input: &str) -> impl Iterator<Item = &str> {
    let trimmed_input = input.trim();
    trimmed_input.split_whitespace()
}
