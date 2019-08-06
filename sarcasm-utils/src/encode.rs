use crate::StartingCase;
use itertools::Itertools;

#[inline]
fn char_to_lower(c: char) -> impl Iterator<Item = char> {
    c.to_lowercase()
}

#[inline]
fn char_to_upper(c: char) -> impl Iterator<Item = char> {
    c.to_uppercase()
}

#[inline]
fn encode_lower(input: &str) -> String {
    let first = input.chars().step_by(2).flat_map(char_to_lower);
    let second = input.chars().skip(1).step_by(2).flat_map(char_to_upper);

    first.interleave(second).collect()
}

#[inline]
fn encode_upper(input: &str) -> String {
    let first = input.chars().step_by(2).flat_map(char_to_upper);
    let second = input.chars().skip(1).step_by(2).flat_map(char_to_lower);

    first.interleave(second).collect()
}

pub fn encode_sarcasm(input: &str, start: StartingCase) -> String {
    match start {
        StartingCase::Lowercase => encode_lower(input),
        StartingCase::Uppercase => encode_upper(input),
    }
}
