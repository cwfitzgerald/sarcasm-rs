use crate::StartingCase;
use itertools::Itertools;

#[inline]
fn encode_lower(input: &str) -> String {
    let first = input.chars().step_by(2).flat_map(|c| c.to_lowercase());
    let second = input.chars().skip(1).step_by(2).flat_map(|c| c.to_uppercase());

    first.interleave(second).collect()
}

#[inline]
fn encode_upper(input: &str) -> String {
    let first = input.chars().step_by(2).flat_map(|c| c.to_uppercase());
    let second = input.chars().skip(1).step_by(2).flat_map(|c| c.to_lowercase());

    first.interleave(second).collect()
}

pub fn encode_sarcasm(input: &str, start: StartingCase) -> String {
    match start {
        StartingCase::Lowercase => encode_lower(input),
        StartingCase::Uppercase => encode_upper(input),
    }
}
