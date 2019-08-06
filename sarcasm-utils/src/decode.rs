use crate::StartingCase;
use itertools::Itertools;
use log::{debug, info, trace};
use std::convert::identity;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IsSarcasm {
    Yes(StartingCase),
    No,
    TooShort,
}

fn different_case((l, r): (char, char)) -> bool {
    if l.is_lowercase() {
        r.is_uppercase()
    } else {
        r.is_lowercase()
    }
}

pub fn is_sarcasm(input: &str) -> IsSarcasm {
    debug!("Checking sarcasm on {} bytes", input.len());
    trace!("Checking sarcasm on input: {}", input);

    let mut iter = input.chars().skip_while(|c| !c.is_alphabetic()).peekable();
    match iter.peek() {
        Some(&c) => {
            let grouped = iter.group_by(|c| c.is_alphabetic());
            let grouped_filtered = grouped.into_iter().filter(|(key, group)| *key == true).map(|(_, g)| g);
            let sarcasm: bool = grouped_filtered
                .map(|g| g.tuple_windows().map(different_case).all(identity))
                .all(identity);

            if sarcasm {
                IsSarcasm::Yes(StartingCase::from(c))
            } else {
                IsSarcasm::No
            }
        }
        None => IsSarcasm::TooShort,
    }
}
